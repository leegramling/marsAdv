use axum::{
    extract::Json as ExtractJson,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

mod scenes;

use scenes::{GameState, SceneDocument, SceneId};

struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState {
                scene: SceneId::ControlRoom,
                has_wrench: false,
                pump_damaged: true,
                turn: 0,
            },
        }
    }

    fn response(&self, message: impl Into<String>) -> GameResponse {
        let scene = self.state.scene.scene();
        GameResponse {
            message: message.into(),
            location: scene.name().into(),
            prompt: scene.prompt().into(),
            inventory: if self.state.has_wrench {
                vec!["Wrench".into()]
            } else {
                vec![]
            },
            turn: self.state.turn,
            available_exits: scene.exits(),
        }
    }

    fn execute(&mut self, input: &str) -> GameResponse {
        let command = input.trim().to_lowercase();
        let message = if command.is_empty() {
            "Enter a command. Try `help`.".into()
        } else if command == "help" {
            "Commands: look, take wrench, north, south, east, west, inspect pump, inspect robots, repair pump, inventory".into()
        } else if command == "inventory" || command == "i" {
            if self.state.has_wrench {
                "You are carrying a wrench.".into()
            } else {
                "Your inventory is empty.".into()
            }
        } else {
            let scene = self.state.scene.scene();
            let result = scene.execute(&command, &mut self.state);
            if let Some(next_scene) = result.next_scene {
                self.state.scene = next_scene;
            }
            if result.advances_turn {
                self.state.turn += 1;
            }
            result.message
        };
        self.response(message)
    }
}

#[derive(Serialize)]
struct GameResponse {
    message: String,
    location: String,
    prompt: String,
    inventory: Vec<String>,
    turn: u32,
    available_exits: Vec<String>,
}

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

type SharedGame = Arc<Mutex<Game>>;

#[derive(Serialize)]
struct SceneValidation {
    valid: bool,
    errors: Vec<String>,
}

fn validate_scene(scene: &SceneDocument) -> SceneValidation {
    let mut errors = Vec::new();
    if scene.id.trim().is_empty() {
        errors.push("Scene id cannot be empty.".into());
    }
    if scene.name.trim().is_empty() {
        errors.push("Scene name cannot be empty.".into());
    }
    let mut ids = std::collections::HashSet::new();
    for tile in &scene.tiles {
        if !ids.insert(format!("tile:{}", tile.id)) {
            errors.push(format!("Duplicate tile id: {}", tile.id));
        }
        if tile.grid.column < 0 || tile.grid.row < 0 {
            errors.push(format!("Tile {} has a negative grid position.", tile.id));
        }
    }
    let mut item_ids = std::collections::HashSet::new();
    for item in &scene.items {
        if !item_ids.insert(&item.id) {
            errors.push(format!("Duplicate item id: {}", item.id));
        }
        if item.take.commands.is_empty() {
            errors.push(format!("Item {} has no take command.", item.id));
        }
    }
    let mut command_ids = std::collections::HashSet::new();
    for command in &scene.commands {
        if !command_ids.insert(&command.id) {
            errors.push(format!("Duplicate command id: {}", command.id));
        }
        if command.aliases.is_empty() {
            errors.push(format!("Command {} has no aliases.", command.id));
        }
    }
    SceneValidation {
        valid: errors.is_empty(),
        errors,
    }
}

async fn validate_scene_endpoint(
    ExtractJson(scene): ExtractJson<SceneDocument>,
) -> ExtractJson<SceneValidation> {
    ExtractJson(validate_scene(&scene))
}

async fn new_game(State(game): State<SharedGame>) -> Json<GameResponse> {
    let mut game = game.lock().await;
    *game = Game::new();
    let message = game.state.scene.scene().look(&game.state);
    Json(game.response(message))
}
async fn get_state(State(game): State<SharedGame>) -> Json<GameResponse> {
    let game = game.lock().await;
    let message = game.state.scene.scene().look(&game.state);
    Json(game.response(message))
}

async fn command(
    State(game): State<SharedGame>,
    Json(request): Json<CommandRequest>,
) -> Result<Json<GameResponse>, (StatusCode, String)> {
    if request.command.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Command cannot be empty".into()));
    }
    Ok(Json(game.lock().await.execute(&request.command)))
}

#[tokio::main]
async fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    let app = Router::new()
        .route("/api/game/new", post(new_game))
        .route("/api/game/state", get(get_state))
        .route("/api/game/command", post(command))
        .route("/api/scenes/validate", post(validate_scene_endpoint))
        .layer(CorsLayer::permissive())
        .with_state(game);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("ARES SILO 7 server listening on http://127.0.0.1:5000");
    axum::serve(listener, app).await.unwrap();
}

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Clone, Copy, PartialEq)]
enum Location {
    ControlRoom,
    PumpRoom,
    RobotBay,
}

impl Location {
    fn name(self) -> &'static str {
        match self {
            Self::ControlRoom => "Control Room",
            Self::PumpRoom => "Water Pump Room",
            Self::RobotBay => "Robot Bay",
        }
    }
}

struct Game {
    location: Location,
    has_wrench: bool,
    pump_damaged: bool,
    turn: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            location: Location::ControlRoom,
            has_wrench: false,
            pump_damaged: true,
            turn: 0,
        }
    }

    fn response(&self, message: impl Into<String>) -> GameResponse {
        GameResponse {
            message: message.into(),
            location: self.location.name().into(),
            inventory: if self.has_wrench {
                vec!["Wrench".into()]
            } else {
                vec![]
            },
            turn: self.turn,
            available_exits: self.exits(),
        }
    }

    fn exits(&self) -> Vec<String> {
        match self.location {
            Location::ControlRoom => vec!["south".into()],
            Location::PumpRoom => vec!["north".into(), "east".into()],
            Location::RobotBay => vec!["west".into()],
        }
    }

    fn look(&self) -> String {
        match self.location {
            Location::ControlRoom if !self.has_wrench => "The control room is filled with unfinished consoles. A wrench lies beside a terminal.".into(),
            Location::ControlRoom => "The control room is filled with unfinished consoles. The wrench is gone.".into(),
            Location::PumpRoom if self.pump_damaged => "Red emergency lights pulse across the walls. Pump 3 is vibrating violently.".into(),
            Location::PumpRoom => "Pump 3 runs smoothly. The emergency lights have stopped pulsing.".into(),
            Location::RobotBay => "Three construction robots move between unfinished sections of the silo. Their movements are erratic, but not random.".into(),
        }
    }

    fn execute(&mut self, input: &str) -> GameResponse {
        let command = input.trim().to_lowercase();
        let words: Vec<&str> = command.split_whitespace().collect();
        let message = match words.as_slice() {
            [] => "Enter a command. Try `help`.".into(),
            ["look"] | ["l"] => self.look(),
            ["help"] => "Commands: look, take wrench, north, south, east, west, inspect pump, inspect robots, repair pump, inventory".into(),
            ["inventory"] | ["i"] => if self.has_wrench { "You are carrying a wrench." } else { "Your inventory is empty." }.into(),
            ["north"] | ["n"] | ["go", "north"] => self.move_to(Location::ControlRoom, "north"),
            ["south"] | ["s"] | ["go", "south"] => self.move_to(Location::PumpRoom, "south"),
            ["east"] | ["e"] | ["go", "east"] => self.move_to(Location::RobotBay, "east"),
            ["west"] | ["w"] | ["go", "west"] => self.move_to(Location::PumpRoom, "west"),
            ["take", "wrench"] | ["get", "wrench"] => {
                if self.location == Location::ControlRoom && !self.has_wrench { self.has_wrench = true; "You pick up the wrench.".into() }
                else if self.has_wrench { "You already have the wrench.".into() }
                else { "There is no wrench here.".into() }
            }
            ["inspect", "pump"] | ["examine", "pump"] => {
                if self.location != Location::PumpRoom { "There is no pump here.".into() }
                else if self.pump_damaged { "Pump 3 is vibrating violently. The outlet pressure coupling has separated. It could be tightened with a wrench.".into() }
                else { "Pump 3 is operating normally.".into() }
            }
            ["repair", "pump"] | ["fix", "pump"] => self.repair_pump(),
            ["inspect", "robots"] | ["examine", "robots"] => if self.location == Location::RobotBay { "The robots move in strange, deliberate patterns. They are not responding to construction commands.".into() } else { "There are no robots here.".into() },
            _ => "I do not understand that command.".into(),
        };
        self.response(message)
    }

    fn move_to(&mut self, destination: Location, direction: &str) -> String {
        let allowed = matches!(
            (self.location, destination),
            (Location::ControlRoom, Location::PumpRoom)
                | (Location::PumpRoom, Location::ControlRoom)
                | (Location::PumpRoom, Location::RobotBay)
                | (Location::RobotBay, Location::PumpRoom)
        );
        if allowed {
            self.location = destination;
            self.turn += 1;
            format!("You move {direction}. {}", self.look())
        } else {
            format!("You cannot move {direction} from here.")
        }
    }

    fn repair_pump(&mut self) -> String {
        if self.location != Location::PumpRoom {
            return "There is no pump here.".into();
        }
        if !self.pump_damaged {
            return "The pump is already repaired.".into();
        }
        if !self.has_wrench {
            return "You need a wrench to tighten the damaged coupling.".into();
        }
        self.pump_damaged = false;
        self.turn += 1;
        "You tighten the pressure coupling. Pump 3 returns to normal operation.".into()
    }
}

#[derive(Serialize)]
struct GameResponse {
    message: String,
    location: String,
    inventory: Vec<String>,
    turn: u32,
    available_exits: Vec<String>,
}

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

type SharedGame = Arc<Mutex<Game>>;

async fn new_game(State(game): State<SharedGame>) -> Json<GameResponse> {
    let mut game = game.lock().await;
    *game = Game::new();
    let message = game.look();
    Json(game.response(message))
}
async fn get_state(State(game): State<SharedGame>) -> Json<GameResponse> {
    let game = game.lock().await;
    let message = game.look();
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
        .layer(CorsLayer::permissive())
        .with_state(game);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("ARES SILO 7 server listening on http://127.0.0.1:5000");
    axum::serve(listener, app).await.unwrap();
}

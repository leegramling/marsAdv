mod control_room;
mod pump_room;
mod robot_bay;

pub use control_room::ControlRoom;
pub use pump_room::PumpRoom;
pub use robot_bay::RobotBay;

pub struct GameState {
    pub scene: SceneId,
    pub has_wrench: bool,
    pub pump_damaged: bool,
    pub turn: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneDocument {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub background: SceneBackground,
    pub tiles: Vec<SceneTile>,
    pub items: Vec<SceneItem>,
    pub commands: Vec<SceneCommand>,
    pub exits: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneBackground {
    pub color: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneTile {
    pub id: String,
    pub asset: String,
    pub grid: SceneGrid,
    pub layer: i32,
    pub rotation: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneGrid {
    #[serde(alias = "x")]
    pub column: i32,
    #[serde(alias = "y")]
    pub row: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub grid: SceneGrid,
    pub take: SceneTake,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneTake {
    pub commands: Vec<String>,
    pub message: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneCommand {
    pub id: String,
    pub aliases: Vec<String>,
    pub message: String,
    pub requires: Vec<SceneRequirement>,
    pub transition: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SceneRequirement {
    pub kind: String,
    pub value: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SceneId {
    ControlRoom,
    PumpRoom,
    RobotBay,
}

pub struct SceneResult {
    pub message: String,
    pub next_scene: Option<SceneId>,
    pub advances_turn: bool,
}

pub trait Scene: Sync {
    fn name(&self) -> &'static str;
    fn prompt(&self) -> &'static str;
    fn look(&self, state: &GameState) -> &'static str;
    fn exits(&self) -> Vec<String>;
    fn execute(&self, command: &str, state: &mut GameState) -> SceneResult;
}

static CONTROL_ROOM: ControlRoom = ControlRoom;
static PUMP_ROOM: PumpRoom = PumpRoom;
static ROBOT_BAY: RobotBay = RobotBay;

impl SceneId {
    pub fn scene(self) -> &'static dyn Scene {
        match self {
            Self::ControlRoom => &CONTROL_ROOM,
            Self::PumpRoom => &PUMP_ROOM,
            Self::RobotBay => &ROBOT_BAY,
        }
    }
}

pub fn result(message: impl Into<String>) -> SceneResult {
    SceneResult {
        message: message.into(),
        next_scene: None,
        advances_turn: false,
    }
}

pub fn move_result(scene: SceneId, message: impl Into<String>) -> SceneResult {
    SceneResult {
        message: message.into(),
        next_scene: Some(scene),
        advances_turn: true,
    }
}

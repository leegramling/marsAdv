use super::{move_result, result, GameState, Scene, SceneId, SceneResult};

pub struct RobotBay;

impl Scene for RobotBay {
    fn name(&self) -> &'static str {
        "Robot Bay"
    }

    fn prompt(&self) -> &'static str {
        "Construction robots move between unfinished sections of the silo."
    }

    fn look(&self, _state: &GameState) -> &'static str {
        "Three construction robots move between unfinished sections of the silo. Their movements are erratic, but not random."
    }

    fn exits(&self) -> Vec<String> {
        vec!["west".into()]
    }

    fn execute(&self, command: &str, state: &mut GameState) -> SceneResult {
        match command {
            "look" | "l" => result(self.look(state)),
            "west" | "w" | "go west" => {
                move_result(SceneId::PumpRoom, format!("You move west. {}", self.look(state)))
            }
            "inspect robots" | "examine robots" => result("The robots move in strange, deliberate patterns. They are not responding to construction commands."),
            _ => result("I do not understand that command."),
        }
    }
}

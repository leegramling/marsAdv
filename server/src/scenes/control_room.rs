use super::{move_result, result, GameState, Scene, SceneId, SceneResult};

pub struct ControlRoom;

impl Scene for ControlRoom {
    fn name(&self) -> &'static str {
        "Control Room"
    }

    fn prompt(&self) -> &'static str {
        "The unfinished consoles hum quietly."
    }

    fn look(&self, state: &GameState) -> &'static str {
        if state.has_wrench {
            "The control room is filled with unfinished consoles. The wrench is gone."
        } else {
            "The control room is filled with unfinished consoles. A wrench lies beside a terminal."
        }
    }

    fn exits(&self) -> Vec<String> {
        vec!["south".into()]
    }

    fn execute(&self, command: &str, state: &mut GameState) -> SceneResult {
        match command {
            "look" | "l" => result(self.look(state)),
            "south" | "s" | "go south" => move_result(
                SceneId::PumpRoom,
                format!("You move south. {}", self.look(state)),
            ),
            "take wrench" | "get wrench" => {
                if !state.has_wrench {
                    state.has_wrench = true;
                    result("You pick up the wrench.")
                } else {
                    result("You already have the wrench.")
                }
            }
            _ => result("I do not understand that command."),
        }
    }
}

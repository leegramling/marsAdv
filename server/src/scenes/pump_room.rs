use super::{move_result, result, GameState, Scene, SceneId, SceneResult};

pub struct PumpRoom;

impl Scene for PumpRoom {
    fn name(&self) -> &'static str {
        "Water Pump Room"
    }

    fn prompt(&self) -> &'static str {
        "Emergency lights pulse across the walls."
    }

    fn look(&self, state: &GameState) -> &'static str {
        if state.pump_damaged {
            "Red emergency lights pulse across the walls. Pump 3 is vibrating violently."
        } else {
            "Pump 3 runs smoothly. The emergency lights have stopped pulsing."
        }
    }

    fn exits(&self) -> Vec<String> {
        vec!["north".into(), "east".into()]
    }

    fn execute(&self, command: &str, state: &mut GameState) -> SceneResult {
        match command {
            "look" | "l" => result(self.look(state)),
            "north" | "n" | "go north" => move_result(
                SceneId::ControlRoom,
                format!("You move north. {}", self.look(state)),
            ),
            "east" | "e" | "go east" => move_result(
                SceneId::RobotBay,
                format!("You move east. {}", self.look(state)),
            ),
            "inspect pump" | "examine pump" => {
                if state.pump_damaged {
                    result("Pump 3 is vibrating violently. The outlet pressure coupling has separated. It could be tightened with a wrench.")
                } else {
                    result("Pump 3 is operating normally.")
                }
            }
            "repair pump" | "fix pump" => {
                if !state.pump_damaged {
                    result("The pump is already repaired.")
                } else if !state.has_wrench {
                    result("You need a wrench to tighten the damaged coupling.")
                } else {
                    state.pump_damaged = false;
                    let mut response = result(
                        "You tighten the pressure coupling. Pump 3 returns to normal operation.",
                    );
                    response.advances_turn = true;
                    response
                }
            }
            _ => result("I do not understand that command."),
        }
    }
}

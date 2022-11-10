use std::collections::HashMap;
use anyhow::*;

type PlayerId = u64;

#[derive(Default)]
pub struct GameState {
    pub players: HashMap<PlayerId, String>,
    history: Vec<GameEvent>,
}

#[derive(Clone)]
pub enum GameEvent {
    PlayerJoined { player_id: PlayerId, name: String },
}

impl GameState {
	pub fn reduce(&mut self, event: &GameEvent) {
		use GameEvent::*;
		match event {
			PlayerJoined { player_id, name } => {
				self.players.insert(*player_id, name.to_string());
			}
		}

		self.history.push(event.clone());
	}
	pub fn validate(&self, event: &GameEvent) -> bool {
		use GameEvent::*;
		match event {
			PlayerJoined { player_id, name: _ } => {
				if self.players.contains_key(player_id) {
					return false;
				}
			}
		}

		true
	}

	pub fn dispatch(&mut self, event: &GameEvent) -> Result<()> {
		if !self.validate(event) {
			bail!("Error on trigger an action. Try again")
		}
		self.reduce(event);
		Ok(())
	}
}

fn main() {
    let mut game_state = GameState::default();
    let event = GameEvent::PlayerJoined { 
        player_id: 1234, 
        name: "Garry K.".to_string() 
    };

	game_state.dispatch(&event).unwrap();

	game_state.dispatch(&event).unwrap();
}
use std::rc::{Rc, Weak};

pub struct Game<'a, P> where P: 'a + Player{
	map: Map<'a, P>,
	players: &'a Vec<P>,
}

impl<'a, P> Game<'a, P> where P: 'a + Player {
	pub fn new(players: &'a Vec<P>) -> Game<'a, P> {
		let mut map = Map::new();
		// TODO: config
		for _ in 0..42 { 
			let f = Field::new();
			map.fields.push(f);
		}
		let n = 42/players.len();
		for p in players.iter() {
			for _ in 0..n {
				// Random free
				let f = map.fields.get_mut(0).unwrap();
				f.owner = Some(p);
			}	
		}
		Game {
			map: map,
			players: players,
		}	
	}

	fn turn(&mut self) {
		for player in self.players.iter() {
			let action = player.action(&self.map);
			match self.map.apply(action) {
				Ok(_) => {},
				Err(_) => {},
			}
		}
	}
}

//TODO: Connections
//TODO: Add RefCell<Rng>
#[derive(Default)]
pub struct Map<'a, P: 'a + Player> {
	fields: Vec<Field<'a, P>>,
}

impl<'a, P> Map<'a, P> where P: 'a + Player {
	fn new() -> Map<'a, P> {
		Map {fields: Vec::new()}
	}
	
	fn apply(&self, action: Action<P>) -> Result<(),()> {
		Ok(())
	}
	pub fn random(&'a self) -> Option<&'a Field<'a, P>> {
		self.fields.get(0)
	}
}

pub trait Player : std::marker::Sized {
	fn action<'a>(&self, map: &'a Map<'a, Self>) -> Action<'a, Self>;
}

pub struct Field<'a, P: 'a + Player> {
	units: u8, 
	owner: Option<&'a P>,
}

impl<'a, P> Field<'a, P> where P: 'a + Player {
	fn new() -> Field<'a, P> {
		Field {
			units: 0,
			owner: None,
		}
	}
}

pub enum Action<'a, P: 'a + Player> {
	Attack(&'a Field<'a, P>, &'a Field<'a, P>),
	Move(&'a Field<'a, P>, &'a Field<'a, P>), 
	Support,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

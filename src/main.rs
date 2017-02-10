extern crate imperium;
use imperium::*;
use std::cmp::{PartialEq, Eq};


struct Human {
	id: u8,
}

impl Human {
	fn new() -> Human {
		Human {
			id : 0,	
		}
	}
}

impl Eq for Human {}
impl PartialEq for Human {
	fn eq(&self, other: &Human) -> bool {
		self.id == other.id
	}
}


impl Player for Human {
	fn action<'a>(&self, map: &'a Map<'a, Self>) -> Action<'a, Self> {
		Action::Attack(map.random().unwrap(), map.random().unwrap())
	}
}

fn main() {
	let players: Vec<_> = (0..4).map(|_| Human::new())
						.collect();
	let game = Game::new(&players);
}

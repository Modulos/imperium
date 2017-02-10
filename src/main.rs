extern crate imperium;
use imperium::*;

struct Human {
}

impl Human {
	fn new() -> Human {
		Human {}
	}
}

impl Player for Human {
	fn action<'a>(&self, map: &'a Map<'a, Self>) -> Action<'a, Self> {
		Attack(map.random().unwrap(), map.random().unwrap())
	}
}

fn main() {
	let players: Vec<_> = (0..4).map(|_| Human::new())
						.collect();
	let game = Game::new(&players);
}

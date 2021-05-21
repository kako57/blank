use crate::State;
use std::collections::HashMap;

pub struct Engine {
	/// current state that the engine stores
	/// when evaluate_state() or get_best_move() is called,
	/// the state will be used for evaluation or finding the best move,
	/// respectively.
	pub state: State,
	/// table to save states and their evaluations
	/// State -> (best move, W-D-L eval)
	table: HashMap<State, (u32, u32, u32)>,
}

impl Engine {
	/// initializes engine instance
	pub fn new() -> Self {
		Engine {
			state: State::new(),
			table: HashMap::new(),
		}
	}
	/// checks for wins
	/// doesn't check who won. just checks if anyone won
	fn is_win_state(&self) -> bool {
		const WINNING_PATTERNS: [(usize, usize, usize); 8] = [
			(1, 2, 3),
			(4, 5, 6),
			(7, 8, 9),
			(1, 4, 7),
			(2, 5, 8),
			(3, 6, 9),
			(1, 5, 9),
			(3, 5, 7),
		];
		for (a, b, c) in WINNING_PATTERNS.iter() {
			if self.state.get(*a) != 0
				&& self.state.get(*a) == self.state.get(*b)
				&& self.state.get(*b) == self.state.get(*c)
			{
				return true;
			}
		}
		false
	}
	/// checks if there are no empty cells in the state
	/// used by evaluate_state to imply draws
	fn no_empty_cells(&self) -> bool {
		for i in 1..=9 {
			if self.state.get(i) == 0 {
				return false;
			}
		}
		true
	}
	/// evaluates the engine's current state
	/// assumes perfect play
	pub fn evaluate_state(&mut self, depth: u8) -> (u32, u32, u32) {
		if self.table.contains_key(&self.state) {
			self.table[&self.state]
		} else if self.is_win_state() {
			let wdl = if depth & 1 == 1 {
				// win
				(1, 0, 0)
			} else {
				// loss
				(0, 0, 1)
			};
			self.table.insert(self.state.clone(), wdl);
			wdl
		} else if self.no_empty_cells() {
			let wdl = (0, 1, 0);
			self.table.insert(self.state.clone(), wdl);
			wdl
		} else {
			let wdl = {
				let mut possible_wdls = Vec::new();
				for i in 1..=9 {
					if self.state.get(i) == 0 {
						self.state.set(i);
						let tmp = self.evaluate_state(depth + 1);
						possible_wdls.push(tmp);
						self.state.unset(i);
					}
				}
				possible_wdls.sort_by(|a, b| {
					if depth & 1 == 0 {
						if a.2 != b.2 {
							a.2.cmp(&b.2)
						} else if a.1 != b.1 {
							a.1.cmp(&b.1)
						} else {
							b.0.cmp(&a.0)
						}
					} else if a.2 != b.2 {
						b.2.cmp(&a.2)
					} else if a.1 != b.1 {
						b.1.cmp(&a.1)
					} else {
						a.0.cmp(&b.0)
					}
				});
				match possible_wdls.first() {
					Some(r) => *r,
					None => (0, 0, 0),
				}
			};
			self.table.insert(self.state.clone(), wdl);
			wdl
		}
	}
	/// gets best move for the current state in engine
	pub fn get_best_move(&mut self, depth: u8) -> (u8, (u32, u32, u32), u8) {
		if self.is_win_state() || self.no_empty_cells() {
			return (depth, self.evaluate_state(depth), 0);
		}

		let mut possible_moves = Vec::new();

		for i in 1..=9 {
			if self.state.get(i) == 0 {
				self.state.set(i);
				let (dp, (w, d, l), _) = self.get_best_move(depth + 1);
				possible_moves.push((dp, (w, d, l), i as u8));
				self.state.unset(i);
			}
		}

		possible_moves.sort_by(|x, y| {
			let a = x.1;
			let b = y.1;
			if depth & 1 == 0 {
				if a.2 != b.2 {
					a.2.cmp(&b.2)
				} else if a.1 != b.1 {
					a.1.cmp(&b.1)
				} else if a.0 != b.0 {
					a.0.cmp(&b.0)
				} else {
					x.0.cmp(&y.0)
				}
			} else if a.2 != b.2 {
				b.2.cmp(&a.2)
			} else if a.1 != b.1 {
				b.1.cmp(&a.1)
			} else if a.0 != b.0 {
				b.0.cmp(&a.0)
			} else {
				x.0.cmp(&y.0)
			}
		});

		if depth == 0 {
			println!("{:?}", possible_moves);
		}

		let (bdepth, (bestw, bestd, bestl), best_move) =
			match possible_moves.first() {
				Some(r) => *r,
				None => (depth, (0, 0, 0), 0 as u8),
			};

		/*
		eprintln!(
			"info depth {} wdl {}-{}-{} curmove {}",
			bdepth, bestw, bestd, bestl, best_move
		);
		*/

		(best_move, (bestw, bestd, bestl), bdepth)
	}
}

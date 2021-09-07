use crate::State;

pub struct Engine {
	/// current state that the engine stores
	/// when evaluate_state() or get_best_move() is called,
	/// the state will be used for evaluation or finding the best move,
	/// respectively.
	pub state: State,
}

impl Engine {
	/// initializes engine instance
	pub fn new() -> Self {
		Engine {
			state: State::new(),
		}
	}
	/// checks for wins
	/// doesn't check who won. just checks if anyone won
	fn is_win_state(&self) -> usize {
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
			let d = self.state.get(*a);
			if d != 0 && d == self.state.get(*b) && self.state.get(*b) == self.state.get(*c) {
				return d;
			}
		}
		0
	}
	/// evaluates the engine's current state
	pub fn evaluate_state(
		&mut self,
		depth: isize,
		mut best_move: &mut usize,
		mut alpha: isize,
		mut beta: isize,
	) -> isize {
		let check_win = self.is_win_state();
		if check_win > 0 {
			if depth & 1 == 1 {
				return 20 - depth;
			} else {
				return depth - 20;
			}
		}
		let available_moves = {
			let mut v = Vec::new();
			for i in 1..=9 {
				if self.state.get(i) == 0 {
					v.push(i);
				}
			}
			v
		};
		if available_moves.is_empty() {
			return 0;
		}
		let mut result;
		let depth = depth + 1;
		if depth & 1 == 1 {
			for cur_move in available_moves {
				self.state.set(cur_move);
				result = self.evaluate_state(depth, &mut best_move, alpha, beta);
				self.state.unset(cur_move);
				if result > alpha {
					alpha = result;
					if depth == 1 {
						*best_move = cur_move;
					}
				} else if alpha >= beta {
					return alpha;
				}
			}
			alpha
		} else {
			for cur_move in available_moves {
				self.state.set(cur_move);
				result = self.evaluate_state(depth, &mut best_move, alpha, beta);
				self.state.unset(cur_move);
				if result < beta {
					beta = result;
				} else if beta <= alpha {
					return beta;
				}
			}
			beta
		}
	}
	/// gets best move for the current state in engine
	pub fn get_best_move(&mut self) -> usize {
		let mut best_move = 0;
		let alpha = isize::MIN;
		let beta = isize::MAX;
		self.evaluate_state(0, &mut best_move, alpha, beta);
		best_move
	}
}

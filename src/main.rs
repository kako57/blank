mod engine;
mod parser;
mod state;

use engine::Engine;
use parser::InputType;
use parser::Parser;
use state::State;

fn main() {
	println!("「」 never loses");
	let mut parser = Parser::new();
	let mut engine = Engine::new();
	loop {
		if parser.read_input().is_err() {
			continue;
		}
		let result = parser.parse_input();
		match result {
			Some(InputType::StateRequest(b, t)) => {
				engine.state.update(b, t);
			}
			Some(InputType::MoveRequest) => {
				let best_move = engine.get_best_move();
				println!("{:?}", best_move);
			}
			Some(InputType::EvalRequest) => {
				let mut best_move = 0;
				let alpha = isize::MIN;
				let beta = isize::MAX;
				let w = engine.evaluate_state(
					0,
					&mut best_move,
					alpha,
					beta,
				);
				println!("{:?}", w);
				println!("{} {} {}", best_move, alpha, beta);
			}
			None => {}
		}
	}
}

use std::io;

pub enum InputType {
    StateRequest(Vec<char>, char),
    MoveRequest,
    EvalRequest,
    QuitRequest,
}

pub struct Parser {
    input_lines: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            input_lines: Vec::new(),
        }
    }
    pub fn read_input(&mut self) -> io::Result<()> {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        self.input_lines = line
            .trim()
            .split_whitespace()
            .map(|s| s.into())
            .collect::<Vec<String>>();
        Ok(())
    }
    pub fn parse_input(&self) -> Option<InputType> {
        if self.input_lines.is_empty() {
            None
        } else {
            match &*self.input_lines[0] {
                "state" => {
                    if self.input_lines.len() != 3 {
                        return None;
                    }
                    let board: Vec<char> = self.input_lines[1].chars().collect();
                    if board.len() != 9 {
                        return None;
                    }
                    for &c in &board {
                        match c {
                            'x' | 'o' | '-' => {}
                            _ => return None,
                        }
                    }
                    let turn = self.input_lines[2].chars().last().unwrap();
                    match turn {
                        'x' | 'o' => {}
                        _ => return None,
                    }
                    Some(InputType::StateRequest(board, turn))
                }
                "move" => Some(InputType::MoveRequest),
                "eval" => Some(InputType::EvalRequest),
                "quit" => Some(InputType::QuitRequest),
                _ => None,
            }
        }
    }
}

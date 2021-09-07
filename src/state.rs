use bit_vec::BitVec;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    /// info about the board
    /// first two bits for turn, then two bits for each of the 9 cells
    info: BitVec,
}

impl State {
    /// creates a state with an empty board
    /// with X having the first turn
    pub fn new() -> Self {
        State {
            info: {
                let mut bv = BitVec::from_elem(20, false);
                bv.set(0, true);
                bv
            },
        }
    }
    /// updates the state based on board and turn input
    pub fn update(&mut self, board: Vec<char>, turn: char) {
        self.info = {
            let mut bv = BitVec::from_elem(20, false);
            match turn {
                'x' => {
                    bv.set(0, true);
                    bv.set(1, false);
                }
                'o' => {
                    bv.set(0, false);
                    bv.set(1, true);
                }
                _ => return,
            };
            for (i, c) in board.iter().enumerate().take(9) {
                match c {
                    'x' => {
                        bv.set(2 * i + 2, true);
                        bv.set(2 * i + 3, false);
                    }
                    'o' => {
                        bv.set(2 * i + 2, false);
                        bv.set(2 * i + 3, true);
                    }
                    '-' => {
                        bv.set(2 * i + 2, false);
                        bv.set(2 * i + 3, false);
                    }
                    _ => return,
                }
            }
            bv
        }
    }
    /// returns what's in the cell (X, O, or empty) in usize (1, 2, or 0)
    /// every cell is 2 bits
    /// get(0) to get turn
    pub fn get(&self, idx: usize) -> usize {
        (*self.info.get(idx * 2 + 1).get_or_insert(false) as usize) * 2
            + (*self.info.get(idx * 2).get_or_insert(false) as usize)
    }
    /// pass the turn to the other
    pub fn switch_turn(&mut self) {
        self.info.set(0, !*self.info.get(0).get_or_insert(false));
        self.info.set(1, !*self.info.get(1).get_or_insert(false));
    }
    /// a move in tictactoe; adds a character in the board
    /// and passes the turn to the other
    pub fn set(&mut self, idx: usize) {
        self.info
            .set(idx * 2, *self.info.get(0).get_or_insert(false));
        self.info
            .set(idx * 2 + 1, *self.info.get(1).get_or_insert(false));
        self.switch_turn();
    }
    pub fn unset(&mut self, idx: usize) {
        self.info.set(idx * 2, false);
        self.info.set(idx * 2 + 1, false);
        self.switch_turn();
    }
}

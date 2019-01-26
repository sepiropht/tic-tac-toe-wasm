use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    board: Vec<Cell>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    EMPTY = 0,
    PLAYER1 = 1,
    PLAYER2 = 2,
    TIE = 3,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor, catch)]
    pub fn new(width: u32) -> Board {
        if width < 3 || width > 10 {
            panic!("Width value must be between 3 and 10 {}", width);
        }
        let mut board = vec![];
        for _ in 0..width.pow(2) {
            board.push(Cell::EMPTY);
        }

        Self { board }
    }
    pub fn get_dim(&self) -> usize {
        (self.board.len() as f64).sqrt() as usize
    }

    #[wasm_bindgen(js_name=getCell)]
    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        //let res = self.board[self.get_index(x, y)];
        //log(format!("{:?}", res));
        self.board[self.get_index(x, y)]
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        self.get_dim() * x + y
    }

    #[wasm_bindgen(js_name=playerMove)]
    pub fn player_move(&mut self, x: usize, y: usize, player: Cell) {
        let index = self.get_index(x, y);
        self.board[index] = player;
    }

    #[wasm_bindgen(js_name=checkWin)]
    pub fn check_win(&self) -> Cell {
        let board_dim = self.get_dim();

        //Check rows
        for row in self.board.chunks(board_dim) {
            if all_equal(row) && row[0] != Cell::EMPTY {
                return row[0]
            }
        }

        //Check columns
        for x in 0..board_dim {
            let mut col = vec![];
            for y in 0..board_dim {
                col.push(self.get_cell(y, x));
            }
            if all_equal(&col) && col[0] != Cell::EMPTY {
                return col[0];
            }
        }
        //check diagonal
        let mut left = vec![];
        let mut right = vec![];

        for i in 0..board_dim {
            left.push(self.get_cell(i, i));
            right.push(self.get_cell(board_dim - i - 1, i));
        }

        for diag in vec![left, right].iter() {
            if all_equal(diag) && diag[0] != Cell::EMPTY {
                return diag[0];
            }
        }

        // check if the game if over
        if self
            .board
            .iter()
            .any(|cell| *cell == Cell::EMPTY)
        {
            return Cell::EMPTY;
        }
        //
        Cell::TIE
    }
    pub fn clone_board(original: &Board) -> Board {
        let mut board = vec![];
        for cell in original.board.iter() {
            board.push(cell.clone());
        }
        Board { board }
    }
}

fn all_equal(v: &[Cell]) -> bool {
    !v.iter().any(|curr| *curr != v[0])
}

#[test]
fn test_constructor() {
    let board = Board::new(3);
    assert!(
        board.board
            == vec![
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY
            ]
    );

    let board = Board::new(4);
    assert!(
        board.board
            == vec![
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY,
                Cell::EMPTY
            ]
    );
}
#[test]
#[should_panic]
fn test_constructor_shoul_panic() {
    let board = Board::new(1);
}

#[test]
fn test_length() {
    let board = Board::new(3);
    assert!(board.get_dim() == 3);
    let board = Board::new(6);
    assert!(board.get_dim() == 6);
}

#[test]
fn test_get_move() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(2, 2, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER1);

    assert!(board.get_cell(0, 0) == Cell::PLAYER1);
    assert!(board.get_cell(2, 2) == Cell::PLAYER2);
    assert!(board.get_cell(1, 2) == Cell::PLAYER1);
    assert!(board.get_cell(0, 1) == Cell::EMPTY);
}



#[test]
fn test_check_player1_win() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(0, 1, Cell::PLAYER2);
    board.player_move(0, 2, Cell::PLAYER1);
    board.player_move(1, 0, Cell::PLAYER1);
    board.player_move(1, 1, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER1);
    board.player_move(2, 0, Cell::PLAYER1);
    board.player_move(2, 1, Cell::PLAYER2);
    board.player_move(2, 2, Cell::PLAYER1);

    assert!(board.check_win() == Cell::PLAYER1);
}

#[test]
fn test_check_player2_win() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(0, 1, Cell::PLAYER2);
    board.player_move(0, 2, Cell::PLAYER2);
    board.player_move(1, 0, Cell::PLAYER1);
    board.player_move(1, 1, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER2);
    board.player_move(2, 0, Cell::PLAYER2);
    board.player_move(2, 1, Cell::PLAYER2);
    board.player_move(2, 2, Cell::PLAYER2);

    assert!(board.check_win() == Cell::PLAYER2);
}

#[test]
fn test_check_diaonal_win() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(0, 1, Cell::PLAYER2);
    board.player_move(0, 2, Cell::PLAYER2);
    board.player_move(1, 0, Cell::PLAYER1);
    board.player_move(1, 1, Cell::PLAYER1);
    board.player_move(1, 2, Cell::EMPTY);
    board.player_move(2, 0, Cell::PLAYER2);
    board.player_move(2, 1, Cell::PLAYER2);
    board.player_move(2, 2, Cell::PLAYER1);

    assert!(board.check_win() == Cell::PLAYER1);
}

#[test]
fn test_check_game_not_finished_yet() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::EMPTY);
    board.player_move(0, 1, Cell::PLAYER2);
    board.player_move(0, 2, Cell::PLAYER2);
    board.player_move(1, 0, Cell::PLAYER1);
    board.player_move(1, 1, Cell::PLAYER1);
    board.player_move(1, 2, Cell::EMPTY);
    board.player_move(2, 0, Cell::PLAYER2);
    board.player_move(2, 1, Cell::PLAYER2);
    board.player_move(2, 2, Cell::PLAYER1);

    assert!(board.check_win() == Cell::EMPTY);
}

#[test]
fn test_check_tie() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER2);
    board.player_move(0, 1, Cell::PLAYER2);
    board.player_move(0, 2, Cell::PLAYER1);
    board.player_move(1, 0, Cell::PLAYER1);
    board.player_move(1, 1, Cell::PLAYER1);
    board.player_move(1, 2, Cell::PLAYER2);
    board.player_move(2, 0, Cell::PLAYER2);
    board.player_move(2, 1, Cell::PLAYER2);
    board.player_move(2, 2, Cell::PLAYER1);

    assert!(board.check_win() == Cell::TIE);
}

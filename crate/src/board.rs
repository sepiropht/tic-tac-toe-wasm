pub struct Board {
    board: Vec<Vec<Cell>>,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    EMPTY = 0,
    PLAYER1 = 1,
    PLAYER2 = 2,
    TIE = 3,
}

impl Board {
    pub fn new(width: u32) -> Board {
        if width < 3 || width > 10 {
            panic!("Width vaalue must be between 3 and 10 {}", width);
        }
        let mut board = vec![];
        for _ in 0..width {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Cell::EMPTY);
            }
            board.push(row);
        }

        Self { board }
    }
    pub fn get_dim(&self) -> usize {
        self.board.len()
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.board[x][y]
    }

    fn player_move(&mut self, x: usize, y: usize, player: Cell) {
        self.board[x][y] = player;
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for x in 0..self.get_dim() {
            for y in 0..self.get_dim() {
                let current_cell = self.board[x][y];
                if current_cell == Cell::EMPTY {
                    v.push((x, y));
                }
            }
        }
        v
    }

    pub fn check_win(&self) -> Cell {
        let board_dim = self.get_dim();

        //Check rows
        for x in 0..board_dim {
            if all_equal(&self.board[x]) && self.board[x][0] != Cell::EMPTY {
                return self.board[x][0];
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
        //TODO check diagonal
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
        if self
            .board
            .iter()
            .flat_map(|row| row.iter().map(|state| state))
            .any(|state| *state == Cell::EMPTY)
        {
            return Cell::EMPTY;
        }
        Cell::TIE
    }
}

fn all_equal(v: &Vec<Cell>) -> bool {
    !v.iter().any(|curr| *curr != v[0])
}

#[test]
fn test_constructor() {
    let board = Board::new(3);
    assert!(
        board.board
            == vec![
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY]
            ]
    );

    let board = Board::new(4);
    assert!(
        board.board
            == vec![
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
                vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY]
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
fn get_empty_cells() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(2, 2, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER1);

    assert!(board.get_empty_cells() == vec![(0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1)]);
}

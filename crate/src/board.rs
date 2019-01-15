pub struct Board {
    board: Vec<Vec<Cell>>,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    EMPTY,
    PLAYER1,
    PLAYER2,
    TIE
}

impl Board {
 
    pub fn new(width: u32) -> Board  {
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
        };

        Self {
            board
        }
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
                    v.push((x,y));
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
              return self.board[x][0]
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
        Cell::EMPTY
    }
    
}
  fn all_equal(v: &Vec<Cell>) -> bool{
       !v.iter().any(|curr| *curr != v[0])
    }

#[test]
fn constructor() {
    let board = Board::new(3); 
    assert!(board.board, vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]]);
}

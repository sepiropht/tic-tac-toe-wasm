use board::Board;
use board::Cell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    fn floor(n: f64) -> f64;
// #[wasm_bindgen(js_namespace = console)]
// // fn log(message: String);
}
#[wasm_bindgen]
pub struct Point {
    x: u32,
    y: u32,
}
#[wasm_bindgen]
pub struct Ai {
    num_trial: u32,
    trial_board: Board,
    scores: Vec<Vec<u32>>,
}

    pub fn get_coord(&self, index: usize) -> Point {
        let x = index / self.get_dim();
        let y = index - (self.get_dim() * x);
        Point {
            x: x as u32,
            y: y as u32
        }
    }
impl Ai {
    fn trial(&mut self, mut player: Cell) {
        let mut empty_cells = self.trial_board.get_empty_cells();

        while self.trial_board.check_win() == Cell::EMPTY {
            let index = floor(random() * empty_cells.len() as f64) as usize;
            let (x, y) = empty_cells[index];
            self.trial_board.player_move(x, y, player);
            empty_cells = self.trial_board.get_empty_cells();
            player = match player {
                Cell::PLAYER1 => Cell::PLAYER2,
                Cell::PLAYER2 => Cell::PLAYER1,
                _ => player,
            }
        }
    }

    pub fn update_scores(&mut self, player: Cell) {
        unimplemented!();
        /* let winner = self.trial_board.check_win();
        let score_player = 2;
        let score_other = 1;
        if winner == Cell::PLAYER1 || winner == Cell::PLAYER2 {
            let other = match player {
                Cell::PLAYER1 => Cell::PLAYER2,
                _ => Cell::PLAYER1
            };

            self.scores.iter().enumerate().for_each(|(row_ind, row)| {
                row.iter().enumerate().for_each(|(cell_ind, _)| {
                  if self.trial_board.get_cell(row_ind, cell_ind) == player {
                      if player == winner {
                          self.scores[row_ind][cell_ind] += score_player;
                      } else {
                          self.scores[row_ind][cell_ind] += score_player;
                      }
                  } else if self.trial_board.get_cell(row_ind, cell_ind) == other {
                      if player == winner {
                          self.scores[row_ind][cell_ind] -= score_other;
                      } else {
                          self.scores[row_ind][cell_ind] += score_other;
                      }
                  }
               })
            })
        }
        */
    }

    pub fn get_best_move(&self, board: &Board) -> (usize, usize) {
        let mut high_scores = board
            .get_empty_cells()
            .iter()
            .map(|(x, y)| (*x, *y, self.scores[*x][*y]))
            .collect::<Vec<(usize, usize, u32)>>();

        high_scores.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        let max_score = high_scores[0].2;
        let high_scores: Vec<(usize, usize, u32)> = high_scores
            .into_iter()
            .filter(|(_, _, score)| *score == max_score)
            .collect();

        let (x, y, _) = if high_scores.len() == 1 {
            high_scores[0]
        } else {
            let index = floor(random() * high_scores.len() as f64) as usize;
            high_scores[index]
        };

        (x, y)
    }

    // Use a Monte Carlo simulation to return a move
    // for the AI player.
    pub fn ai_move(&mut self, current_board: &Board, player: Cell) -> (usize, usize) {
        let mut scores = vec![];
        for _ in 0..current_board.get_dim() {
            let mut row = vec![];
            for _ in 0..current_board.get_dim() {
                row.push(0);
            }
            scores.push(row);
        }
        self.scores = scores;

        for _ in 0..self.num_trial {
            self.trial_board = Board::clone_board(&current_board);;
            self.trial(player);
            self.update_scores(player);
        }
        self.get_best_move(&current_board)
    }
}

 fn get_empty_cells(&board: Board) -> Vec<Point> {
        let mut v = vec![];
        for x in 0..board.get_dim() {
            for y in 0..board.get_dim() {
                let current_cell = board.get_cell(x, y);
                if current_cell == Cell::EMPTY {
                    v.push( Point {
                        x: x as u32,
                        y: y as u32
                    });
                }
            }
        }
        v
    }
/*#[test]
fn test_get_empty_cells() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(2, 2, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER1);

    assert!(board.get_empty_cells() == vec![(0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1)]);
}*/

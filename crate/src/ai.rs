use board::Board;
use board::Cell;
use wasm_bindgen::prelude::*;
extern crate js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}
#[wasm_bindgen]

impl Point {
    #[wasm_bindgen(js_name=getX)]
    pub fn get_x(&self) -> u32 {
        self.x
    }
    #[wasm_bindgen(js_name=getY)]
    pub fn get_y(&self) -> u32 {
        self.y
    }
}
#[wasm_bindgen]
pub struct Ai {
    num_trial: u32,
    trial_board: Board,
    scores: Vec<Vec<u32>>,
}

/*   pub fn get_coord(&self, index: usize) -> Point {
    let x = index / self.get_dim();
    let y = index - (self.get_dim() * x);
    Point {
        x: x as u32,
        y: y as u32
    }
}
*/
#[wasm_bindgen]
impl Ai {
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Ai {
        Ai {
            num_trial: 20, /*TODO: it should be 1000 to be unbeatable but then is too slow
                             event much much slower than a pur js version. The browser frozen
                             */
            trial_board: Board::new(3),
            scores: vec![vec![0]],
        }
    }
    fn trial(&mut self, mut player: Cell) {
        let mut empty_cells = get_empty_cells(&self.trial_board);
        // Simulate a complete game between the currentState of the board and the endgame
        while self.trial_board.check_win() == Cell::EMPTY {
            let index =
                js_sys::Math::floor(js_sys::Math::random() * empty_cells.len() as f64) as usize;
            {
                let pt = &empty_cells[index];
                self.trial_board
                    .player_move(pt.x as usize, pt.y as usize, player);
            }
            empty_cells = get_empty_cells(&self.trial_board);
            player = match player {
                Cell::PLAYER1 => Cell::PLAYER2,
                Cell::PLAYER2 => Cell::PLAYER1,
                _ => player,
            }
        }
    }
    /*Put a weigth on each cell according to the result of the trial
      if the currentPlayer win the trial then all the cells where you have 'ENUM::CURRENTPLAYER' is incremented
      orthewise decremented
      this code is called trial_num times aka 1000 times!!
      this is necessary to make the ai unbeatable
     */
    pub fn update_scores(&mut self, player: Cell) {
        let winner = self.trial_board.check_win();
        let score_player = 2;
        let score_other = 1;
        let len = self.trial_board.get_dim();
        if winner == Cell::PLAYER1 || winner == Cell::PLAYER2 {
            let other = match player {
                Cell::PLAYER1 => Cell::PLAYER2,
                _ => Cell::PLAYER1,
            };
            log("yeah5".to_string());
            for row_ind in 0..len {
                for cell_ind in 0..len {
                    log(format!(
                        "x: {} y: {}, cellValue: {}",
                        row_ind, cell_ind, self.scores[row_ind][cell_ind]
                    ));

                    if self.trial_board.get_cell(row_ind, cell_ind) == player {
                        if player == winner {
                            log("yeah11".to_string());
                        // TODO
                        // help: why these mutations are impossible
                        // at runtime ?
                        //self.scores[row_ind][cell_ind] += score_player
                        } else {
                            log("yeah10".to_string());
                            // self.scores[row_ind][cell_ind] -= score_player
                        }
                    } else if self.trial_board.get_cell(row_ind, cell_ind) == other {
                        if player == winner {
                            log("yeah8".to_string());
                        //self.scores[row_ind][cell_ind] -= score_other
                        } else {
                            log("yeah7".to_string());
                            // self.scores[row_ind][cell_ind] += score_other
                        }
                    }
                }
            }
        }
    }
    // Take all the best scores form self.scores then randomly chose one of them
    pub fn get_best_move(&self, board: &Board) -> Point {
        let mut high_scores = get_empty_cells(&board)
            .iter()
            .map(|pt| {
                (
                    pt.x as usize,
                    pt.y as usize,
                    self.scores[pt.x as usize][pt.y as usize],
                )
            })
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
            let index =
                js_sys::Math::floor(js_sys::Math::random() * high_scores.len() as f64) as usize;
            high_scores[index]
        };

        Point {
            x: x as u32,
            y: y as u32,
        }
    }

    // Use a Monte Carlo simulation to return a move
    // for the AI player.
    #[wasm_bindgen(js_name=aiMove)]
    pub fn ai_move(&mut self, current_board: &Board, player: Cell) -> Point {
        let mut scores = vec![];
        for _ in 0..current_board.get_dim() {
            let mut row = vec![];
            for _ in 0..current_board.get_dim() {
                row.push(0);
            }
            scores.push(row);
        }
        self.scores = scores;
        log("yeah1".to_string());
        for _ in 0..self.num_trial {
            self.trial_board = Board::clone_board(&current_board);;
            log("yeah2".to_string());
            self.trial(player);
            log("yeah3".to_string());
            self.update_scores(player);
            log("yeah4".to_string());
        }
        self.get_best_move(&current_board)
    }
}

fn get_empty_cells(board: &Board) -> Vec<Point> {
    let mut v = vec![];
    for x in 0..board.get_dim() {
        for y in 0..board.get_dim() {
            let current_cell = board.get_cell(x, y);
            if current_cell == Cell::EMPTY {
                v.push(Point {
                    x: x as u32,
                    y: y as u32,
                });
            }
        }
    }
    v
}
#[test]
fn test_get_empty_cells() {
    let mut board = Board::new(3);

    board.player_move(0, 0, Cell::PLAYER1);
    board.player_move(2, 2, Cell::PLAYER2);
    board.player_move(1, 2, Cell::PLAYER1);

    assert!(
        get_empty_cells(&board)
            == vec![
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 }
            ]
    );
}

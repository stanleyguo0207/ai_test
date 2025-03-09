use rand::Rng;
use rand::prelude::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    cells: [[u32; 4]; 4],
    pub score: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            cells: [[0; 4]; 4],
            score: 0,
        };
        board.add_random_tile();
        board.add_random_tile();
        board
    }

    pub fn get_cell(&self, row: usize, col: usize) -> u32 {
        self.cells[row][col]
    }

    pub fn is_game_over(&self) -> bool {
        if self.has_empty_cells() {
            return false;
        }

        for i in 0..4 {
            for j in 0..4 {
                let current = self.cells[i][j];
                if (i < 3 && current == self.cells[i + 1][j]) ||
                   (j < 3 && current == self.cells[i][j + 1]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn has_won(&self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if self.cells[row][col] == 2048 {
                    return true;
                }
            }
        }
        false
    }

    fn has_empty_cells(&self) -> bool {
        self.cells.iter().any(|row| row.iter().any(|&cell| cell == 0))
    }

    fn add_random_tile(&mut self) {
        if !self.has_empty_cells() {
            return;
        }

        let mut rng = rand::thread_rng();
        let mut empty_cells = Vec::new();

        for i in 0..4 {
            for j in 0..4 {
                if self.cells[i][j] == 0 {
                    empty_cells.push((i, j));
                }
            }
        }

        if let Some(&(row, col)) = empty_cells.choose(&mut rng) {
            self.cells[row][col] = if rng.gen_bool(0.9) { 2 } else { 4 };
        }
    }

    pub fn move_tiles(&mut self, direction: Direction) -> bool {
        let old_cells = self.cells;
        let mut moved = false;

        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }

        if old_cells != self.cells {
            moved = true;
            self.add_random_tile();
        }

        moved
    }

    fn move_left(&mut self) {
        for row in 0..4 {
            let mut merged = [false; 4];
            for col in 1..4 {
                if self.cells[row][col] != 0 {
                    let mut new_col = col;
                    while new_col > 0 {
                        if self.cells[row][new_col - 1] == 0 {
                            self.cells[row][new_col - 1] = self.cells[row][new_col];
                            self.cells[row][new_col] = 0;
                            new_col -= 1;
                        } else if !merged[new_col - 1] && 
                                  self.cells[row][new_col - 1] == self.cells[row][new_col] {
                            self.cells[row][new_col - 1] *= 2;
                            self.score += self.cells[row][new_col - 1];
                            self.cells[row][new_col] = 0;
                            merged[new_col - 1] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_right(&mut self) {
        for row in 0..4 {
            let mut merged = [false; 4];
            for col in (0..3).rev() {
                if self.cells[row][col] != 0 {
                    let mut new_col = col;
                    while new_col < 3 {
                        if self.cells[row][new_col + 1] == 0 {
                            self.cells[row][new_col + 1] = self.cells[row][new_col];
                            self.cells[row][new_col] = 0;
                            new_col += 1;
                        } else if !merged[new_col + 1] && 
                                  self.cells[row][new_col + 1] == self.cells[row][new_col] {
                            self.cells[row][new_col + 1] *= 2;
                            self.score += self.cells[row][new_col + 1];
                            self.cells[row][new_col] = 0;
                            merged[new_col + 1] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_up(&mut self) {
        for col in 0..4 {
            let mut merged = [false; 4];
            for row in 1..4 {
                if self.cells[row][col] != 0 {
                    let mut new_row = row;
                    while new_row > 0 {
                        if self.cells[new_row - 1][col] == 0 {
                            self.cells[new_row - 1][col] = self.cells[new_row][col];
                            self.cells[new_row][col] = 0;
                            new_row -= 1;
                        } else if !merged[new_row - 1] && 
                                  self.cells[new_row - 1][col] == self.cells[new_row][col] {
                            self.cells[new_row - 1][col] *= 2;
                            self.score += self.cells[new_row - 1][col];
                            self.cells[new_row][col] = 0;
                            merged[new_row - 1] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_down(&mut self) {
        for col in 0..4 {
            let mut merged = [false; 4];
            for row in (0..3).rev() {
                if self.cells[row][col] != 0 {
                    let mut new_row = row;
                    while new_row < 3 {
                        if self.cells[new_row + 1][col] == 0 {
                            self.cells[new_row + 1][col] = self.cells[new_row][col];
                            self.cells[new_row][col] = 0;
                            new_row += 1;
                        } else if !merged[new_row + 1] && 
                                  self.cells[new_row + 1][col] == self.cells[new_row][col] {
                            self.cells[new_row + 1][col] *= 2;
                            self.score += self.cells[new_row + 1][col];
                            self.cells[new_row][col] = 0;
                            merged[new_row + 1] = true;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        let mut non_zero_count = 0;
        
        for row in 0..4 {
            for col in 0..4 {
                if board.get_cell(row, col) != 0 {
                    non_zero_count += 1;
                }
            }
        }

        assert_eq!(non_zero_count, 2);
    }

    #[test]
    fn test_move_tiles() {
        let mut board = Board::new();
        board.cells = [
            [2, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];

        board.move_tiles(Direction::Left);
        assert_eq!(board.cells[0][0], 4);
        assert_eq!(board.score, 4);
    }

    #[test]
    fn test_game_over() {
        let mut board = Board::new();
        board.cells = [
            [2, 4, 2, 4],
            [4, 2, 4, 2],
            [2, 4, 2, 4],
            [4, 2, 4, 2],
        ];

        assert!(board.is_game_over());
    }

    #[test]
    fn test_win_condition() {
        let mut board = Board::new();
        board.cells = [
            [2048, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];

        assert!(board.has_won());
    }
}
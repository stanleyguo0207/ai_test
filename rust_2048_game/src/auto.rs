use super::*;
use crate::test_visualizer::{TestVisualizer, TestStatus};
use eframe::egui;

pub fn run_auto_test() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(400.0, 600.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "2048 自动测试",
        options,
        Box::new(|_cc| Box::new(AutoTestApp::new())),
    );
}

pub struct AutoTestApp {
    visualizer: TestVisualizer,
    board: Board,
    moves: u32,
    max_moves: u32,
}

impl AutoTestApp {
    pub fn new() -> Self {
        Self {
            visualizer: TestVisualizer::new(),
            board: Board::new(),
            moves: 0,
            max_moves: 1000,
        }
    }

    pub fn with_max_moves(max_moves: u32) -> Self {
        Self {
            visualizer: TestVisualizer::new(),
            board: Board::new(),
            moves: 0,
            max_moves,
        }
    }
}

impl eframe::App for AutoTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.board.has_won() && !self.board.is_game_over() && self.moves < self.max_moves {
                // 尝试所有可能的移动方向
                let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                
                // 记录移动前的状态
                let old_cells = self.board.cells.clone();
                let old_score = self.board.score;
                
                // 尝试每个方向的移动
                let mut moved = false;
                let mut chosen_direction = None;
                for &direction in directions.iter() {
                    if self.board.move_tiles(direction) {
                        moved = true;
                        chosen_direction = Some(direction);
                        break;
                    }
                }
                
                // 如果没有任何方向可以移动，游戏结束
                if !moved {
                    self.visualizer.set_status(TestStatus::Completed);
                    return;
                }
                
                self.moves += 1;
                
                // 验证游戏状态
                if self.board.score < old_score {
                    self.visualizer.add_log("错误：分数减少".to_string());
                    self.visualizer.set_status(TestStatus::Failed);
                    return;
                }
                
                if self.board.cells == old_cells {
                    self.visualizer.add_log("错误：棋盘状态未改变".to_string());
                    self.visualizer.set_status(TestStatus::Failed);
                    return;
                }
                
                // 验证数字的有效性
                for row in 0..4 {
                    for col in 0..4 {
                        let value = self.board.get_cell(row, col);
                        if value != 0 && !value.is_power_of_two() {
                            self.visualizer.add_log(format!("错误：非法数字 {}", value));
                            self.visualizer.set_status(TestStatus::Failed);
                            return;
                        }
                    }
                }
                
                // 更新可视化状态
                self.visualizer.update_board(self.board.clone(), chosen_direction);
                
                // 添加测试日志
                self.visualizer.add_log(format!("移动 {}: 分数 {}", self.moves, self.board.score));
                
                // 请求重绘以保持动画流畅
                ctx.request_repaint();
            } else {
                if self.board.has_won() {
                    self.visualizer.add_log(format!("测试成功：在{}步内达成2048", self.moves));
                    self.visualizer.set_status(TestStatus::Success);
                } else if self.board.is_game_over() {
                    self.visualizer.add_log(format!("游戏结束，最高分数：{}", self.board.score));
                    self.visualizer.set_status(TestStatus::Completed);
                } else {
                    self.visualizer.add_log(format!("达到最大移动次数限制，最高分数：{}", self.board.score));
                    self.visualizer.set_status(TestStatus::Completed);
                }
            }
            
            // 绘制可视化界面
            self.visualizer.draw(ui);
        });
    }
}

pub fn test_merge_mechanics() -> bool {
    let mut board = Board::new();
    board.cells = [
        [2, 2, 4, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];

    // 测试向左移动时的合并
    board.move_tiles(Direction::Left);
    if board.cells[0][0] != 4 || board.cells[0][1] != 8 || board.score != 12 {
        return false;
    }

    // 测试连续合并的情况
    board.cells = [
        [4, 4, 4, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];
    board.score = 0;
    board.move_tiles(Direction::Left);
    if board.cells[0][0] != 8 || board.cells[0][1] != 8 || board.score != 16 {
        return false;
    }

    true
}

pub fn test_move_without_merge() -> bool {
    let mut board = Board::new();
    board.cells = [
        [2, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];
    let initial_score = board.score;

    // 测试向左移动但不合并的情况
    board.move_tiles(Direction::Left);
    board.cells[0][0] == 2 && board.cells[0][1] == 4 && board.score == initial_score
}
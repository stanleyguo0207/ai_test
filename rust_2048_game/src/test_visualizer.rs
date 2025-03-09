use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, Vec2};
use crate::board::{Board, Direction};

pub struct TestVisualizer {
    board: Board,
    current_move: Option<Direction>,
    moves_count: u32,
    test_logs: Vec<String>,
    test_status: TestStatus,
    animation_progress: f32,
}

#[derive(PartialEq)]
pub enum TestStatus {
    Running,
    Success,
    Failed,
    Completed,
}

impl TestVisualizer {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_move: None,
            moves_count: 0,
            test_logs: Vec::new(),
            test_status: TestStatus::Running,
            animation_progress: 0.0,
        }
    }

    pub fn update_board(&mut self, board: Board, direction: Option<Direction>) {
        self.board = board;
        self.current_move = direction;
        self.moves_count += 1;
        self.animation_progress = 0.0;
    }

    pub fn add_log(&mut self, message: String) {
        self.test_logs.push(message);
    }

    pub fn set_status(&mut self, status: TestStatus) {
        self.test_status = status;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        // 显示测试状态和进度
        ui.vertical_centered(|ui| {
            let status_text = match self.test_status {
                TestStatus::Running => "测试运行中...",
                TestStatus::Success => "测试成功！",
                TestStatus::Failed => "测试失败",
                TestStatus::Completed => "测试完成",
            };
            let status_color = match self.test_status {
                TestStatus::Running => Color32::YELLOW,
                TestStatus::Success => Color32::GREEN,
                TestStatus::Failed => Color32::RED,
                TestStatus::Completed => Color32::WHITE,
            };
            ui.colored_label(status_color, status_text);
            ui.label(format!("移动次数: {}", self.moves_count));
        });

        // 绘制游戏棋盘
        let board_size = 300.0;
        let cell_size = board_size / 4.0;
        let board_rect = Rect::from_min_size(
            Pos2::new(
                (ui.available_width() - board_size) / 2.0,
                ui.cursor().min.y + 20.0,
            ),
            Vec2::new(board_size, board_size),
        );

        let painter = ui.painter();

        // 绘制背景
        painter.rect_filled(
            board_rect,
            5.0,
            Color32::from_rgb(187, 173, 160),
        );

        // 绘制每个格子，带有动画效果
        for row in 0..4 {
            for col in 0..4 {
                let cell_value = self.board.get_cell(row, col);
                let mut cell_rect = Rect::from_min_size(
                    Pos2::new(
                        board_rect.min.x + col as f32 * cell_size + 5.0,
                        board_rect.min.y + row as f32 * cell_size + 5.0,
                    ),
                    Vec2::new(cell_size - 10.0, cell_size - 10.0),
                );

                // 添加移动动画效果
                if let Some(direction) = self.current_move {
                    self.animation_progress = (self.animation_progress + 0.1).min(1.0);
                    let offset = 5.0 * (1.0 - self.animation_progress);
                    match direction {
                        Direction::Up => cell_rect = cell_rect.translate(Vec2::new(0.0, -offset)),
                        Direction::Down => cell_rect = cell_rect.translate(Vec2::new(0.0, offset)),
                        Direction::Left => cell_rect = cell_rect.translate(Vec2::new(-offset, 0.0)),
                        Direction::Right => cell_rect = cell_rect.translate(Vec2::new(offset, 0.0)),
                    }
                }

                // 设置格子颜色
                let cell_color = match cell_value {
                    0 => Color32::from_rgb(205, 193, 180),
                    2 => Color32::from_rgb(238, 228, 218),
                    4 => Color32::from_rgb(237, 224, 200),
                    8 => Color32::from_rgb(242, 177, 121),
                    16 => Color32::from_rgb(245, 149, 99),
                    32 => Color32::from_rgb(246, 124, 95),
                    64 => Color32::from_rgb(246, 94, 59),
                    128 => Color32::from_rgb(237, 207, 114),
                    256 => Color32::from_rgb(237, 204, 97),
                    512 => Color32::from_rgb(237, 200, 80),
                    1024 => Color32::from_rgb(237, 197, 63),
                    2048 => Color32::from_rgb(237, 194, 46),
                    _ => Color32::from_rgb(205, 193, 180),
                };

                painter.rect_filled(cell_rect, 5.0, cell_color);

                if cell_value > 0 {
                    let font_size = if cell_value < 100 {
                        24.0
                    } else if cell_value < 1000 {
                        20.0
                    } else {
                        16.0
                    };

                    painter.text(
                        cell_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        cell_value.to_string(),
                        FontId::proportional(font_size),
                        if cell_value <= 4 {
                            Color32::from_rgb(119, 110, 101)
                        } else {
                            Color32::WHITE
                        },
                    );
                }
            }
        }

        // 显示测试日志
        ui.add_space(20.0);
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            for log in self.test_logs.iter().rev().take(5) {
                ui.label(log);
            }
        });
    }
}
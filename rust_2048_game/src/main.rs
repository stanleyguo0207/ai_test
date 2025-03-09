use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, Vec2};
use log::info;
use rand::prelude::*;

mod board;
mod recorder;
use board::{Board, Direction};
use recorder::GameRecorder;

fn main() {
    env_logger::init();
    let mut options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(500.0, 700.0)),
        resizable: true,
        follow_system_theme: true,
        min_window_size: Some(egui::Vec2::new(400.0, 600.0)),
        ..Default::default()
    };
    
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "microsoft_yahei".to_owned(),
        egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/msyh.ttc")).tweak(egui::FontTweak {
            scale: 1.2,
            ..Default::default()
        })
    );
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "microsoft_yahei".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .insert(0, "microsoft_yahei".to_owned());
    options.follow_system_theme = false;

    eframe::run_native(
        "2048 自动测试",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(AutoTestApp::new())
        }),
    );
}

struct AutoTestApp {
    board: Board,
    game_over: bool,
    total_games: u32,
    total_score: u32,
    max_score: u32,
    last_move_time: f64,
    move_interval: f64,
    recorder: Option<GameRecorder>,
    recording_start_time: Option<f64>,
    recording_duration: f64,
}

impl AutoTestApp {
    fn new() -> Self {
        Self {
            board: Board::new(),
            game_over: false,
            total_games: 0,
            total_score: 0,
            max_score: 0,
            last_move_time: 0.0,
            move_interval: 0.2,
            recorder: None,
            recording_start_time: None,
            recording_duration: 30.0, // 录制30秒
        }
    }

    fn auto_move(&mut self) {
        let mut rng = rand::thread_rng();
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let direction = directions.choose(&mut rng).unwrap();
        self.board.move_tiles(*direction);
    }

    fn reset_game(&mut self) {
        self.total_games += 1;
        self.total_score += self.board.score;
        if self.board.score > self.max_score {
            self.max_score = self.board.score;
        }
        self.board = Board::new();
        self.game_over = false;
        info!("游戏重置 - 总场次: {}, 平均分数: {}", 
            self.total_games,
            if self.total_games > 0 { self.total_score / self.total_games } else { 0 }
        );
    }

    fn start_recording(&mut self, window_pos: Pos2) {
        self.recorder = Some(GameRecorder::new(
            window_pos.x as i32,
            window_pos.y as i32, // 移除Y轴偏移
            500, // 使用实际窗口宽度
            700, // 使用实际窗口高度
        ));
        self.recording_start_time = Some(0.0);
    }

    fn update_recording(&mut self, time: f64) {
        if let Some(start_time) = self.recording_start_time {
            if time - start_time >= self.recording_duration {
                if let Some(recorder) = &self.recorder {
                    if let Err(e) = recorder.save_gif("game_recording.gif") {
                        info!("保存GIF失败: {}", e);
                    } else {
                        info!("GIF录制完成！");
                    }
                }
                self.recorder = None;
                self.recording_start_time = None;
            } else if let Some(recorder) = &mut self.recorder {
                recorder.capture_frame();
            }
        }
    }
}

impl eframe::App for AutoTestApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);

        if self.recorder.is_none() && self.recording_start_time.is_none() {
            self.start_recording(frame.info().window_info.position.unwrap_or_default());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示测试统计信息
            ui.vertical_centered(|ui| {
                ui.heading(format!("当前分数: {}", self.board.score));
                ui.label(format!("总场次: {}", self.total_games));
                if self.total_games > 0 {
                    ui.label(format!("平均分数: {}", self.total_score / self.total_games));
                }
                ui.label(format!("最高分数: {}", self.max_score));

                if let Some(start_time) = self.recording_start_time {
                    let remaining_time = (self.recording_duration - (now - start_time)).max(0.0);
                    ui.label(format!("录制中... 剩余时间: {:.1}秒", remaining_time));
                }
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

            // 绘制每个格子
            for row in 0..4 {
                for col in 0..4 {
                    let cell_value = self.board.get_cell(row, col);
                    let cell_rect = Rect::from_min_size(
                        Pos2::new(
                            board_rect.min.x + col as f32 * cell_size + 5.0,
                            board_rect.min.y + row as f32 * cell_size + 5.0,
                        ),
                        Vec2::new(cell_size - 10.0, cell_size - 10.0),
                    );

                    // 绘制格子背景
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

                    // 绘制数字
                    if cell_value > 0 {
                        let text_color = if cell_value <= 4 {
                            Color32::from_rgb(119, 110, 101)
                        } else {
                            Color32::WHITE
                        };

                        let font_size = if cell_value >= 1000 { 24.0 } else { 32.0 };
                        painter.text(
                            cell_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            cell_value.to_string(),
                            FontId::proportional(font_size),
                            text_color,
                        );
                    }
                }
            }
        });

        // 自动移动逻辑
        if now - self.last_move_time >= self.move_interval {
            self.auto_move();
            self.last_move_time = now;

            if self.board.is_game_over() {
                self.reset_game();
            }
        }

        // 更新录制状态
        self.update_recording(now);

        // 请求持续重绘以保持动画流畅
        ctx.request_repaint();
    }
}

mod game;

use std::sync::Arc;

use eframe::egui;
use game::{Board, Color, GameState, PieceType, Pos};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chinese Chess",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(ChessApp::new()))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

struct ChessApp {
    board: Board,
}

impl ChessApp {
    fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chinese Chess");
            match self.board.state {
                GameState::Playing => {
                    ui.label(format!("Turn: {:?}", self.board.turn));
                }
                GameState::Won(winner) => {
                    ui.label(
                        egui::RichText::new(format!("{:?} Wins!", winner))
                            .color(egui::Color32::GOLD)
                            .size(20.0),
                    );
                    if ui.button("Restart").clicked() {
                        self.board = Board::new();
                    }
                }
            }

            let available_size = ui.available_size();
            let board_width = available_size.x.min(available_size.y * 0.9);
            let cell_size = board_width / 10.0;

            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click());

            let offset = response.rect.min
                + egui::vec2(
                    (available_size.x - board_width) / 2.0 + cell_size / 2.0,
                    50.0,
                );

            // Draw grid
            let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);

            // Horizontal lines
            for y in 0..10 {
                let start = offset + egui::vec2(0.0, y as f32 * cell_size);
                let end = offset + egui::vec2(8.0 * cell_size, y as f32 * cell_size);
                painter.line_segment([start, end], stroke);
            }

            // Vertical lines
            for x in 0..9 {
                let start_top = offset + egui::vec2(x as f32 * cell_size, 0.0);
                let end_top = offset + egui::vec2(x as f32 * cell_size, 4.0 * cell_size);
                painter.line_segment([start_top, end_top], stroke);

                let start_bottom = offset + egui::vec2(x as f32 * cell_size, 5.0 * cell_size);
                let end_bottom = offset + egui::vec2(x as f32 * cell_size, 9.0 * cell_size);
                painter.line_segment([start_bottom, end_bottom], stroke);
            }

            // River boundaries
            let river_left_start = offset + egui::vec2(0.0, 4.0 * cell_size);
            let river_left_end = offset + egui::vec2(0.0, 5.0 * cell_size);
            painter.line_segment([river_left_start, river_left_end], stroke);

            let river_right_start = offset + egui::vec2(8.0 * cell_size, 4.0 * cell_size);
            let river_right_end = offset + egui::vec2(8.0 * cell_size, 5.0 * cell_size);
            painter.line_segment([river_right_start, river_right_end], stroke);

            // Palace diagonals
            // Top (Black)
            painter.line_segment(
                [
                    offset + egui::vec2(3.0 * cell_size, 0.0),
                    offset + egui::vec2(5.0 * cell_size, 2.0 * cell_size),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    offset + egui::vec2(5.0 * cell_size, 0.0),
                    offset + egui::vec2(3.0 * cell_size, 2.0 * cell_size),
                ],
                stroke,
            );

            // Bottom (Red)
            painter.line_segment(
                [
                    offset + egui::vec2(3.0 * cell_size, 7.0 * cell_size),
                    offset + egui::vec2(5.0 * cell_size, 9.0 * cell_size),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    offset + egui::vec2(5.0 * cell_size, 7.0 * cell_size),
                    offset + egui::vec2(3.0 * cell_size, 9.0 * cell_size),
                ],
                stroke,
            );

            // Draw pieces
            for y in 0..10 {
                for x in 0..9 {
                    let pos = Pos::new(x, y);
                    let center = offset + egui::vec2(x as f32 * cell_size, y as f32 * cell_size);

                    // Highlight selected
                    if let Some(selected) = self.board.selected {
                        if selected == pos {
                            painter.circle_filled(
                                center,
                                cell_size * 0.45,
                                egui::Color32::from_rgba_premultiplied(0, 255, 0, 100),
                            );
                        }
                    }

                    if let Some(piece) = self.board.get_piece(pos) {
                        let color = match piece.color {
                            Color::Red => egui::Color32::RED,
                            Color::Black => egui::Color32::BLACK,
                        };
                        let bg_color = egui::Color32::from_rgb(240, 220, 180);

                        painter.circle_filled(center, cell_size * 0.4, bg_color);
                        painter.circle_stroke(
                            center,
                            cell_size * 0.4,
                            egui::Stroke::new(2.0, color),
                        );

                        let text = match (piece.color, piece.piece_type) {
                            (Color::Red, PieceType::General) => "帥",
                            (Color::Red, PieceType::Advisor) => "仕",
                            (Color::Red, PieceType::Elephant) => "相",
                            (Color::Red, PieceType::Horse) => "傌",
                            (Color::Red, PieceType::Chariot) => "俥",
                            (Color::Red, PieceType::Cannon) => "炮",
                            (Color::Red, PieceType::Soldier) => "兵",
                            (Color::Black, PieceType::General) => "將",
                            (Color::Black, PieceType::Advisor) => "士",
                            (Color::Black, PieceType::Elephant) => "象",
                            (Color::Black, PieceType::Horse) => "馬",
                            (Color::Black, PieceType::Chariot) => "車",
                            (Color::Black, PieceType::Cannon) => "砲",
                            (Color::Black, PieceType::Soldier) => "卒",
                        };

                        painter.text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            text,
                            egui::FontId::proportional(cell_size * 0.5),
                            color,
                        );
                    }
                }
            }

            // Handle input
            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let relative_pos = pointer_pos - offset;
                    // Round to nearest grid point
                    let x = (relative_pos.x / cell_size).round() as i32;
                    let y = (relative_pos.y / cell_size).round() as i32;

                    if x >= 0 && x < 9 && y >= 0 && y < 10 {
                        let clicked_pos = Pos::new(x as usize, y as usize);

                        if let Some(selected) = self.board.selected {
                            if self.board.move_piece(selected, clicked_pos) {
                                self.board.selected = None;
                            } else {
                                if let Some(piece) = self.board.get_piece(clicked_pos) {
                                    if piece.color == self.board.turn {
                                        self.board.selected = Some(clicked_pos);
                                    } else {
                                        self.board.selected = None;
                                    }
                                } else {
                                    self.board.selected = None;
                                }
                            }
                        } else {
                            if let Some(piece) = self.board.get_piece(clicked_pos) {
                                if piece.color == self.board.turn {
                                    self.board.selected = Some(clicked_pos);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

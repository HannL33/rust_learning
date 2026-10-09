use eframe::egui::{self, Color32, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use ml_project::gomoku_engine::{BOARD_SIZE, Cell, Game, GameResult, Player};

const BACKGROUND: Color32 = Color32::from_rgb(218, 229, 219);
const TEXT: Color32 = Color32::from_rgb(30, 48, 38);
const MUTED: Color32 = Color32::from_rgb(65, 85, 71);
const ACCENT: Color32 = Color32::from_rgb(30, 87, 55);
const STATUS_BACKGROUND: Color32 = Color32::from_rgb(242, 246, 237);
const WOOD: Color32 = Color32::from_rgb(218, 181, 126);
const GRID: Color32 = Color32::from_rgb(114, 83, 48);

pub(super) fn configure_style(ctx: &egui::Context) {
    // Keep the palette consistent even when the system uses a different theme.
    ctx.set_theme(egui::Theme::Light);
    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = BACKGROUND;
    visuals.override_text_color = Some(TEXT);
    visuals.selection.bg_fill = Color32::from_rgb(188, 215, 191);
    visuals.selection.stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(238, 244, 234);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(238, 244, 234);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(201, 224, 199);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(201, 224, 199);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Light, |style| {
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(17.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, egui::FontId::proportional(17.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, egui::FontId::proportional(14.0));
        style.spacing.item_spacing = Vec2::new(12.0, 10.0);
        style.spacing.button_padding = Vec2::new(16.0, 10.0);
    });
}

#[derive(Default)]
pub(super) struct GomokuApp {
    game: Game,
    // History is UI state. Undo rebuilds the game through the engine's public API.
    moves: Vec<(usize, usize)>,
    error: Option<String>,
}

impl GomokuApp {
    fn play(&mut self, position: (usize, usize)) {
        match self.game.play(position) {
            Ok(()) => {
                self.moves.push(position);
                self.error = None;
            }
            Err(error) => self.error = Some(error.to_string()),
        }
    }

    fn undo(&mut self) {
        let Some(remaining) = self.moves.len().checked_sub(1) else {
            return;
        };
        let mut restored = Game::new();
        for &position in &self.moves[..remaining] {
            if let Err(error) = restored.play(position) {
                self.error = Some(format!("Nie udało się cofnąć ruchu: {error}"));
                return;
            }
        }
        // Commit only after the entire history has been replayed successfully.
        self.game = restored;
        self.moves.pop();
        self.error = None;
    }

    fn status(&self) -> String {
        match self.game.result() {
            GameResult::InProgress => format!("Ruch: {}", player_name(self.game.turn())),
            GameResult::Win(player) => format!("Wygrywa {}!", player_name(player)),
            GameResult::Draw => "Remis — plansza jest pełna".into(),
        }
    }

    fn show(&mut self, ui: &mut egui::Ui, viewport_bottom: f32) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("GOMOKU").size(32.0).strong().color(TEXT));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new("15 × 15  /  DWÓCH GRACZY")
                        .color(MUTED)
                        .size(13.0),
                );
            });
        });
        ui.label(RichText::new("Pięć kamieni w linii. Jeden ruch bliżej zwycięstwa.").color(MUTED));
        ui.add_space(8.0);

        egui::Frame::new()
            .fill(STATUS_BACKGROUND)
            .stroke(Stroke::new(1.0, Color32::from_rgb(169, 190, 166)))
            .corner_radius(12.0)
            .inner_margin(14.0)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    let player = match self.game.result() {
                        GameResult::Win(player) => player,
                        _ => self.game.turn(),
                    };
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                    draw_stone(ui.painter(), rect.center(), 11.0, player);
                    ui.label(
                        RichText::new(self.status())
                            .size(23.0)
                            .strong()
                            .color(ACCENT),
                    );
                });
                ui.label(
                    RichText::new(format!(
                        "Czarne (X) zaczynają  ·  Białe (O) grają drugie  ·  Ruchy: {}",
                        self.moves.len()
                    ))
                    .color(MUTED)
                    .size(15.0),
                );
            });
        ui.add_space(8.0);

        // Reserve room below the board; the scroll area handles unusually small windows.
        let side = ui
            .available_width()
            .min((viewport_bottom - ui.cursor().top() - 125.0).max(320.0));
        ui.horizontal(|ui| {
            ui.add_space(((ui.available_width() - side) / 2.0).max(0.0));
            if let Some(position) = self.board_ui(ui, side) {
                self.play(position);
                ui.ctx().request_repaint();
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.moves.is_empty(), egui::Button::new("Cofnij ruch"))
                .clicked()
            {
                self.undo();
            }
            if ui.button("Nowa gra").clicked() {
                *self = Self::default();
            }
            if let Some(&(row, column)) = self.moves.last() {
                ui.label(
                    RichText::new(format!("Ostatni ruch: {}{}", column_label(column), row + 1))
                        .color(MUTED),
                );
            }
        });
        ui.label(RichText::new(
            "Kliknij przecięcie linii. Wygrywa co najmniej 5 kamieni poziomo, pionowo lub po przekątnej."
        ).color(MUTED).size(15.0));
        if let Some(error) = &self.error {
            ui.colored_label(Color32::from_rgb(160, 35, 35), error);
        }
    }

    fn board_ui(&self, ui: &mut egui::Ui, side: f32) -> Option<(usize, usize)> {
        let (response, painter) = ui.allocate_painter(Vec2::splat(side), Sense::click());
        let outer = response.rect;
        let geometry = BoardGeometry::new(outer);
        let radius = geometry.spacing * 0.41;
        painter.rect_filled(outer, 14.0, WOOD);
        painter.rect_stroke(
            outer.shrink(1.0),
            14.0,
            Stroke::new(2.0, GRID),
            egui::StrokeKind::Inside,
        );

        for index in 0..BOARD_SIZE {
            painter.line_segment(
                [
                    geometry.point(index, 0),
                    geometry.point(index, BOARD_SIZE - 1),
                ],
                Stroke::new(1.0, GRID),
            );
            painter.line_segment(
                [
                    geometry.point(0, index),
                    geometry.point(BOARD_SIZE - 1, index),
                ],
                Stroke::new(1.0, GRID),
            );
            painter.text(
                Pos2::new(geometry.point(0, index).x, outer.top() + 15.0),
                egui::Align2::CENTER_CENTER,
                column_label(index),
                egui::FontId::proportional(12.0),
                GRID,
            );
            painter.text(
                Pos2::new(outer.left() + 15.0, geometry.point(index, 0).y),
                egui::Align2::CENTER_CENTER,
                (index + 1).to_string(),
                egui::FontId::proportional(12.0),
                GRID,
            );
        }
        for (row, column) in [(3, 3), (3, 11), (7, 7), (11, 3), (11, 11)] {
            painter.circle_filled(geometry.point(row, column), 3.0, GRID);
        }

        for (row, cells) in self.game.board().cells().iter().enumerate() {
            for (column, &cell) in cells.iter().enumerate() {
                if let Cell::Taken(player) = cell {
                    draw_stone(&painter, geometry.point(row, column), radius, player);
                }
            }
        }
        if let Some(&(row, column)) = self.moves.last() {
            painter.circle_stroke(
                geometry.point(row, column),
                radius * 0.55,
                Stroke::new(2.0, Color32::from_rgb(212, 105, 58)),
            );
        }

        let hovered = response.hover_pos().and_then(|pos| geometry.position(pos));
        if self.game.result() == GameResult::InProgress
            && let Some((row, column)) = hovered
            && self.game.board().cells()[row][column] == Cell::Empty
        {
            let center = geometry.point(row, column);
            let color = match self.game.turn() {
                Player::X => Color32::from_black_alpha(100),
                Player::O => Color32::from_white_alpha(170),
            };
            painter.circle_filled(center, radius, color);
            painter.circle_stroke(center, radius, Stroke::new(1.5, GRID));
            let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
            if response.clicked() {
                return Some((row, column));
            }
        }
        None
    }
}

impl eframe::App for GomokuApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BACKGROUND).inner_margin(22.0))
            .show(ui, |ui| {
                let viewport_bottom = ui.max_rect().bottom();
                egui::ScrollArea::vertical().show(ui, |ui| self.show(ui, viewport_bottom));
            });
    }
}

fn player_name(player: Player) -> &'static str {
    match player {
        Player::X => "Czarne (X)",
        Player::O => "Białe (O)",
    }
}

fn column_label(column: usize) -> char {
    (b'A' + column as u8) as char
}

fn draw_stone(painter: &egui::Painter, center: Pos2, radius: f32, player: Player) {
    painter.circle_filled(
        center + Vec2::new(1.5, 2.0),
        radius,
        Color32::from_black_alpha(45),
    );
    let (fill, outline, highlight) = match player {
        Player::X => (
            Color32::from_rgb(31, 34, 33),
            Color32::from_rgb(12, 17, 15),
            Color32::from_rgb(70, 76, 73),
        ),
        Player::O => (
            Color32::from_rgb(248, 246, 235),
            Color32::from_rgb(162, 148, 123),
            Color32::WHITE,
        ),
    };
    painter.circle_filled(center, radius, fill);
    painter.circle_stroke(center, radius, Stroke::new(1.0, outline));
    painter.circle_filled(
        center + Vec2::new(-radius * 0.25, -radius * 0.28),
        radius * 0.25,
        highlight,
    );
}

struct BoardGeometry {
    grid: Rect,
    spacing: f32,
}

impl BoardGeometry {
    fn new(rect: Rect) -> Self {
        let grid = rect.shrink(34.0);
        Self {
            grid,
            spacing: grid.width() / (BOARD_SIZE - 1) as f32,
        }
    }

    fn point(&self, row: usize, column: usize) -> Pos2 {
        self.grid.min + Vec2::new(column as f32 * self.spacing, row as f32 * self.spacing)
    }

    fn position(&self, pointer: Pos2) -> Option<(usize, usize)> {
        // Half a cell around each edge belongs to the edge intersection.
        if !self.grid.expand(self.spacing * 0.5).contains(pointer) {
            return None;
        }
        let relative = (pointer - self.grid.min) / self.spacing;
        let row = relative.y.round() as isize;
        let column = relative.x.round() as isize;
        if (0..BOARD_SIZE as isize).contains(&row) && (0..BOARD_SIZE as isize).contains(&column) {
            Some((row as usize, column as usize))
        } else {
            None
        }
    }
}

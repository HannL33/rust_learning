use eframe::egui::{self, Color32, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use ml_project::gomoku_engine::{BOARD_SIZE, Cell, Game, GameResult, Player};

const BACKGROUND: Color32 = Color32::from_rgb(218, 229, 219);
const TEXT: Color32 = Color32::from_rgb(30, 48, 38);
const MUTED: Color32 = Color32::from_rgb(65, 85, 71);
const ACCENT: Color32 = Color32::from_rgb(30, 87, 55);
const STATUS_BACKGROUND: Color32 = Color32::from_rgb(242, 246, 237);
const WOOD: Color32 = Color32::from_rgb(218, 181, 126);
const GRID: Color32 = Color32::from_rgb(114, 83, 48);

pub fn configure_style(ctx: &egui::Context) {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    ManVsMan,
    ManVsMachine,
    MachineVsMachine,
}

impl Mode {
    const ALL: [Self; 3] = [Self::ManVsMan, Self::ManVsMachine, Self::MachineVsMachine];

    fn label(self) -> &'static str {
        match self {
            Self::ManVsMan => "Man vs Man",
            Self::ManVsMachine => "Man vs Machine",
            Self::MachineVsMachine => "Machine vs Machine",
        }
    }
}

pub struct GomokuApp {
    game: Game,
    mode: Mode,
    human_player: Player,
    moves: Vec<(usize, usize)>,
    error: Option<String>,
}

impl Default for GomokuApp {
    fn default() -> Self {
        Self {
            game: Game::new(),
            mode: Mode::default(),
            human_player: Player::X,
            moves: Vec::new(),
            error: None,
        }
    }
}

impl GomokuApp {
    fn new_game(&mut self) {
        self.game = Game::new();
        self.moves.clear();
        self.error = None;
    }

    fn human_can_play(&self) -> bool {
        self.game.result() == GameResult::InProgress
            && match self.mode {
                Mode::ManVsMan => true,
                Mode::ManVsMachine => self.game.turn() == self.human_player,
                Mode::MachineVsMachine => false,
            }
    }

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
                self.error = Some(format!("Could not undo the move: {error}"));
                return;
            }
        }
        self.game = restored;
        self.moves.pop();
        self.error = None;
    }

    fn bot_move(&self) -> Option<(usize, usize)> {
        None
    }

    fn status(&self) -> String {
        match self.game.result() {
            GameResult::InProgress => format!("Turn: {}", player_name(self.game.turn())),
            GameResult::Win(player) => format!("{} wins!", player_name(player)),
            GameResult::Draw => "Draw — the board is full".into(),
        }
    }

    fn show(&mut self, ui: &mut egui::Ui, viewport_bottom: f32) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("GOMOKU").size(32.0).strong().color(TEXT));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("15 × 15 BOARD").color(MUTED).size(13.0));
            });
        });
        ui.label(RichText::new("Five stones in a row. One move closer to victory.").color(MUTED));
        self.mode_ui(ui);
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
                        "Black (X) plays first  ·  White (O) plays second  ·  Moves: {}",
                        self.moves.len()
                    ))
                    .color(MUTED)
                    .size(15.0),
                );
                if self.mode != Mode::ManVsMan {
                    ui.label(
                        RichText::new(
                            "No bot connected yet. This mode is ready for a bot to be added.",
                        )
                        .color(TEXT)
                        .size(16.0),
                    );
                }
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
                .add_enabled(!self.moves.is_empty(), egui::Button::new("Undo move"))
                .clicked()
            {
                self.undo();
            }
            if ui.button("New game").clicked() {
                self.new_game();
            }
            if let Some(&(row, column)) = self.moves.last() {
                ui.label(
                    RichText::new(format!("Last move: {}{}", column_label(column), row + 1))
                        .color(MUTED),
                );
            }
        });
        ui.label(RichText::new(
            "Click an intersection. Five or more stones in a row win horizontally, vertically or diagonally."
        ).color(MUTED).size(15.0));
        if let Some(error) = self.error.as_deref() {
            ui.colored_label(Color32::from_rgb(160, 35, 35), error);
        }
    }

    fn mode_ui(&mut self, ui: &mut egui::Ui) {
        let mut mode = self.mode;
        ui.horizontal_wrapped(|ui| {
            for option in Mode::ALL {
                ui.selectable_value(&mut mode, option, option.label());
            }
        });
        if self.mode != mode {
            self.mode = mode;
            self.new_game();
        }

        if mode == Mode::ManVsMachine {
            let mut player = self.human_player;
            ui.horizontal_wrapped(|ui| {
                ui.label("Your stones:");
                ui.selectable_value(&mut player, Player::X, "Black (X)");
                ui.selectable_value(&mut player, Player::O, "White (O)");
            });
            if self.human_player != player {
                self.human_player = player;
                self.new_game();
            }
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
        if self.human_can_play()
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
        if self.game.result() == GameResult::InProgress
            && !self.human_can_play()
            && self.error.is_none()
            && let Some(position) = self.bot_move()
        {
            self.play(position);
            ui.ctx().request_repaint();
        }
    }
}

fn player_name(player: Player) -> &'static str {
    match player {
        Player::X => "Black (X)",
        Player::O => "White (O)",
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

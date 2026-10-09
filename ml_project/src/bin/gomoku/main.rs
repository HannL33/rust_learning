mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([760.0, 860.0])
            .with_min_inner_size([560.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gomoku",
        options,
        Box::new(|cc| {
            app::configure_style(&cc.egui_ctx);
            Ok(Box::<app::GomokuApp>::default())
        }),
    )
}

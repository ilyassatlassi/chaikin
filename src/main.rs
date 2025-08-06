mod app;
use app::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut app = AppState::new();
    loop {
        clear_background(BLACK);
        app.draw();
        app.check_update();
        draw_text(
            "Press ENTER to start smoothing, SPACE || ESC to Exit.",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );
        next_frame().await
    }
}

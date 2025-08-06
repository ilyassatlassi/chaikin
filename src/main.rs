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
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        // println!("ilyass");
        next_frame().await
    }
}

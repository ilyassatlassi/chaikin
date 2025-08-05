pub use macroquad::prelude::*;

pub struct Message {
    pub text: String,
    pub color: Color,
    pub show: bool,
}

// impl Message {
//     pub fn new(text: &str, color: Color, duration: i32) -> Self {
//         Self {
//             text: text.to_string(),
//             color,
//             counter: duration,
//         }
//     }
// }
pub struct AppState {
    // circles: Vec<Vec2>,
    points: Vec<Vec2>,
    original_points: Vec<Vec2>,
    is_enter: bool,
    step: i32,
    message: Message,
    creat_line: bool,
    restart: Rect,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            original_points: Vec::new(),
            is_enter: false,
            step: 0,
            message: Message {
                text: "initialize".to_string(),
                color: RED,
                show: false,
            },
            creat_line: false,
            restart: Rect::new(20.0, 60.0, 100.0, 40.0),
        }
    }

    pub fn draw(&self) {
        for &pos in &self.original_points {
            draw_circle_lines(pos.x, pos.y, 3.0, 1., WHITE);
        }

        if self.creat_line && self.points.len() >= 2 {
            for i in 0..self.points.len() - 1 {
                let p1 = self.points[i];
                let p2 = self.points[i + 1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
            }
        }
        if self.message.show {
            draw_text(&self.message.text, 20.0, 40.0, 24.0, self.message.color);
        }
        draw_rectangle_lines(
            self.restart.x,
            self.restart.y,
            self.restart.w,
            self.restart.h,
            2.0,
            GOLD,
        );
        draw_text(
            "Restart",
            self.restart.x + 10.0,
            self.restart.y + 28.0,
            24.0,
            WHITE,
        );
    }

    pub fn restart(&mut self) {
        self.points.clear();
        self.original_points.clear();
        self.is_enter = false;
        self.step = 0;
        self.message = Message {
            text: "initialize".to_string(),
            color: RED,
            show: false,
        };
        self.creat_line = false;
    }

    pub fn check_update(&mut self) {
        // Update message
        // if let Some(msg) = &mut self.message {
        //     msg.update();
        //     if !msg.is_active() {
        //         self.message = None;
        //     }
        // }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = Vec2::from(mouse_position());

            // Check if restart button is clicked
            if self.restart.contains(mouse_pos) {
                self.restart();
                return;
            }

            if !self.is_enter {
                self.original_points.push(mouse_pos);
                self.points.push(mouse_pos);
            }
        }

        // Handle keyboard input
        if is_key_pressed(KeyCode::Enter) {
            if !self.is_enter && self.original_points.len() >= 2 {
                self.creat_line = true;
                self.is_enter = true;
                self.step = 0;
                // self.original_points = self.points.clone();
            } else if self.original_points.len() < 2 {
                self.message.text = "You need to select two Points.".to_string();
                self.message.show = true;
            }
        }

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}

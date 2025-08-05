pub use macroquad::prelude::*;

pub struct Message {
    pub text: String,
    pub color: Color,
    pub counter: i32,
}

pub struct AppState {
    circles: Vec<Vec2>,
    points: Vec<Vec2>,
    original_points: Vec<Vec2>,
    is_enter: bool,
    step: i32,
    message: Option<Message>,
    creat_line: bool,
    restart: Rect,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            circles: Vec::new(),
            points: Vec::new(),
            original_points: Vec::new(),
            is_enter: false,
            step: 0,
            message: None,
            creat_line: false,
            restart: Rect::new(20.0, 60.0, 100.0, 40.0),
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.restart.x,
            self.restart.y,
            self.restart.w,
            self.restart.h,
            2.0,
            GOLD,
        );
    }

    pub fn restart(&mut self) {
        self.circles.clear();
        self.points.clear();
        self.original_points.clear();
        self.is_enter = false;
        self.step = 0;
        self.message = None;
        self.creat_line = false;
    }
}

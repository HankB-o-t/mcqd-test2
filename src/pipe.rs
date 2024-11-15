use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;

const DT: f32 = 0.5;

pub struct Pipe {
    pub rect: Rect,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
                80.0,20.0
            )}}
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x, self.rect.y/*+ 150.0*/,
            self.rect.w, self.rect.h, RED);}

    pub fn update(&mut self, mv: f32) {
        let num = thread_rng().gen_range(0.0..screen_height() / 2.0);
        if self.rect.x < 10.0 { 
            self.rect.y = num;
            self.rect.x = screen_width();
        }
        self.rect.x -= mv * DT;
    }
}

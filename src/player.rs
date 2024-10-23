use macroquad::prelude::*;
use crate::pipe::Pipe;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0 - 20.0,
                screen_height() / 2.0,
                40.0,40.0
            ),}}
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x, self.rect.y, 
            self.rect.w, self.rect.h, 
            GREEN);}
    pub fn update(&mut self, 
        obj: &Pipe, obj2: &Pipe, obj3: &Pipe) {
        let mut y_move = 0.0;
        if is_key_down(KeyCode::W) { y_move += 1.0 }
        else if self.rect.intersect(obj.rect).is_some()
            || self.rect.intersect(obj2.rect).is_some()
            || self.rect.intersect(obj3.rect).is_some() { 
            y_move = 0.0 }
        else {y_move -= 0.5}
        if is_key_down(KeyCode::P) { self.rect.y = screen_height() / 2.0 }
        self.rect.y += y_move * 30.0 * -0.5;
    }
}

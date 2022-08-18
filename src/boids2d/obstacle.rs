use raylib::prelude::*;

pub struct Obstacle {
    collider: Rectangle,
    color: Color,
}

impl Obstacle {

    pub fn new(collider: Rectangle, color: Color) -> Self {
        Self { collider, color }
    }

    pub fn draw(&self, rdh: &mut RaylibDrawHandle){
        rdh.draw_rectangle(
            self.collider.x as i32,
            self.collider.y as i32,
            self.collider.width as i32,
            self.collider.height as i32,
            self.color
        );
    }

    pub fn collider(&self) -> Rectangle {
        self.collider
    }
}
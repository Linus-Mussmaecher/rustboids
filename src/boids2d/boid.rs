use std::f32::consts::PI;
use raylib::prelude::*;
use crate::boids2d::obstacle::Obstacle;

const SIZE: f32 = 10.0;
const VISION_RANGE: f32 = 100.0;
const VISION_ANGLE: i32 = 230;
const SPEED: f32 = 4.0;
const STEER_FORCE: f32 = 0.8;


const COLORS: &'static [Color] = &[
    Color::PURPLE,
    Color::RED,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::BLUE,
];


pub struct Boid{
    color: Color,
    pos: Vector2,
    dir: Vector2,
    chosen: bool,
}

impl Boid {
    pub fn new(pos: Vector2, dir: Vector2) -> Self {
        let r:i32 = get_random_value(0, 255);
        let g:i32 = get_random_value(0, 255);
        let b:i32 = get_random_value(0, 255);
        let a:i32 = get_random_value(0, 255);
        Self { color: Color::new(
            r as u8,
            g as u8,
            b as u8,
            a as u8,
        ), pos, dir, chosen: false }
    }

    pub fn move_boid(&mut self, boids: &Vec<Boid>, obstacles: &Vec<Obstacle>){

    }

    pub fn draw_boid(&self, rdh: &mut RaylibDrawHandle){
        //calculate angle of direction vector in [0, 2PI[
        let mut angle: f32 = self.dir.y.atan2(self.dir.x);
        if (angle < 0.0){
            angle = angle + 2.0 * PI;
        }

        //calculate what part of the circle is reserved for each color
        let angle_step: f32 = 2.0 * PI / (COLORS.len() as f32);
        //calculate the index of one of the two colors to be blended
        let c1_index: i32 = ((angle / angle_step).floor()) as i32;

        //calclate blending colors
        let c1: Color = COLORS[(c1_index + 0) as usize % COLORS.len()];
        let c2: Color = COLORS[(c1_index + 1) as usize % COLORS.len()];
        //caculate how far the angle is in the current sector of the circle -> determines how much of each color is to be used
        let sp: f32 = (angle - (c1_index as f32) * angle_step) / angle_step;

        rdh.draw_circle(
            self.pos.x as i32, self.pos.y as i32, SIZE/2.0,
            Color::new(
                (c1.r as f32 + sp * (c2.r - c1.r) as f32) as u8,
                (c1.g as f32 + sp * (c2.g - c1.g) as f32) as u8,
                (c1.b as f32 + sp * (c2.b - c1.b) as f32) as u8,
                (c1.a as f32 + sp * (c2.a - c1.a) as f32) as u8,
            )
        );

        if self.chosen {
            rdh.draw_circle(
                self.pos.x as i32, self.pos.y as i32,
                VISION_RANGE, Color::new(255, 255, 255, 100)
            );
            let ray = self.dir.clamp(VISION_RANGE, VISION_RANGE);
            let proj_pos = self.pos + ray;
            rdh.draw_line(
                self.pos.x as i32, self.pos.y as i32,
                proj_pos.x as i32, proj_pos.y as i32,
                Color::new(255, 200, 200, 150)
            );
        }
    }

    pub fn choose(&mut self){
        self.chosen = true;
    }
}
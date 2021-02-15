use ggez::timer;
use ggez::event::{EventHandler};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Image};
use ggez::nalgebra as na;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, DESIRED_FPS};

const TEX_WIDTH: usize = 256;
const TEX_HEIGHT: usize = 256;
const TICK_INCREMENT: f64 = 1. / 80.; 

// code ported from this: https://lodev.org/cgtutor/tunnel.html
pub struct TunnelState {
    texture: Vec<Vec<Vec<u8>>>,
    distances: Vec<Vec<i64>>,
    angles: Vec<Vec<f64>>,
    buffer: Vec<u8>,
    ticks: f64,
}

impl TunnelState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let texture = TunnelState::generate_texture(ctx);
        let (distances, angles) = TunnelState::generate_transformation_tables();
        let buffer = vec![0; SCREEN_HEIGHT * SCREEN_WIDTH * 4];
        let mut state = Self {texture, distances, angles, buffer, ticks: 0.0};
        state.buffer = state.updated_buffer();
        Ok(state)
    }

    fn generate_texture(ctx: &mut Context) -> Vec<Vec<Vec<u8>>> {
        let mut texture = vec![vec![vec![]; TEX_WIDTH]; TEX_HEIGHT];
        let image = Image::new(ctx, "/space.jpg").unwrap().to_rgba8(ctx).unwrap();
        let mut slice_start = 0;
        for y in 0..TEX_HEIGHT {
            for x in 0..TEX_WIDTH {
                texture[y][x] = (&image[slice_start..slice_start + 4])
                    .iter().map(|x| x.clone()).collect();
                slice_start += 4;
            }
        }
        // simple XOR texture, not super pretty, but super simple :D
        /*for y in 0..TEX_HEIGHT {
            for x in 0..TEX_WIDTH {
                let blue = (x * 256 / TEX_WIDTH) as u8 ^ (y * 256 / TEX_HEIGHT) as u8;
                texture[y][x] = vec![0, 0, blue, 255];
            }
        }*/
        texture
    }
    
    //generate non-linear transformation table
    fn generate_transformation_tables() -> (Vec<Vec<i64>>, Vec<Vec<f64>>) {
        let mut distance_table = vec![vec![0; SCREEN_WIDTH * 2]; SCREEN_HEIGHT * 2];
        let mut angle_table = vec![vec![0.0; SCREEN_WIDTH * 2]; SCREEN_HEIGHT * 2];
        for y in 0..SCREEN_HEIGHT * 2 {
            for x in 0..SCREEN_WIDTH * 2 {
                let ratio = 64.0;
                let dy = y as f64 - SCREEN_HEIGHT as f64;
                let dx = x as f64 - SCREEN_WIDTH as f64;
                distance_table[y][x] = (ratio * TEX_HEIGHT as f64 / (dx * dx + dy * dy).sqrt()) as i64 % TEX_HEIGHT as i64;
                angle_table[y][x] = 0.5 * TEX_WIDTH as f64 * dy.atan2(dx) / 3.1416;
            }
        }

        (distance_table, angle_table)
    }

    fn updated_buffer(&self) -> Vec<u8> {
        let shift_x = TEX_WIDTH as f64 * 0.5 * self.ticks;
        let shift_y = TEX_HEIGHT as f64 * 0.25 * self.ticks;
        let x_sin = ((self.ticks / 9.0).sin() + (self.ticks / 7.0).cos()) / 3.0;
        let y_sin = ((self.ticks / 3.0).sin() + (self.ticks / 5.0).cos()) / 3.0;
        let shift_look_x = (SCREEN_WIDTH / 2) as isize + (SCREEN_WIDTH as f64 / 2 as f64 * x_sin) as isize;
        let shift_look_y = (SCREEN_HEIGHT / 2) as isize + (SCREEN_HEIGHT as f64 / 2 as f64 * y_sin) as isize;
        let mut new_buffer = vec![];//vec![vec![vec![0,0,0,0]; SCREEN_HEIGHT]; SCREEN_WIDTH];
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let dx = (x as isize + shift_look_x) as usize;
                let dy = (y as isize + shift_look_y) as usize;
                let tx = ((self.distances[dy][dx] as f64 + shift_x) as i32 % TEX_WIDTH as i32) as usize;
                // oh god why is % not modulo, but remainder (can give negative result)!!!!
                let ty = ((((self.angles[dy][dx] + shift_y) as i32 % TEX_HEIGHT as i32) + TEX_HEIGHT as i32) % TEX_HEIGHT as i32) as usize;
                new_buffer.extend_from_slice(&self.texture[tx][ty]);
            }
        }
        new_buffer
    }
}

impl EventHandler for TunnelState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.ticks += TICK_INCREMENT;
            self.buffer = self.updated_buffer();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let image = Image::from_rgba8(ctx, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, &self.buffer)?;
        graphics::draw(
            ctx, 
            &image, 
            (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}
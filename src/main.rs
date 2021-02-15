#![windows_subsystem = "windows"]
mod tunnel;

use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::audio::{self, SoundSource};
use ggez::conf::WindowMode;
use ggez::input::mouse::{self, MouseButton};
use ggez::mint::Point2;

use std::path;
use std::env;
use tunnel::TunnelState;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const DESIRED_FPS: u32 = 30;

struct MainState {
    audio_intro: audio::Source,
    audio_loop: audio::Source,
    tunnel_state: TunnelState,
    mouse_position: Option<Point2<f32>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut audio_intro = audio::Source::new(ctx, "/razor_1911_23_first_part.mp3").unwrap();
        let mut audio_loop = audio::Source::new(ctx, "/razor_1911_23_second_part.mp3").unwrap();
        audio_intro.set_volume(0.8);
        audio_loop.set_volume(0.8);
        audio_loop.set_repeat(true);
        let tunnel_state = TunnelState::new(ctx)?;

        Ok(Self {audio_intro,
                 audio_loop, 
                 tunnel_state, 
                 mouse_position: None,})
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context) {
        let is_pressed = mouse::button_pressed(ctx, MouseButton::Left);
        let current_pos = mouse::position(ctx);
        if is_pressed {
            if let Some(m_pos) = self.mouse_position {
                let diff = Point2 {x: m_pos.x - current_pos.x, y: m_pos.y - current_pos.y};
                if diff.x != 0.0 || diff.y != 0.0 {
                    let window = graphics::window(ctx);
                    let mut window_pos = window.get_position().unwrap();
                    window_pos.x -= diff.x as f64;
                    window_pos.y -= diff.y as f64;
                    window.set_position(window_pos);
                }
            } else {
                self.mouse_position = Some(current_pos)
            } 
        } else {
            self.mouse_position = None;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.audio_intro.playing() && !self.audio_loop.playing() {
            let _ = self.audio_loop.play();
        }
        self.mouse_motion_event(ctx);
        self.tunnel_state.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.tunnel_state.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("Space travel", "¯\\_(ツ)_/¯")
        .window_mode( WindowMode {
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            resizable: false,
            borderless: true,
            ..WindowMode::default()
        });
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "¯\\_(ツ)_/¯ Space traveller ¯\\_(ツ)_/¯");
    mouse::set_cursor_hidden(ctx, true);

    let state = &mut MainState::new(ctx)?;
    let _ = state.audio_intro.play();

    event::run(ctx, event_loop, state)
}

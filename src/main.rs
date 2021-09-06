mod grid;
mod particle;

use crate::grid::{Grid, WorldPosition};
use crate::particle::{Particle, ParticleType};
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use rand::rngs::ThreadRng;

fn main() {
    let grid = Grid::new((32, 32), (30, 30));
    let window_mode = WindowMode::default().dimensions(
        (grid.cell_size.0 * grid.grid_size.0) as f32,
        (grid.cell_size.1 * grid.grid_size.1) as f32,
    );
    let (mut ctx, event_loop) = ContextBuilder::new("particles", "Jimty")
        .window_setup(ggez::conf::WindowSetup::default().title("Particles"))
        .window_mode(window_mode)
        .build()
        .unwrap();
    let game_state = GameState::new(&mut ctx, grid);

    event::run(ctx, event_loop, game_state);
}

struct GameState {
    grid: Grid,
    particles: Vec<Particle>,
    mouse_state: MouseState,
    rng: ThreadRng,
}

impl GameState {
    pub fn new(_ctx: &mut Context, grid: Grid) -> GameState {
        let rng = rand::thread_rng();
        GameState {
            grid,
            particles: vec![],
            mouse_state: MouseState::new(),
            rng,
        }
    }
    fn _update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.mouse_state.left {
            self.spawn_particle(ctx, ParticleType::Water)
        }
        if self.mouse_state.right {
            self.spawn_particle(ctx, ParticleType::Sand)
        }
        for mut particle in self.particles.iter_mut() {
            particle.update(&mut self.grid, &mut self.rng);
        }
        Ok(())
    }

    fn spawn_particle(&mut self, ctx: &mut Context, type_: ParticleType) {
        let pos = {
            let mouse_pos = ggez::input::mouse::position(ctx);
            WorldPosition::new(mouse_pos.x as i32, mouse_pos.y as i32).to_grid(&self.grid)
        };
        let particle = Particle::new(pos, type_);
        self.particles.push(particle);
    }
}

struct MouseState {
    left: bool,
    right: bool,
}

impl MouseState {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ggez::timer::check_update_time(ctx, 60) {
            self._update(ctx)
        } else {
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        for particle in self.particles.iter() {
            particle.draw(ctx, &self.grid);
        }
        graphics::present(ctx)
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => self.mouse_state.left = false,
            MouseButton::Right => self.mouse_state.right = false,
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        println!("Mouse Down: {},{} ({:?})", x, y, button);
        match button {
            MouseButton::Left => self.mouse_state.left = true,
            MouseButton::Right => self.mouse_state.right = true,
            _ => (),
        }
        /*
        let pos = WorldPosition::new(x as i32, y as i32).to_grid(&self.grid);
        let mut type_ = match button {
            MouseButton::Left => Some(ParticleType::Water),
            MouseButton::Right => Some(ParticleType::Sand),
            _ => None,
        };
        if let Some(type_) = type_.take() {
            let particle = Particle::new(pos, type_);
            self.particles.push(particle);
        }
         */
    }
}

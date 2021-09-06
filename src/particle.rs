use crate::grid::{Direction, Grid, GridPosition};
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum ParticleType {
    Water = 1,
    Sand = 2,
}

impl ParticleType {
    fn as_colour(&self) -> Color {
        match self {
            Self::Water => [0.0, 0.0, 1.0, 1.0].into(),
            Self::Sand => [1.0, 0.741, 0.291, 1.0].into(),
        }
    }
}
#[derive(Debug)]
pub struct Particle {
    pos: GridPosition,
    type_: usize,
    colour: Color,
    last_move: Option<Direction>,
}

impl Particle {
    pub fn new(pos: GridPosition, type_: ParticleType) -> Self {
        Self {
            pos,
            colour: type_.as_colour(),
            type_: type_ as usize,
            last_move: None,
        }
    }
    pub fn update(&mut self, grid: &mut Grid, rng: &mut ThreadRng) {
        if grid.can_move(&Direction::Down, self.pos, self.type_) {
            self.move_(grid, &Direction::Down);
            self.last_move = Some(Direction::Down);
        } else {
            let dirs = {
                let mut dirs = vec![Direction::DownLeft, Direction::DownRight];
                dirs.shuffle(rng);
                dirs
            };
            for dir in dirs.iter() {
                if grid.can_move(dir, self.pos, self.type_) {
                    self.move_(grid, dir);
                    self.last_move = Some(Direction::Down);
                    return;
                }
            }
            let experimental_move = false;
            if experimental_move && self.type_ == (ParticleType::Water as usize) {
                let direction = match &self.last_move {
                    &Some(Direction::Left) => Direction::Left,
                    &Some(Direction::Right) => Direction::Right,
                    _ => {
                        let mut dirs = vec![Direction::Left, Direction::Right];
                        dirs.shuffle(rng);
                        dirs[0]
                    }
                };
                if grid.can_move(&direction, self.pos, self.type_) {
                    self.move_(grid, &direction);
                    self.last_move = Some(direction);
                    return;
                }
            }
        }
    }

    fn move_(&mut self, grid: &mut Grid, dir: &Direction) {
        // Assumes move is valid
        grid.grid[self.pos.into_grid_coord()] = 0;
        self.pos.move_in_direction(dir);
        grid.grid[self.pos.into_grid_coord()] = self.type_ as usize;
    }

    pub fn draw(&self, ctx: &mut Context, grid: &Grid) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos.to_world(grid).to_rect(grid),
            self.colour,
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
    }
}

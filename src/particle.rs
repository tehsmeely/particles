use crate::grid::{Direction, Grid, GridPosition};
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};

#[derive(Debug)]
pub struct Particle {
    pos: GridPosition,
    type_: usize,
    colour: Color,
}

impl Particle {
    pub fn new(pos: GridPosition, colour: Color) -> Self {
        Self {
            pos,
            type_: 1,
            colour,
        }
    }
    pub fn update(&mut self, grid: &mut Grid) {
        if grid.can_move(Direction::Down, self.pos) {
            grid.grid[self.pos.into_grid_coord()] = 0;
            self.pos.move_in_direction(Direction::Down);
            grid.grid[self.pos.into_grid_coord()] = self.type_;
        }
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

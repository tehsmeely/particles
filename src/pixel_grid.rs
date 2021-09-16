use crate::grid::GridPosition;
use crate::pixel::{MaterialType, Pixel};
use array2d::Array2D;
use ggez::{graphics, Context};

#[derive(Debug)]
pub struct Grid {
    pub cell_size: (i32, i32),
    pub grid_size: (i32, i32),
    pub grid: Array2D<Pixel>,
}

impl Grid {
    pub fn new(cell_size: (i32, i32), grid_size: (i32, i32)) -> Self {
        let grid = Array2D::filled_with(Pixel::blank(), grid_size.1 as usize, grid_size.0 as usize);
        Self {
            cell_size,
            grid_size,
            grid,
        }
    }

    pub fn update(&mut self) {
        for y in (0i32..self.grid_size.1).rev() {
            for x in 0i32..self.grid_size.0 {
                if self.grid[(x as usize, y as usize)].material_type_raw() != 0 {
                    let pos = GridPosition::new(x, y);
                    let new_pos = self.grid[(x as usize, y as usize)].update(&pos, &self.grid);
                    if new_pos.is_in_bounds(&self.grid_size) {
                        let tmp = self.grid[new_pos.into_grid_coord()];
                        self.grid[new_pos.into_grid_coord()] = self.grid[pos.into_grid_coord()];
                        self.grid[pos.into_grid_coord()] = tmp
                    } else {
                        self.grid[pos.into_grid_coord()] = Pixel::blank();
                    }
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for y in (0i32..self.grid_size.1).rev() {
            for x in 0i32..self.grid_size.0 {
                let pos = GridPosition::new(x, y);
                let material_type =
                    MaterialType::from_u8(self.grid[(x as usize, y as usize)].material_type_raw());
                let colour = material_type.as_colour();
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    pos.to_world_(&self.cell_size).to_rect_(&self.cell_size),
                    colour,
                )
                .unwrap();
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            }
        }
    }
}

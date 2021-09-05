use crate::particle::Particle;
use array2d::Array2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_in_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn new_in_direction(&self, dir: Direction) -> Self {
        let mut new = self.clone();
        new.move_in_direction(dir);
        new
    }

    pub fn to_world(&self, grid: &Grid) -> WorldPosition {
        WorldPosition::new(self.x * grid.cell_size.0, self.y * grid.cell_size.1)
    }

    pub fn is_at_max_y(&self, grid: &Grid) -> bool {
        self.y == grid.grid_size.1 - 1
    }

    pub fn into_grid_coord(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
impl From<(i32, i32)> for GridPosition {
    fn from((x, y): (i32, i32)) -> Self {
        GridPosition::new(x, y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WorldPosition {
    x: i32,
    y: i32,
}

impl WorldPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn to_grid(&self, grid: &Grid) -> GridPosition {
        GridPosition::new(
            self.x as i32 / grid.cell_size.0,
            self.y as i32 / grid.cell_size.1,
        )
    }

    pub fn to_rect(&self, grid: &Grid) -> ggez::graphics::Rect {
        ggez::graphics::Rect::new_i32(self.x, self.y, grid.cell_size.0, grid.cell_size.1)
    }
}
impl From<(i32, i32)> for WorldPosition {
    fn from((x, y): (i32, i32)) -> Self {
        WorldPosition::new(x, y)
    }
}

#[derive(Debug)]
pub struct Grid {
    pub cell_size: (i32, i32),
    pub grid_size: (i32, i32),
    pub grid: Array2D<usize>,
}

impl Grid {
    pub fn new(cell_size: (i32, i32), grid_size: (i32, i32)) -> Self {
        let grid = Array2D::filled_with(0, grid_size.1 as usize, grid_size.0 as usize);
        Self {
            cell_size,
            grid_size,
            grid,
        }
    }

    pub fn can_move(&self, direction: Direction, pos: GridPosition) -> bool {
        let new_pos = pos.new_in_direction(direction);
        let in_x_bounds = new_pos.x >= 0 && new_pos.x < self.grid_size.0;
        let in_y_bounds = new_pos.y >= 0 && new_pos.y < self.grid_size.1;
        if in_x_bounds && in_y_bounds {
            self.grid[new_pos.into_grid_coord()] == 0
        } else {
            false
        }
    }
}

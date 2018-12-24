use crate::algorithms::MazeAlgorithm;
use crate::rng::RngWrapper;
use crate::grid::{Grid, cell::Cell};

#[derive(Debug)]
pub struct AldousBroder;

/// Aldous Broder Algorithm
/// #1 start by pick a random cell in the grid
/// #2 from that cell, randomly choose a neighbor. 
/// #3a if that neighbor has not been visited, link and set the neighbor as the current cell
/// #3b if that neighbor HAS been visited, set the neighbor as the current cell, but do not link.
/// Repeat until every cell has been visited.
impl MazeAlgorithm for AldousBroder {
    fn on(&self, grid: &Grid, rng_generator: &RngWrapper) {

        let mut cell = grid.random_cell(rng_generator).unwrap();
       
        // this only works becuase the maze is "perfect"
        let mut unvisited_cells = grid.size() - 1;
        while unvisited_cells > 0 {
            let neighbors = cell.borrow().neighbors();
            let index = rng_generator.gen_range(0, neighbors.len());

            let next_neighbor = &neighbors[index].upgrade().unwrap();

            if next_neighbor.borrow().links.is_empty() {
                Cell::link(cell.clone(), next_neighbor.clone(), true);
                unvisited_cells -= 1;
            }

            cell = next_neighbor.clone();
        }
    }
}
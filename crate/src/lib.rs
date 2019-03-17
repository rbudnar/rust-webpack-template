#![feature(const_vec_new)]
#![feature(range_contains)]
#![feature(vec_remove_item)]
#![feature(test)]
#![allow(dead_code)]
#[macro_use]
extern crate cfg_if;
extern crate test;
mod algorithms;
mod rng;
mod grid;
mod cells;

use crate::grid::standard_grid::STANDARD_GRID;
use crate::grid::GridType;
use crate::grid::{Grid,
    standard_grid::StandardGrid,
    distances::DistanceGrid, 
    canvas::*,
    grid_base::GridBase,
    polar_grid::*,
    hex_grid::*,
    triangle_grid::*
};

use crate::cells::{ICellStrong};
use crate::algorithms::{MazeAlgorithm, binary_tree::*, recursive_backtracker::*, aldous_broder::*, hunt_and_kill::*, wilson::*};
use crate::rng::wasm_rng;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

static mut GRID: StandardGrid = StandardGrid {
    grid: GridBase {
        cells: Vec::new(),
        rows: 1, columns: 1,
        cells_: None
    },
};

static mut COLORIZE: bool = true;

static mut SECONDARY_GRID_TYPE: GridType = GridType::StandardGrid;

static mut SECONDARY_GRID:  Option<Box<dyn Grid>> = None;


/****** ALGORITHMS ******/
// This is still broken for the polar grid.
// #[wasm_bindgen]
// pub fn basic_binary_tree(rows: usize, columns: usize) {
//     build_and_display_grid(BinaryTree, rows, columns);    
// }

// #[wasm_bindgen]
// pub fn sidewinder(rows: usize, columns: usize) {
//     build_and_display_grid(Sidewinder, rows, columns);
// }

#[wasm_bindgen]
pub fn aldous_broder(rows: usize, columns: usize) {
    build_and_display_grid(AldousBroder, rows, columns);
}

#[wasm_bindgen]
pub fn wilson(rows: usize, columns: usize) {
    build_and_display_grid(Wilson, rows, columns);
}

#[wasm_bindgen]
pub fn hunt_and_kill(rows: usize, columns: usize) {
    build_and_display_grid(HuntAndKill, rows, columns);
}

#[wasm_bindgen]
pub fn recursive_backtracker(rows: usize, columns: usize) {
    build_and_display_grid(RecursiveBacktracker, rows, columns);
}

/****** OTHER FEATURES ******/

#[wasm_bindgen]
pub fn redisplay_grid() {
    unsafe {
        if let Some(ref grid) = SECONDARY_GRID {
            let secondary_distance_grid = prepare_distance_grid(&**grid);
            // render_maze_web(&**grid, &secondary_distance_grid, COLORIZE);
            grid.to_web(&secondary_distance_grid, COLORIZE);
        }

        // let distance_grid = prepare_distance_grid(&GRID);
        // grid_to_web(&GRID, &distance_grid, COLORIZE);
    }
}

#[wasm_bindgen]
pub fn on_colorize_change(colorize: bool) {
    unsafe {
        COLORIZE = colorize;
        redisplay_grid();
    }
}

#[wasm_bindgen]
pub fn on_grid_type_change(grid_type: &str) {
    unsafe {
        if let Some(_) = &SECONDARY_GRID {
            match SECONDARY_GRID_TYPE {
                GridType::StandardGrid => cleanup_old_canvas(STANDARD_GRID),
                GridType::PolarGrid => cleanup_old_canvas(POLAR_GRID),
                GridType::HexGrid => cleanup_old_canvas(HEX_GRID),
                GridType::TriangleGrid => cleanup_old_canvas(TRIANGLE_GRID),
                _ => ()
            }            
        }

        SECONDARY_GRID_TYPE = match grid_type {
            "standard" => GridType::StandardGrid,
            "polar" => GridType::PolarGrid,
            "hex" => GridType::HexGrid,
            "triangle" => GridType::TriangleGrid,
            _ => GridType::PolarGrid
        };        
    }
}

#[wasm_bindgen]
pub fn add_canvas() {
    append_canvas();
}

#[wasm_bindgen]
pub fn apply_mask() {
    unsafe{
        canvas_to_mask(COLORIZE);
    }
}

/****** HELPERS ******/

fn build_and_display_grid(alg: impl MazeAlgorithm, rows: usize, columns: usize) {
    set_panic_hook();
    unsafe {        
        SECONDARY_GRID = match SECONDARY_GRID_TYPE {
            GridType::PolarGrid => Some(Box::new(PolarGrid::new(rows, columns))),
            GridType::HexGrid => Some(Box::new(HexGrid::new(rows, columns))),
            GridType::TriangleGrid => Some(Box::new(TriangleGrid::new(rows, columns))),
            GridType::StandardGrid => Some(Box::new(StandardGrid::new(rows, columns))),
        };

        if let Some(ref grid) = SECONDARY_GRID {
            render_secondary_grid(&**grid, alg);
        }
    }
}

fn render_secondary_grid(grid: &dyn Grid, alg: impl MazeAlgorithm) {
    let wasm_generator = wasm_rng::WasmRng;
    alg.on(grid, &wasm_generator);
    let distance_grid = prepare_distance_grid(grid);
    unsafe {
        grid.to_web(&distance_grid, COLORIZE);
    }
}

fn prepare_distance_grid(grid: &dyn Grid) -> DistanceGrid {   
    if let Some(root) = grid.cells()[grid.rows() / 2][grid.columns() / 2].clone() {
        DistanceGrid::new(&root)
    }
    else {
        let root = grid.random_cell(&wasm_rng::WasmRng).unwrap();
        DistanceGrid::new(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{CellFormatter, mask::Mask, masked_grid::MaskedGrid};
    use crate::rng::{thread_rng};
    // use crate::test::Bencher;
    use std::fs;

    pub struct ConsoleGridFormatter;
    impl CellFormatter for ConsoleGridFormatter {
        fn contents_of(&self, _cell: &ICellStrong) -> String {
            String::from(" ")
        }

        fn background_color(&self, _cell: &ICellStrong) -> String {
            String::from("")
        }    
    }

    fn test_std_grid(alg: impl MazeAlgorithm) -> StandardGrid {
        let grid = StandardGrid::new(5,5);
        
        let thread_rng = thread_rng::ThreadRng;
        alg.on(&grid, &thread_rng);

        // Prints normal grid without distances.
        println!("{}", grid.to_string(&ConsoleGridFormatter));
        grid
    }

//     #[test]
//     fn binary_tree() {
//         let grid = StandardGrid::new(5,5);

//         let thread_rng = thread_rng::ThreadRng;
//         BinaryTree.on(&grid, &thread_rng);

//         // This prints the grid with Dijkstra's distances inside, rendered as characters a,b,c, etc. 
//         // Will probably need to adjust for really large grids if I really want to display them with distances.
//         // grabs first cell of first row
//         let root = grid.cells().first().unwrap().first().unwrap();
//         let last = grid.cells().last().unwrap().first().unwrap();
//         let mut distance_grid = DistanceGrid::new(&root.as_ref().unwrap());
        
        
//         // builds a path to the first cell of the last row
//         distance_grid.build_path_to(&last.as_ref().unwrap());
//         println!("{}", grid.to_string(&distance_grid));
//         distance_grid.set_show_path_only(true);
        
//         // shows the shortest path from root (NW) to SW corner as constructed above
//         println!("{}", grid.to_string(&distance_grid));

//         // rebuilds path grid to determine and show the longest path
//         distance_grid.build_longest_path(&grid);
//         println!("{}", grid.to_string(&distance_grid));
//     }

//     #[test]
//     fn sidewinder() {
//         test_std_grid(Sidewinder);
//     }

    #[test]
    fn aldous_broder() {
        test_std_grid(AldousBroder);
    }

    #[test]
    fn wilson() {
        test_std_grid(Wilson);
    }

    
    #[test]
    fn hunt_and_kill() {
        test_std_grid(HuntAndKill);
    }

    #[test]
    fn recursive_backtracker() {
        test_std_grid(RecursiveBacktracker);
    }
    
//     #[test]
//     fn kill_cells() {
//         let grid = StandardGrid::new(5,5);
//         let f = grid.cells()[0][0].as_ref().unwrap();
//         let first = f.borrow();
        
//         let first_e = first.east.clone().unwrap().upgrade().unwrap();
//         first_e.borrow_mut().west = None;
//         let first_s = first.south.clone().unwrap().upgrade().unwrap();
//         first_s.borrow_mut().north = None;

//         let l = &grid.cells()[4][4].as_ref().unwrap();
//         let last = l.borrow();
//         let last_w = last.west.clone().unwrap().upgrade().unwrap();
//         last_w.borrow_mut().east = None;
//         let last_n = last.north.clone().unwrap().upgrade().unwrap();
//         last_n.borrow_mut().south = None;


//         let thread_rng = thread_rng::ThreadRng;
//         RecursiveBacktracker.on(&grid, &thread_rng);

//         // Prints normal grid without distances.
//         println!("{}", grid.to_string(&ConsoleGridFormatter));
//     }

//     #[bench]
//     fn hunt_and_kill_bench(b: &mut Bencher) {
//         let grid = StandardGrid::new(5,5);
        
//         let thread_rng = thread_rng::ThreadRng;
//         b.iter(|| HuntAndKill.on(&grid, &thread_rng));

//         println!("{}", grid.to_string(&ConsoleGridFormatter));
//     }


    // #[test]
    // #[ignore]
    // fn dead_ends() {
    //     let algorithms: Vec<Box<MazeAlgorithm>> =
    //         vec![Box::new(BinaryTree), Box::new(Sidewinder), Box::new(AldousBroder), Box::new(Wilson), Box::new(HuntAndKill)];

    //     let tries = 100;
    //     let size = 20;

    //     let thread_rng = thread_rng::ThreadRng;
    //     let mut averages: HashMap<String, f64> = HashMap::new();

    //     for alg in algorithms.iter() {
    //         let mut dead_end_counts: Vec<usize> = vec![];
    //         println!("Running {:?}", alg);
            
    //         for _ in 0..tries {
    //             let mut grid = StandardGrid::new(size,size);
    //             alg.on(&grid, &thread_rng);
    //             dead_end_counts.push(grid.dead_ends().len())
    //         }

    //         let total_deadends = dead_end_counts.iter().fold(0, |acc, x| acc + x);
    //         averages.insert(format!("{:?}", alg), total_deadends as f64 / dead_end_counts.len() as f64);
    //     }

    //     let total_cells = size * size;
    //     println!("Average dead-ends per {}x{} maze ({} cells):", size, size, total_cells);

    //     for (alg, avg) in averages.iter() {
    //         let formatted = format!("{:.*}", 1, (*avg/total_cells as f64) * 100.0);
    //         println!("{} : {} / {} ({:02}%)", alg, avg, total_cells, formatted);
    //     }        
    // }


    #[test]
    fn colors() {
        let grid = StandardGrid::new(5,5);

        let thread_rng = thread_rng::ThreadRng;
        RecursiveBacktracker.on(&grid, &thread_rng);

        // This prints the grid with Dijkstra's distances inside, rendered as characters a,b,c, etc. 
        // Will probably need to adjust for really large grids if I really want to display them with distances.
        // grabs first cell of first row
        let root = grid.cells().first().unwrap().first().unwrap();
        let distance_grid = DistanceGrid::new(root.as_ref().unwrap());
        let color = distance_grid.background_color(&root.as_ref().unwrap());
        assert_eq!(color, "rgb(255,255,255)");

        for row in grid.cells().iter() {
            for cell in row.iter() {
                if let Some(cell) = cell {
                    println!("{}", distance_grid.background_color(&cell));
                }
            }
        }
    }

    #[test]
    fn mask() {
        let mut mask = Mask::new(5, 5);

        mask.set(0,2, false);
        mask.set(1,2, false);
        mask.set(2,2, false);
        mask.set(3,2, false);
        mask.set(4,2, false);
        println!("{}", mask.get(1,2));
        println!("{:#?}", mask);
        println!("{}", mask.count());
        println!("{:?}", mask.rand_location(&thread_rng::ThreadRng));
    }

    #[test]
    fn masked_grid() {
        let mut mask = Mask::new(5, 5);

        mask.set(0,2, false);
        mask.set(1,2, false);
        mask.set(2,2, false);
        
        mask.set(0,0, false);
        mask.set(2,0, false);
        mask.set(3,0, false);

        mask.set(1,4, false);
        mask.set(2,4, false);
        mask.set(4,4, false);
        let masked_grid = MaskedGrid::new(mask);
        RecursiveBacktracker.on(&masked_grid, &thread_rng::ThreadRng);
        println!("{}", masked_grid.grid.to_string(&ConsoleGridFormatter));
    }

    #[test]
    fn basic_grid() {
        let grid = StandardGrid::new(5,5);
        println!("{}", grid.to_string(&ConsoleGridFormatter));    
    }

    #[test]
    fn mask_from_file() {
        let filename = "mask.txt";
        let contents = fs::read_to_string(filename).expect("Error with file");
        let rows = contents.lines().count();
        let cols = contents.lines().map(|line| line.len()).max().unwrap();
        println!("{}, {}", rows, cols);
        let mut mask = Mask::new(rows, cols);
        let X = 'X';
        for (i, line) in contents.lines().enumerate() {
            println!("{}: {}", i, line);
            for (j, c) in line.chars().enumerate() {
                if c == X {
                    mask.set(i, j, false);
                }
            }
        }

        let masked_grid = MaskedGrid::new(mask);
        AldousBroder.on(&masked_grid, &thread_rng::ThreadRng);
        println!("{}", masked_grid.grid.to_string(&ConsoleGridFormatter));
    }
}

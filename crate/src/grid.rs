use rng::RngWrapper;
use std::rc::{Rc};
use std::cell::RefCell;
use cell::*;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<CellLinkStrong>>,
    pub rows: usize, 
    pub columns: usize
}

impl Grid {
    pub fn new(rows: usize, columns: usize)-> Grid {
        Grid {
            cells: Vec::new(),
            rows, columns            
        }
    }

    pub fn to_string(&self, contents: &CellFormatter) -> String {
        let mut output = String::new();
        output += "\r";
        for _ in 0..self.columns {
            output += "+---"
        }
        output += "+\r\n";
        
        for row in self.cells.iter() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");
            for cell in row.iter() {
                let mut body = format!(" {} ", contents.contents_of(&cell));
                let e = cell.borrow();
                let east = e.east.as_ref();
                let east_border = if east.is_some() && cell.borrow().is_linked(Rc::clone(&east.unwrap().upgrade().unwrap())) {
                    " "
                }
                else {
                    "|"
                };

                top += &body;
                top += east_border;

                let south = e.south.as_ref();
                let south_border = if south.is_some() && cell.borrow().is_linked(Rc::clone(&south.unwrap().upgrade().unwrap())) {
                    "   "
                }
                else {
                    "---"
                };
                let corner = String::from("+");
                bottom += south_border;
                bottom += &corner;
            }

            output += &format!("{}\r\n", &top);
            output += &format!("{}\r\n", &bottom);

        }
        
        output
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn random_cell(&self, rng: &RngWrapper) -> Option<CellLinkStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
        println!("{} {}", row, col);
        self.get_cell(row, col)
    }

    pub fn each_cell(&self) -> Vec<CellLinkStrong> { 
        self.cells.iter()
            .flatten()                
            .map(|x| Rc::clone(x))
            .collect()        
    }

    pub fn configure_cells(&mut self) {
        for (_, row) in &mut self.cells.iter().enumerate() {
            for (_, cell) in &mut row.iter().enumerate() {            
                // can't subtract from a usize of 0 apparently
                let cell_row = cell.borrow().row;
                if cell_row > 0 {
                    let north = self.get_cell(cell_row - 1, cell.borrow().column);
                    if north.is_some() {
                        cell.borrow_mut().north = Some(Rc::downgrade(&Rc::clone(&north.unwrap())));
                    }
                }

                let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                if south.is_some() {
                    cell.borrow_mut().south = Some(Rc::downgrade(&Rc::clone(&south.unwrap())));
                }

                let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                if east.is_some() {
                    cell.borrow_mut().east = Some(Rc::downgrade(&Rc::clone(&east.unwrap())));
                }

                let cell_column = cell.borrow().column;
                if cell_column > 0 {
                    let west = self.get_cell(cell.borrow().row, cell_column - 1);
                    if west.is_some() {
                        cell.borrow_mut().west = Some(Rc::downgrade(&Rc::clone(&west.unwrap())));
                    }
                }
            }
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        Some(Rc::clone(&self.cells[row][column]))
    }

    pub fn prepare_grid(&mut self) {
        for i in 0..self.rows {
            let mut row: Vec<CellLinkStrong> = Vec::new();
            for j in 0..self.columns {
                row.push(Rc::new(RefCell::new(Cell::new(i as usize, j as usize))));
            }
            self.cells.push(row);
        }   
    }
}

pub fn link(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {    
    let newlink: CellLinkWeak = Rc::downgrade(&other);
    _self.borrow_mut().links.push(newlink);
    if bidir {
        link(Rc::clone(&other), Rc::clone(&_self), false);
    }
}

pub fn unlink(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {
    let index = _self.borrow().index_of_other(Rc::clone(&other));

    if let Some(i) = index {
        _self.borrow_mut().links.remove(i);
    }

    if bidir {
        unlink(Rc::clone(&other), Rc::clone(&_self), false);
    }
}

pub trait CellFormatter {
    fn contents_of(&self, cell: &CellLinkStrong) -> String;
    fn background_color(&self, cell: &CellLinkStrong) -> String;

}

pub struct StandardGrid;
impl CellFormatter for StandardGrid {
    fn contents_of(&self, _cell: &CellLinkStrong) -> String {
        String::from(" ")
    }

    fn background_color(&self, _cell: &CellLinkStrong) -> String {
        String::from("")
    }    
}
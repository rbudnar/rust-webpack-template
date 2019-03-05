use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;
pub type ICellStrong = Rc<RefCell<ICell>>;
pub type ICellWeak = Weak<RefCell<ICell>>;


pub trait ICell {
    fn neighbors(&self) -> Vec<ICellStrong>;
    fn links(&self) -> Vec<Option<ICellStrong>>;
    fn link(&mut self, other: ICellStrong);
    fn as_any(&self) -> &dyn Any;
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn index(&self) -> usize;
}

impl Debug for ICell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "")
    }
}

impl PartialEq for ICell {
    fn eq(&self, rhs: &ICell) -> bool {
        self.index() == rhs.index()
    }
}

pub type CellLinkWeak = Weak<RefCell<Cell>>; // Think of a better name
pub type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub index: usize,
    pub links: Vec<Option<CellLinkWeak>>,
    pub north: Option<CellLinkWeak>,
    pub south: Option<CellLinkWeak>,
    pub east: Option<CellLinkWeak>,
    pub west: Option<CellLinkWeak>,
    self_rc: Weak<RefCell<Cell>>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for Cell {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn row(&self) -> usize {
        self.row
    }
    fn column(&self) -> usize {
        self.column
    }
    fn index(&self) -> usize {
        self.index
    }

    fn neighbors(&self) -> Vec<ICellStrong> {
        let mut vec: Vec<CellLinkWeak> = vec![];

        if let Some(ref north) = self.north {
            vec.push(north.clone());
        }

        if let Some(ref south) = self.south {
            vec.push(south.clone());
        }

        if let Some(ref east) = self.east {
            vec.push(east.clone());
        }

        if let Some(ref west) = self.west {
            vec.push(west.clone());
        }

        let v: Vec<ICellStrong> = vec.iter()
            .map(|c| {
                let cell = c.upgrade();
                let cell = cell.unwrap();

                let cell: ICellStrong = Rc::clone(&cell) as ICellStrong;
                cell
            })
            .collect();

        v
    }

    fn links(&self) -> Vec<Option<ICellStrong>> {
        self.links.iter()
            .map(|c| {
                let c1 = c.as_ref().unwrap().upgrade();
                let c2 = c1.unwrap();
                let cell: ICellStrong = Rc::clone(&c2) as ICellStrong;
                Some(cell)
            }).collect()
    }

    fn link(&mut self, other: ICellStrong) {       
        if let Some(nl) = other.borrow().as_any().downcast_ref::<Cell>() {
            let _other: CellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
            self.links.push(Some(_other));
        }
    }
}

impl Cell {
    pub fn new(row: usize, column: usize, index: usize) -> CellLinkStrong {
        let c = Cell {
            row, column, 
            north: None, 
            south: None, 
            east: None, 
            west: None, 
            links: Vec::new(), 
            index,
            self_rc: Weak::new(),
        };

        let rc = Rc::new(RefCell::new(c));
        rc.borrow_mut().self_rc = Rc::downgrade(&rc);

        rc
    }

    pub fn is_linked(&self, other: CellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: CellLinkStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : CellLinkStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }

    // pub fn unlink(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {
    //     let index = _self.borrow().index_of_other(Rc::clone(&other));

    //     if let Some(i) = index {
    //         _self.borrow_mut().links.remove(i);
    //     }

    //     if bidir {
    //         Cell::unlink(Rc::clone(&other), Rc::clone(&_self), false);
    //     }
    // }    
}
// pub const NUM_SIDE_TOTAL : usize = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Live,
    Dead
}
impl Default for Status {
    fn default() -> Status {
        Status::Dead
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    status: Status,
    position: (i32, i32)
}

impl Cell {
    //  init
    pub fn new() -> Cell {
        Cell {
            status: Status::Dead,
            position: (0, 0)
        }
    }
    //  geter
    pub fn is_alive(&mut self) -> bool {
        self.status == Status::Live
    }
    pub fn get_x(&mut self) -> i32 {
        self.position.0
    }
    pub fn get_y(&mut self) -> i32 {
        self.position.1
    }
    //  setter
    pub fn set_x(&mut self, x: i32) {
        self.position.0 = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.position.1 = y;
    }
    pub fn change_status(&mut self, next: bool) {
        if next {
            self.status = Status::Live;
        }
        else {
            self.status = Status::Dead;
        }
    }
    pub fn reverse_status(&mut self) {
        if self.status == Status::Dead {
            self.status = Status::Live;
        }
        else {
            self.status = Status::Dead;
        }
    }
    // helper for easy display
    pub fn get_status_in_string(&mut self) -> &str {
        if self.status == Status::Live {
            return "▆▆";
        }
        else {
            return "口";
        }
    }
}
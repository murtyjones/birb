use std::fmt;

#[derive(Copy, Clone)]
pub enum Year {
    TwentySixteen = 2016,
    TwentySeventeen = 2017,
    TwentyEighteen = 2018,
    TwentyNineteen = 2019,
}

#[derive(Copy, Clone)]
pub enum Quarter {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            _ => *self as i32,
        };
        write!(f, "{}", printable)
    }
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            _ => *self as i32,
        };
        write!(f, "{}", printable)
    }
}

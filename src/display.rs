use crate::{Allele, Gene, Genome3, Genome4};
use itertools::Itertools;
use std::fmt;

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Allele::*;

        match self.0 {
            [Zero, Zero] => write!(f, "00"),
            [Zero, One] | [One, Zero] => write!(f, "01"),
            [One, One] => write!(f, "11"),
        }
    }
}

impl fmt::Display for Genome3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().format(":"))
    }
}

impl fmt::Display for Genome4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().format(":"))
    }
}

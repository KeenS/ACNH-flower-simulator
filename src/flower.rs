use rand::prelude::*;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Allele {
    Zero,
    One,
}

impl Allele {
    fn to_bit(&self) -> u8 {
        use Allele::*;
        match self {
            Zero => 0,
            One => 1,
        }
    }
}

impl Add for Allele {
    type Output = Gene;
    fn add(self, other: Self) -> Self::Output {
        use Allele::*;
        match (self, other) {
            (Zero, Zero) => Gene([Zero, Zero]),
            (Zero, One) | (One, Zero) => Gene([Zero, One]),
            (One, One) => Gene([One, One]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gene(pub [Allele; 2]);

impl Gene {
    pub fn to_bits(&self) -> u8 {
        self.0[0].to_bit() + self.0[1].to_bit()
    }

    pub fn choose<R: Rng>(&self, rng: &mut R) -> &Allele {
        self.0.choose(rng).unwrap()
    }
}

impl Gene {
    const ZERO: Gene = Gene([Allele::Zero, Allele::Zero]);
    const ONE: Gene = Gene([Allele::Zero, Allele::One]);
    const TWO: Gene = Gene([Allele::One, Allele::One]);

    pub fn zero() -> Self {
        use Allele::*;
        Self([Zero, Zero])
    }
    pub fn one() -> Self {
        use Allele::*;
        Self([Zero, One])
    }
    pub fn two() -> Self {
        use Allele::*;
        Self([One, One])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Genome3(pub [Gene; 3]);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Genome4(pub [Gene; 4]);

impl Genome3 {
    pub fn genotype(&self) -> u8 {
        let g = self.0;
        0b00 << 6 + g[2].to_bits() << 4 + g[1].to_bits() << 2 + g[0].to_bits()
    }
}

impl Genome4 {
    pub fn genotype(&self) -> u8 {
        let g = self.0;
        g[3].to_bits() << 6 + g[2].to_bits() << 4 + g[1].to_bits() << 2 + g[0].to_bits()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlowerKind {
    /// red, yellow, white, shade
    Rose(Genome4),
    /// red, yellow, shade
    Cosmos(Genome3),
    /// red, yellow, shade
    Lily(Genome3),
    /// red, yellow, white
    Pansy(Genome3),
    /// red, yellow, shade
    Tulip(Genome3),
    /// red, yellow, white
    Hyacinth(Genome3),
    /// red, yellow, white
    Mum(Genome3),
    /// red, orange, white
    Windflower(Genome3),
}

impl FlowerKind {
    pub fn rose_red() -> Self {
        use Gene as G;
        FlowerKind::Rose(Genome4([G::two(), G::zero(), G::two(), G::one()]))
    }
    pub fn rose_yellow() -> Self {
        use Gene as G;
        FlowerKind::Rose(Genome4([G::zero(), G::two(), G::two(), G::zero()]))
    }
    pub fn rose_white() -> Self {
        use Gene as G;
        FlowerKind::Rose(Genome4([G::zero(), G::zero(), G::one(), G::zero()]))
    }

    pub fn cosmos_red() -> Self {
        use Gene as G;
        FlowerKind::Cosmos(Genome3([G::two(), G::zero(), G::zero()]))
    }
    pub fn cosmos_yellow() -> Self {
        use Gene as G;
        FlowerKind::Cosmos(Genome3([G::zero(), G::two(), G::one()]))
    }
    pub fn cosmos_white() -> Self {
        use Gene as G;
        FlowerKind::Cosmos(Genome3([G::zero(), G::zero(), G::one()]))
    }

    pub fn lily_red() -> Self {
        use Gene as G;
        FlowerKind::Lily(Genome3([G::two(), G::zero(), G::one()]))
    }
    pub fn lily_yellow() -> Self {
        use Gene as G;
        FlowerKind::Lily(Genome3([G::zero(), G::two(), G::zero()]))
    }
    pub fn lily_white() -> Self {
        use Gene as G;
        FlowerKind::Lily(Genome3([G::zero(), G::zero(), G::two()]))
    }

    pub fn pansy_red() -> Self {
        use Gene as G;
        FlowerKind::Pansy(Genome3([G::two(), G::zero(), G::two()]))
    }
    pub fn pansy_yellow() -> Self {
        use Gene as G;
        FlowerKind::Pansy(Genome3([G::zero(), G::two(), G::two()]))
    }
    pub fn pansy_white() -> Self {
        use Gene as G;
        FlowerKind::Pansy(Genome3([G::zero(), G::zero(), G::two()]))
    }

    pub fn tulip_red() -> Self {
        use Gene as G;
        FlowerKind::Tulip(Genome3([G::two(), G::zero(), G::one()]))
    }
    pub fn tulip_yellow() -> Self {
        use Gene as G;
        FlowerKind::Tulip(Genome3([G::zero(), G::two(), G::zero()]))
    }
    pub fn tulip_white() -> Self {
        use Gene as G;
        FlowerKind::Tulip(Genome3([G::zero(), G::zero(), G::one()]))
    }

    pub fn hyacinth_red() -> Self {
        use Gene as G;
        FlowerKind::Hyacinth(Genome3([G::two(), G::zero(), G::one()]))
    }
    pub fn hyacinth_yellow() -> Self {
        use Gene as G;
        FlowerKind::Hyacinth(Genome3([G::zero(), G::two(), G::two()]))
    }
    pub fn hyacinth_white() -> Self {
        use Gene as G;
        FlowerKind::Hyacinth(Genome3([G::zero(), G::zero(), G::one()]))
    }

    pub fn mum_red() -> Self {
        use Gene as G;
        FlowerKind::Mum(Genome3([G::two(), G::zero(), G::two()]))
    }
    pub fn mum_yellow() -> Self {
        use Gene as G;
        FlowerKind::Mum(Genome3([G::zero(), G::two(), G::two()]))
    }
    pub fn mum_white() -> Self {
        use Gene as G;
        FlowerKind::Mum(Genome3([G::zero(), G::zero(), G::one()]))
    }

    pub fn windflower_red() -> Self {
        use Gene as G;
        FlowerKind::Windflower(Genome3([G::two(), G::zero(), G::two()]))
    }
    pub fn windflower_yellow() -> Self {
        use Gene as G;
        FlowerKind::Windflower(Genome3([G::zero(), G::two(), G::two()]))
    }
    pub fn windflower_white() -> Self {
        use Gene as G;
        FlowerKind::Windflower(Genome3([G::zero(), G::zero(), G::one()]))
    }

    pub fn same_kind(&self, other: &Self) -> bool {
        use FlowerKind::*;
        match (self, other) {
            (Rose(_), Rose(_))
            | (Cosmos(_), Cosmos(_))
            | (Lily(_), Lily(_))
            | (Pansy(_), Pansy(_))
            | (Tulip(_), Tulip(_))
            | (Hyacinth(_), Hyacinth(_))
            | (Mum(_), Mum(_))
            | (Windflower(_), Windflower(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FlowerState {
    Sprout,
    Stem,
    Bud,
    Blooming,
    Picked,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flower {
    kind: FlowerKind,
    state: FlowerState,
    crossed: bool,
    // showered
    // flags
}

impl Flower {
    pub fn sprout(kind: FlowerKind) -> Self {
        Flower {
            kind,
            state: FlowerState::Sprout,
            crossed: false,
        }
    }

    pub fn kind(&self) -> &FlowerKind {
        &self.kind
    }

    pub fn matured(&self) -> bool {
        use FlowerState::*;
        match self.state {
            Sprout | Bud | Stem => false,
            Blooming | Picked => true,
        }
    }

    pub fn growup(&mut self) {
        use FlowerState::*;
        match self.state {
            Sprout => self.state = Stem,
            Stem => self.state = Bud,
            Bud => self.state = Blooming,
            Blooming => (),
            Picked => self.state = Blooming,
        }
    }
    pub fn refresh(&mut self) {
        self.crossed = false;
    }
}

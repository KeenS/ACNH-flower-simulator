use crate::Flower;
use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Copy, Clone)]
struct Checked(usize);

impl Add<usize> for Checked {
    type Output = Option<usize>;
    fn add(self, other: usize) -> Self::Output {
        self.0.checked_add(other)
    }
}
impl Sub<usize> for Checked {
    type Output = Option<usize>;
    fn sub(self, other: usize) -> Self::Output {
        self.0.checked_sub(other)
    }
}

// +------> x
// |
// |  vec![
// |    ...,
// |  ]
// v
//
// y
#[derive(Debug)]
pub struct Board {
    data: Vec<Option<Flower>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            data: vec![None; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.width
    }

    pub fn is_in_bound(&self, (x, y): (usize, usize)) -> bool {
        x < self.width() && y < self.height()
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&Option<Flower>> {
        let width = self.width();
        self.data.get(y * width + x)
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Option<Flower>> {
        let width = self.width();
        self.data.get_mut(y * width + x)
    }

    pub fn neigbor_positions((x, y): (usize, usize)) -> [(Option<usize>, Option<usize>); 8] {
        let x = Checked(x);
        let y = Checked(y);

        [
            (x - 1, y - 1),
            (x + 0, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 0),
            (x + 1, y + 0),
            (x - 1, y + 1),
            (x + 0, y + 1),
            (x + 1, y + 1),
        ]
    }

    pub fn neighbors(&self, (x, y): (usize, usize)) -> [Option<&Flower>; 8] {
        fn at(board: &Board, x: Option<usize>, y: Option<usize>) -> Option<&Flower> {
            match (x, y) {
                (Some(x), Some(y)) => board.get((x, y)).map(|o| o.as_ref()).flatten(),
                _ => None,
            }
        }
        let x = Checked(x);
        let y = Checked(y);
        [
            at(self, x - 1, y - 1),
            at(self, x + 0, y - 1),
            at(self, x + 1, y - 1),
            at(self, x - 1, y + 0),
            at(self, x + 1, y + 0),
            at(self, x - 1, y + 1),
            at(self, x + 0, y + 1),
            at(self, x + 1, y + 1),
        ]
    }

    pub fn flower_positions(&self) -> FlowerPositions {
        FlowerPositions::new(self)
    }

    pub fn flowers_mut(&mut self) -> FlowersMut {
        FlowersMut::new(self)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Option<Flower>;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        self.get(pos).expect("index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        self.get_mut(pos).expect("index out of bounds")
    }
}

pub struct FlowerPositions<'a> {
    board: &'a Board,
    position: (usize, usize),
}

impl<'a> FlowerPositions<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            position: (0, 0),
        }
    }
}

impl<'a> Iterator for FlowerPositions<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.position;
        for yi in y..self.board.height() {
            for xi in x..self.board.width() {
                match self.board.get((xi, yi)) {
                    Some(Some(_)) => {
                        self.position = (xi + 1, yi);
                        return Some((xi, yi));
                    }
                    _ => continue,
                }
            }
        }
        None
    }
}

pub struct FlowersMut<'a> {
    inner: std::iter::FilterMap<
        std::slice::IterMut<'a, Option<Flower>>,
        fn(&'a mut Option<Flower>) -> Option<&'a mut Flower>,
    >,
}

impl<'a> FlowersMut<'a> {
    fn new(board: &'a mut Board) -> Self {
        Self {
            inner: board.data.iter_mut().filter_map(|o| o.as_mut()),
        }
    }
}

impl<'a> Iterator for FlowersMut<'a> {
    type Item = &'a mut Flower;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

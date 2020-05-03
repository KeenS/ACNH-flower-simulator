pub use board::Board;
pub use flower::{Allele, Flower, FlowerKind, Gene, Genome3, Genome4};
use rand::distributions::{Bernoulli, Distribution};
use rand::prelude::*;
use rand::seq::SliceRandom;

mod board;
mod display;
mod flower;

#[derive(Debug)]
pub struct Simulator {
    rng: ThreadRng,
    board: Board,
}

impl Simulator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rng: rand::thread_rng(),
            board: Board::new(width, height),
        }
    }

    pub fn plant_at(&mut self, position: (usize, usize), kind: FlowerKind) {
        self.board[position] = Some(Flower::sprout(kind))
    }

    fn mix(&mut self, g1: Gene, g2: Gene) -> Gene {
        *g1.choose(&mut self.rng) + *g2.choose(&mut self.rng)
    }

    fn mate3(&mut self, g1: Genome3, g2: Genome3) -> Genome3 {
        let g1 = g1.0;
        let g2 = g2.0;
        Genome3([
            self.mix(g1[0], g2[0]),
            self.mix(g1[1], g2[1]),
            self.mix(g1[2], g2[2]),
        ])
    }
    fn mate4(&mut self, g1: Genome4, g2: Genome4) -> Genome4 {
        let g1 = g1.0;
        let g2 = g2.0;

        Genome4([
            self.mix(g1[0], g2[0]),
            self.mix(g1[1], g2[1]),
            self.mix(g1[2], g2[2]),
            self.mix(g1[3], g2[3]),
        ])
    }

    fn mate(&mut self, f1: &FlowerKind, f2: &FlowerKind) -> FlowerKind {
        use FlowerKind::*;
        match (f1, f2) {
            (&Rose(g1), &Rose(g2)) => Rose(self.mate4(g1, g2)),
            (&Cosmos(g1), &Cosmos(g2)) => Cosmos(self.mate3(g1, g2)),
            (&Lily(g1), &Lily(g2)) => Lily(self.mate3(g1, g2)),
            (&Pansy(g1), &Pansy(g2)) => Pansy(self.mate3(g1, g2)),
            (&Tulip(g1), &Tulip(g2)) => Tulip(self.mate3(g1, g2)),
            (&Hyacinth(g1), &Hyacinth(g2)) => Hyacinth(self.mate3(g1, g2)),
            (&Mum(g1), &Mum(g2)) => Mum(self.mate3(g1, g2)),
            (&Windflower(g1), &Windflower(g2)) => Windflower(self.mate3(g1, g2)),
            _ => panic!("trying to mate different kind of flowers"),
        }
    }

    pub fn next_state(&mut self) {
        self.grow_all();
        self.spawn_all();
        // clear showered state
    }

    fn grow_all(&mut self) {
        for f in self.board.flowers_mut() {
            f.growup()
        }
    }

    fn spawn_all(&mut self) {
        let mut positions = self.board.flower_positions().collect::<Vec<_>>();
        positions.shuffle(&mut self.rng);
        for &pos in &positions {
            let f = self.board[pos]
                .as_ref()
                .expect("flower_positions must return positions that contain flowers");
            // TODO: calcurate it with flower state
            let spawn_rate = 0.05;
            let to_spawn = Bernoulli::new(spawn_rate).unwrap().sample(&mut self.rng);
            if !to_spawn {
                continue;
            }
            let crossable_neighbors = self
                .board
                .neighbors(pos)
                .iter()
                .filter_map(|o| o.as_ref())
                .filter(|other| f.kind().same_kind(other.kind()))
                .filter(|f| f.matured())
                // TODO: check if not crossed yet
                .map(|f| f.kind().clone())
                .collect::<Vec<_>>();
            let new_flower = match crossable_neighbors.choose(&mut self.rng) {
                // if some partner, cross them
                Some(partner) => {
                    let kind = f.kind().clone();
                    Flower::sprout(self.mate(&kind, &partner))
                }
                // if no partner, clone itself
                None => Flower::sprout(f.kind().clone()),
            };

            let empty_neighbor_positions = Board::neigbor_positions(pos)
                .iter()
                .filter_map(|&pos| match pos {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                })
                .filter(|&pos| self.board.is_in_bound(pos) && self.board[pos].is_none())
                .collect::<Vec<_>>();
            let place = match empty_neighbor_positions.choose(&mut self.rng) {
                Some(&p) => p,
                None => continue,
            };
            self.board[place] = Some(new_flower);
        }
    }
}

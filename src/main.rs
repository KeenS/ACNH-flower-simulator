use flower_simulator::{FlowerKind, Simulator};

fn main() {
    let mut s = Simulator::new(10, 10);
    s.plant_at((1, 1), FlowerKind::rose_white());
    for _ in 0..100 {
        s.next_state();
    }
    println!("{:?}", s);
}

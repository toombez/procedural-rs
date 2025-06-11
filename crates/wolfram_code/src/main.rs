use wolfram_code::{
    automaton::{WolframCodeAutomaton, WolframCodeLattice},
    rule::WolframCodeRule,
    state::WolframCodeState,
};

use toolkit::lattice::{lattice1::Lattice1Size, lattice1_point::Lattice1Point};

pub fn main() {
    let ca = WolframCodeAutomaton::new(WolframCodeRule::new(99));
    let mut l =
        WolframCodeLattice::from_states(vec![WolframCodeState::Dead], Lattice1Size::new(10));

    l.set_state(
        &Lattice1Point::new(l.size().width() as i128 / 2),
        WolframCodeState::Alive,
    );

    for _ in 0..30 {
        for x in 0..l.size().width() {
            let state = l.get_state(&Lattice1Point::new(x as i128));

            let ch = match state {
                WolframCodeState::Alive => '█',
                WolframCodeState::Dead => '.',
            };

            print!("{ch}");
        }
        ca.step_wrapper(&mut l);
        println!()
    }
}

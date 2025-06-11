use game_of_life::{
    automaton::{GameOfLifeAutomaton, GameOfLifeLattice},
    rule::GameOfLifeRule,
    state::GameOfLifeState,
};
use toolkit::{
    lattice::{lattice2::Lattice2Size, lattice2_point::Lattice2Point},
    types::BoundaryHandling,
};

pub fn main() {
    let ca = GameOfLifeAutomaton::new(GameOfLifeRule::default());

    let mut lattice2 = {
        let mut states = GameOfLifeLattice::from_states(
            vec![GameOfLifeState::Dead; 30 * 30],
            Lattice2Size::new(30, 30),
        );

        states.set_state(&Lattice2Point::new(4, 0), GameOfLifeState::Alive);
        states.set_state(&Lattice2Point::new(4, 1), GameOfLifeState::Alive);
        states.set_state(&Lattice2Point::new(4, 2), GameOfLifeState::Alive);
        states.set_state(&Lattice2Point::new(3, 2), GameOfLifeState::Alive);
        states.set_state(&Lattice2Point::new(2, 1), GameOfLifeState::Alive);

        states.set_boundary_handling(BoundaryHandling::Wrap);

        states
    };

    let size = lattice2.size();

    for _ in 0..100 {
        let mut s = String::new();

        for y in 0..size.height() {
            for x in 0..size.width() {
                let state = lattice2.get_state(&Lattice2Point::new(x as i128, y as i128));

                let ch = match state {
                    GameOfLifeState::Alive => "█",
                    GameOfLifeState::Dead => ".",
                };

                s += ch
            }
            s.push('\n');
        }

        print!("{s}");

        ca.step_wrapper(&mut lattice2);
    }
}

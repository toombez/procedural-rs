use game_of_life::{
    automaton::{GameOfLifeAutomaton, GameOfLifeLattice, GameOfLifeLatticePoint, GameOfLifeLatticeSize},
    rule::GameOfLifeRule,
    state::GameOfLifeState,
};
use toolkit::{
    types::BoundaryHandling,
};

pub fn main() {
    let ca = GameOfLifeAutomaton::new(GameOfLifeRule::default());

    let mut lattice2 = {
        let mut states = GameOfLifeLattice::from_states(
            vec![GameOfLifeState::Dead; 30 * 30],
            GameOfLifeLatticeSize::new([30, 30].to_vec()),
        );

        states.set_state(&GameOfLifeLatticePoint::new([4, 0].to_vec()), GameOfLifeState::Alive);
        states.set_state(&GameOfLifeLatticePoint::new([4, 1].to_vec()), GameOfLifeState::Alive);
        states.set_state(&GameOfLifeLatticePoint::new([4, 2].to_vec()), GameOfLifeState::Alive);
        states.set_state(&GameOfLifeLatticePoint::new([3, 2].to_vec()), GameOfLifeState::Alive);
        states.set_state(&GameOfLifeLatticePoint::new([2, 1].to_vec()), GameOfLifeState::Alive);

        states.set_boundary_handling(BoundaryHandling::Wrap);

        states
    };

    let size = lattice2.size();

    for _ in 0..100 {
        let mut s = String::new();

        for y in 0..size.get(1).unwrap() {
            for x in 0..size.get(0).unwrap() {
                let state = lattice2.get_state(&GameOfLifeLatticePoint::new([x as isize, y as isize].to_vec()));

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

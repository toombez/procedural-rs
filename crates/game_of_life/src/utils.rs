use crate::state::GameOfLifeState;
use toolkit::prelude::*;

pub fn count_alive(neighborhood: &impl Neighborhood<State = GameOfLifeState>) -> usize {
    neighborhood
        .iter_states()
        .filter(|state| **state == GameOfLifeState::Alive)
        .count()
}

pub fn count_dead(neighborhood: &impl Neighborhood<State = GameOfLifeState>) -> usize {
    neighborhood
        .iter_states()
        .filter(|state| **state == GameOfLifeState::Dead)
        .count()
}

use ddo::Dominance;
use smallbitset::Set256;

use crate::state::{State, Position, ElapsedTime};

#[derive(Hash, PartialEq, Eq)]
pub struct Key {
    position: u16,
    must_visit: Set256,
}

pub struct TsptwDominance;

impl Default for TsptwDominance {
    fn default() -> Self {
        Self {}
    }
}

impl Dominance for TsptwDominance {
    type State = State;
    type Key = Key;

    fn get_key(&self, state: &Self::State) -> Option<Self::Key> {
        if let ElapsedTime::FuzzyAmount {earliest: _, latest: _} = &state.elapsed {
            None
        } else if let Some(_maybe) = &state.maybe_visit {
            None
        } else {
            match state.position {
                Position::Node(position) => Some(Key {
                    position,
                    must_visit: state.must_visit
                }),
                Position::Virtual(_) => None,
            }
        }
    }

    fn nb_value_dimensions(&self) -> usize {
        1
    }

    fn get_value_at(&self, state: &Self::State, _: usize) -> isize {
        - (state.elapsed.latest() as isize)
    }
}
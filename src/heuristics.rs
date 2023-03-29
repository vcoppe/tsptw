// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This module provides the implementation for some heuristics that may be used
//! to improve the behavior of the branch-and-bound-MDD solver for travelling
//! salesman problem with time windows.

use ddo::{StateRanking, WidthHeuristic, SubProblem};

use crate::state::State;

#[derive(Debug, Copy, Clone)]
pub struct TsptwRanking;

impl StateRanking for TsptwRanking {
    type State = State;

    fn compare(&self, sa: &Self::State, sb: &Self::State) -> std::cmp::Ordering {
        sa.depth.cmp(&sb.depth)
    }
}

pub struct TsptwWidth {
    nb_vars: usize,
    factor: usize,
}
impl TsptwWidth {
    pub fn new(nb_vars: usize, factor: usize) -> TsptwWidth {
        TsptwWidth { nb_vars, factor }
    }
}
impl WidthHeuristic<State> for TsptwWidth {
    fn max_width(&self, state: &SubProblem<State>) -> usize {
        self.nb_vars * (state.depth as usize + 1) * self.factor
    }
}
use std::{fs::File, path::PathBuf, time::Duration};

use ddo::{MaxUB, NoDupFringe, ParBarrierSolverFc, Problem, Solver, TimeBudget};
use tsptw::{
    heuristics::{TsptwRanking, TsptwWidth},
    instance::TsptwInstance,
    model::Tsptw,
    relax::TsptwRelax,
};

fn locate(id: &str) -> PathBuf {
    PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources/")
        .join(id)
}

const TIMEOUT: u64 = 95;

pub fn solve(instance: &str, width: Option<usize>, threads: Option<usize>) -> f32 {
    let file = File::open(locate(instance)).expect("file not found");
    let inst = TsptwInstance::from(file);
    let pb = Tsptw::new(inst);
    let mut fringe = NoDupFringe::new(MaxUB::new(&TsptwRanking));
    let relax = TsptwRelax::new(&pb);
    let width = TsptwWidth::new(pb.nb_variables(), width.unwrap_or(1));
    let cutoff = TimeBudget::new(Duration::from_secs(TIMEOUT));
    let mut solver = ParBarrierSolverFc::custom(
        &pb,
        &relax,
        &TsptwRanking,
        &width,
        &cutoff,
        &mut fringe,
        threads.unwrap_or(num_cpus::get()),
    );
    let outcome = solver.maximize();

    outcome
        .best_value
        .map(|v| -(v as f32) / 10000.0)
        .unwrap_or(-1.0)

    /*if outcome.is_exact {
        outcome
            .best_value
            .map(|v| -(v as f32) / 10000.0)
            .unwrap_or(-1.0)
    } else {
        -1.0
    }*/
}

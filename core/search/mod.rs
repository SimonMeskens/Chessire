use crate::evaluation::{Evaluation, Evaluator};
use chess::{Board, Color};

// Ordered from simple to complex
pub mod negamax;

pub mod alphabeta;

pub mod memoized;

pub mod negascout;

pub mod mtd;

pub trait Search {
    fn search<E: Evaluator>(board: &Board, depth: usize, player: Color) -> Evaluation;
}

pub mod classic;
pub mod count;

use chess::{Board, BoardStatus, Color};
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evaluation {
    Worst,
    Score(i64),
    Best,
}

pub fn evaluate_board_status(board: &Board) -> Option<Evaluation> {
    match board.status() {
        BoardStatus::Checkmate => Some(board.side_to_move() * Evaluation::Worst),
        BoardStatus::Stalemate => Some(Evaluation::Score(0)),
        BoardStatus::Ongoing => None,
    }
}

pub trait Evaluator {
    fn evaluate(board: &Board) -> Evaluation;
}

impl ops::Neg for Evaluation {
    type Output = Evaluation;
    #[inline]
    fn neg(self) -> Evaluation {
        match self {
            Evaluation::Worst => Evaluation::Best,
            Evaluation::Score(s) => Evaluation::Score(-s),
            Evaluation::Best => Evaluation::Worst,
        }
    }
}

impl ops::Mul<Evaluation> for Color {
    type Output = Evaluation;
    #[inline]
    fn mul(self, e: Evaluation) -> Evaluation {
        match self {
            Color::White => e,
            Color::Black => -e,
        }
    }
}

pub trait Signed {
    fn sign(&self) -> i8;
}

impl Signed for Color {
    fn sign(&self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

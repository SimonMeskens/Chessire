use super::{evaluate_board_status, Evaluation, Evaluator};
use chess::{Board, Color};

pub enum CountEvaluator {}

impl Evaluator for CountEvaluator {
    fn evaluate(board: &Board) -> Evaluation {
        match evaluate_board_status(board) {
            Some(e) => e,
            None => Evaluation::Score(
                board.color_combined(Color::White).popcnt() as i64
                    - board.color_combined(Color::Black).popcnt() as i64,
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CountEvaluator;
    use super::{Evaluation, Evaluator};
    use chess::Board;

    #[test]
    fn test_positions() {
        let positions = [
            ("8/1p4K1/6Rr/4kp2/3p3p/1Q4P1/2p1b1N1/b2n3q w - - 0 1", -6),
            ("1k6/2p5/P1Q5/PP6/N2K4/3p4/1p1BBpP1/4R1n1 w - - 0 1", 4),
            ("1K6/6nR/P3k3/P1B1p1p1/5r2/1bp5/Pn5P/3R4 w - - 0 1", 0),
            ("7r/1K6/P5pN/4p1p1/6R1/Ppp2PQ1/P1k5/1n6 w - - 0 1", 0),
            ("8/2P3p1/2P1Pq2/2Kpp3/6rB/5N1P/k2P4/5R1n w - - 0 1", 2),
        ];

        for (position, evaluation) in positions.iter() {
            assert_eq!(
                CountEvaluator::evaluate(&position.parse::<Board>().unwrap()),
                Evaluation::Score(*evaluation as i64)
            )
        }
    }

    #[test]
    fn test_finished_positions() {
        let positions = [
            (
                "3b1q1q/1N2PRQ1/rR3KBr/B4PP1/2Pk1r1b/1P2P1N1/2P2P2/8 b - -",
                Evaluation::Best,
            ),
            ("7k/8/5b2/8/8/n7/PP6/K7 w - -", Evaluation::Score(0)),
        ];

        for (position, evaluation) in positions.iter() {
            assert_eq!(
                CountEvaluator::evaluate(&position.parse::<Board>().unwrap()),
                *evaluation
            )
        }
    }
}

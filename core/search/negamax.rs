/*

Source: https://www.chessprogramming.org/Negamax

int negaMax( int depth ) {
    if ( depth == 0 ) return evaluate();
    int max = -oo;
    for ( all moves)  {
        score = -negaMax( depth - 1 );
        if( score > max )
            max = score;
    }
    return max;
}

*/

use super::Search;
use crate::evaluation::{Evaluation, Evaluator};
use chess::{Board, BoardStatus, Color};

pub enum NegaMax {}

impl Search for NegaMax {
    fn search<E: Evaluator>(board: &Board, depth: usize, player: Color) -> Evaluation {
        if depth == 0 || board.status() != BoardStatus::Ongoing {
            return player * E::evaluate(board);
        }

        chess::MoveGen::new_legal(&board)
            .map(|m| -NegaMax::search::<E>(&board.make_move_new(m), depth - 1, !player))
            .max()
            .unwrap_or(Evaluation::Worst)
    }
}

#[cfg(test)]
mod test {
    use super::NegaMax;
    use crate::{
        evaluation::{count::CountEvaluator, Evaluation, Evaluator},
        search::Search,
    };
    use chess::{Board, Color};

    #[test]
    fn capture() {
        let positions = [
            "1K6/6nR/P3k3/P1B1p1p1/5r2/1bp5/Pn5P/3R4 w - - 0 1",
            "7r/1K6/P5pN/4p1p1/6R1/Ppp2PQ1/P1k5/1n6 w - - 0 1",
            "8/2P3p1/2P1Pq2/2Kpp3/6rB/5N1P/k2P4/5R1n w - - 0 1",
        ];

        for position in positions.iter() {
            let board = position.parse::<Board>().unwrap();

            let evaluation = CountEvaluator::evaluate(&board);

            assert_eq!(
                NegaMax::search::<CountEvaluator>(&board, 1, Color::White),
                match evaluation {
                    Evaluation::Score(s) => Evaluation::Score(s + 1),
                    _ => evaluation,
                }
            );
        }
    }

    #[test]
    fn mate_in_one() {
        let positions = [
            (
                "8/1p4K1/6Rr/4kp2/3p3p/1Q4P1/2p1b1N1/b2n3q w - - 0 1",
                Evaluation::Best,
            ),
            (
                "1k6/2p5/P1Q5/PP6/N2K4/3p4/1p1BBpP1/4R1n1 w - - 0 1",
                Evaluation::Best,
            ),
        ];

        for (position, expectation) in positions.iter() {
            let board = position.parse::<Board>().unwrap();

            assert_eq!(
                NegaMax::search::<CountEvaluator>(&board, 1, Color::White),
                *expectation
            );
        }
    }

    #[test]
    fn mate_in_two() {
        let positions = [(
            "8/6K1/1p1B1RB1/8/2Q5/2n1kP1N/3b4/4n3 w - - 0 1",
            Evaluation::Best,
        )];

        for (position, expectation) in positions.iter() {
            let board = position.parse::<Board>().unwrap();

            assert_eq!(
                NegaMax::search::<CountEvaluator>(&board, 3, Color::White),
                *expectation
            );
        }
    }
}

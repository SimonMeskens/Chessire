use super::{evaluate_board_status, Evaluation};
use chess::{Board, Color, Piece};

#[inline]
fn piece_value(piece: &Piece) -> i64 {
    match piece {
        Piece::King => 0,
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
    }
}

pub enum ValueEvaluator {}

impl super::Evaluator for ValueEvaluator {
    fn evaluate(board: &Board) -> Evaluation {
        match evaluate_board_status(board) {
            Some(e) => e,
            None => {
                let mut score = 0;

                for piece in [
                    Piece::Pawn,
                    Piece::Knight,
                    Piece::Bishop,
                    Piece::Rook,
                    Piece::Queen,
                ]
                .iter()
                {
                    if *piece == Piece::King {
                        continue;
                    }

                    let value = piece_value(piece);
                    let bitboard = board.pieces(*piece);

                    let piece_advantage = (bitboard & board.color_combined(Color::White)).popcnt()
                        as i64
                        - bitboard.popcnt() as i64;

                    score += value * piece_advantage;
                }

                Evaluation::Score(score)
            }
        }
    }
}

use crate::util::log;
use chessire_core::{
    evaluation::{classic::ValueEvaluator, Evaluation},
    search::{negamax::NegaMax, Search},
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

fn to_move(mov: chess::ChessMove) -> js_sys::Array {
    let array = js_sys::Array::new();

    array.push(&JsValue::from(mov.get_source().to_string()));
    array.push(&JsValue::from(mov.get_dest().to_string()));

    array
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct strategies {}

#[wasm_bindgen]
impl strategies {
    pub fn random(game: &crate::game::Game) -> js_sys::Array {
        let board = game.game().current_position();

        let moves: Vec<_> = chess::MoveGen::new_legal(&board).collect();

        let random = (crate::util::random() * moves.len() as f64) as usize;

        to_move(moves[random])
    }

    pub fn simple(game: &crate::game::Game) -> js_sys::Array {
        let depth = 3;

        let board = game.game().current_position();
        log(&format!("{}", board));

        let moves = chess::MoveGen::new_legal(&board);
        let mut candidate_moves = Vec::new();
        let mut best = Evaluation::Worst;

        for mov in moves {
            let value = NegaMax::search::<ValueEvaluator>(
                &board.make_move_new(mov),
                depth,
                board.side_to_move(),
            );

            if value == best {
                candidate_moves.push(mov);
            } else if value > best {
                candidate_moves.clear();
                candidate_moves.push(mov);
                best = value;
            }
        }

        if candidate_moves.is_empty() {
            js_sys::Array::new()
        } else {
            let range = (crate::util::random() * candidate_moves.len() as f64) as usize;
            to_move(candidate_moves[range])
        }
    }
}

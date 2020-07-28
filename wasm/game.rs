use std::str::FromStr;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct Game(chess::Game);

impl Game {
    pub fn game(&self) -> &chess::Game {
        &self.0
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: Option<String>) -> Game {
        match fen {
            None => Game(chess::Game::new()),
            Some(fen) => Game(chess::Game::from_str(&fen).unwrap()),
        }
    }

    pub fn moves(&self) -> js_sys::Map {
        let board = self.0.current_position();

        let iterable = chess::MoveGen::new_legal(&board);

        let moves = js_sys::Map::new();

        for mov in iterable {
            let (from, to) = (
                &JsValue::from(mov.get_source().to_string()),
                &JsValue::from(mov.get_dest().to_string()),
            );

            if moves.get(from).is_undefined() {
                let _ = moves.set(from, &js_sys::Array::new());
            }

            moves
                .get(from)
                .dyn_into::<js_sys::Array>()
                .unwrap()
                .push(to);
        }

        moves
    }

    #[wasm_bindgen(js_name = move)]
    pub fn make_move(&mut self, from: &str, to: &str) -> String {
        let mov = chess::ChessMove::new(
            chess::Square::from_string(from.to_owned()).unwrap(),
            chess::Square::from_string(to.to_owned()).unwrap(),
            None,
        );

        self.0.make_move(mov);

        self.0.current_position().to_string()
    }

    pub fn side(&self) -> String {
        match self.0.side_to_move() {
            chess::Color::White => String::from("white"),
            chess::Color::Black => String::from("black"),
        }
    }

    pub fn result(&self) -> JsValue {
        match self.0.result() {
            None => JsValue::FALSE,
            Some(result) => JsValue::from(format!("{:?}", result)),
        }
    }
}

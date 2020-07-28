import init, * as engine from "./chessire_wasm.js";
import chessground from "https://dev.jspm.io/chessground";

const { Chessground } = chessground;

const initial = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

init().then(() => {
    const position = initial;
    const game = new engine.Game(position);

    const strategy = (game) => engine.strategies.simple(game);

    const board = Chessground(document.getElementById("board"), {
        fen: position,
        movable: {
            color: "white",
            free: false,
            dests: game.moves(),
        },
        draggable: {
            showGhost: true,
        },
    });

    const move = (from, to, meta) => {
        game.move(from, to);

        board.set({
            turnColor: game.side(),
            movable: {
                color: null,
                dests: new Map(),
            },
        });

        if (!game.result()) {
            setTimeout(() => {
                let reply = strategy(game);

                game.move(...reply);
                board.move(...reply);

                board.set({
                    turnColor: game.side(),
                    movable: {
                        color: game.side(),
                        dests: game.moves(),
                    },
                });
            }, 200);
        }
    };

    board.set({
        movable: { events: { after: move } },
    });
});

# Chessire
A chess engine in Rust/WASM for fun and learning.

# WASM

To compile this project to WASM, you need:

* Node/NPM
* Wasm-Pack

First, install the dependencies with

    npm install

Then, there are a few scripts you can run:

* `npm run build`: builds the project
* `npm run watch`: watch the source and rebuild when it has changed
* `npm run serve`: serve the output as a web server
* `npm run dev`: run the web server and when the code changes, rebuild it and live reload the web page
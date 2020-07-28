import { exec } from "child_process";
import gulp from "gulp";
import connect from "gulp-connect";
import del from "del";

const { series, parallel, src, dest } = gulp;

const folders = ["core", "wasm", "interface"];

const clean = () => del("./dist");

const wasm = () => exec("wasm-pack build --target web --out-dir '../dist' wasm");

const bundle = () => src("./interface/**/*").pipe(dest("./dist")).pipe(connect.reload());

const rebuild = series(wasm, bundle, () => del("./dist/package.json"));
export const build = series(clean, rebuild);

export const watch = () =>
    gulp.watch(
        folders.map((folder) => `${folder}/**/*`),
        rebuild
    );

export const serve = () => {
    connect.server({
        root: "./dist",
        livereload: true,
    });
};

export const dev = series(build, parallel(watch, serve));

export default build;

wasm-pack build --release --target web

cp ./pkg/grid_render.js ../dist/grid_render.js
cp ./pkg/grid_render_bg.wasm ../dist/grid_render_bg.wasm
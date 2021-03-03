CRATE_NAME=statistics_helper # assume crate name is the same as the folder name

# Clear output from old stuff:
rm -f docs/${CRATE_NAME}_bg.wasm

echo "Building rust…"
BUILD=release
cargo build --release --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME}.wasm"
wasm-bindgen "${PWD}./target/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}" \
  --out-dir docs --no-modules --no-typescript

echo "Finished: docs/${CRATE_NAME}.wasm"
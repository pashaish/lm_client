FLAGS="--fix --allow-staged --all-targets --all-features --no-deps"
POST_FLAGS="--"

__CARGO_FIX_YOLO=1 cargo clippy $FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p services $POST_FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p database $POST_FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p api $POST_FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p utils $POST_FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p types $POST_FLAGS && \
__CARGO_FIX_YOLO=1 cargo clippy --allow-dirty $FLAGS --lib -p framework $POST_FLAGS

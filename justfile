set dotenv-load

alias r := run-server

default:
    @just --list

[working-directory: 'frontend']
_fmt-frontend:
    biome format --write
    
fmt: _fmt-frontend
    echo "Formatting"
    cargo fmt

[working-directory: 'frontend']
lint:
    echo "Linting"
    biome lint --write

build:
    just tailwind
    just tsc
    just fmt
    cargo build

dev-db:
    docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:18

test:
    # Unit Test (watch)
    cargo test -- --nocapture

run-server:
    cargo run -p web-server

run-server-release:
    cargo run -p web-server --release

run-server-hot-reload:
    cargo run -p web-server --bin web-server-hot-reload --features hot_reload

build-run-server: build
    cargo run -p web-server

watch-build-run-server: 
    watchexec -q -c \
        -w crates/services/web-server/src/ \
        -w crates/libs/ \
        -w frontend/templates/ \
        -w .cargo/ \
        -r --stop-signal SIGKILL "just build-run-server"
    

    
watch:
    watchexec -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -r --stop-signal SIGKILL "cargo run -p web-server"

[working-directory: 'frontend']
tailwind:
    bunx @tailwindcss/cli -i ./input.css -o ./static/css/build/tailwind.css

tailwind-watch:
    watchexec -q -c -w frontend/templates -w frontend/input.css -r --stop-signal SIGKILL "just tailwind"

[working-directory: 'frontend']
tsc:
    bun run build

tsc-watch:
    watchexec -q -c -w frontend/src/ -w frontend/tsconfig.json -r --stop-signal SIGKILL "just tsc"

watch-example:
    watchexec -q -c -w crates/services/web-server/examples/ -r --stop-signal SIGKILL "cargo run -p web-server --example quick_dev"

watch-test:
    watchexec -q "cargo test -- --nocapture"

watch-test-specific:
    # Specific test with filter.
    watchexec -q -c "cargo test -p lib-web test_create -- --nocapture"
    # watchexec -q -c -x "cargo test -p lib-web model::task::tests::test_create -- --nocapture"

run-example:
    # cargo run -p web-server --example quick_dev
    cargo run -p web-server --example register

run-gen-key:
    cargo run -p gen-key

run-gen-pass:
    cargo run -p gen-pass


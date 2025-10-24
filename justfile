set dotenv-load

alias r := run-server

default:
    @just --list

fmt:
    echo "Formatting"
    cargo fmt
    cd frontend && biome format --write

lint:
    echo "Linting"
    cd frontend && biome lint --write

dev-db:
    docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:18

test:
    # Unit Test (watch)
    cargo test -- --nocapture

[working-directory: 'frontend']
tailwind:
    bunx @tailwindcss/cli -i ./input.css -o ./static/css/build/tailwind.css

run-server:
    cargo run -p web-server

build-run-server: tailwind 
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


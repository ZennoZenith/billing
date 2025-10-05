default:
  @just --list

fmt:
    dprint fmt *

dev-db:
    docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:18

test:
    # Unit Test (watch)
    cargo test -- --nocapture

    
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

run-server:
    cargo run -p web-server

run-example:
    # cargo run -p web-server --example quick_dev
    cargo run -p web-server --example register

run-gen-key:
    cargo run -p gen-key

run-gen-pass:
    cargo run -p gen-pass


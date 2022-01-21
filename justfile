# default recipe to display help information
default:
  @just --list

init:
  git config core.hooksPath .githooks

format:
  cargo fmt

watch:
  cargo watch -c -x check

run:
  cargo run

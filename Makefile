build:
	@cargo build --release

start:
	@[[ -x ./target/release/brain_games ]] && ./target/release/brain_games

format:
	@rustfmt src/**/*

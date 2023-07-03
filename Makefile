build:
	@cargo build --release

start:
	@cargo run --release

format:
	@rustfmt src/*

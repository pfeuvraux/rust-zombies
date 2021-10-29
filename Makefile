build:
	cargo build

release:
	cargo build --release

run_debug_dryrun:
	./target/debug/rust-zombies

run_debug:
	./target/debug/rust-zombies -k

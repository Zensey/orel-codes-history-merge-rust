test:
	cargo test --release -- --nocapture

run:
	cargo run --release

docker-test:
	docker build .

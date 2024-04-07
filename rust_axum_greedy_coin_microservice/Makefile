format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build:
	docker build -t greedy_coin_change .

rundocker:
	docker run -dp 3000:3000 greedy_coin_change

run:
	cargo run

clean:
	cargo clean

all: format lint test run
install:
	#unfortunately python dependencies needed
	pip3 install cargo-lambda
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run:
	cargo run 

release:
	cargo lambda build --release --target x86_64-unknown-linux-gnu.2.17
	# cargo lambda build --release

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"num": 27}' \
  		--output-format json \
  		roman-lambda

all: format lint test run
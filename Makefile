docker:
	@echo "Building image"
	docker build . -t rush

run: docker
	docker run -it rush

build:
	cargo build --release
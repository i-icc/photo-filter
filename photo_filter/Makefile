build-wasm:
	wasm-pack build --target web

clean-build:
	cargo clean
	cargo build

test:
	cargo run

update-client-package: build-wasm
	rm pkg/.gitignore
	cp -r pkg ../photo-filter-client/src/
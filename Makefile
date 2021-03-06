.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all

.PHONY: pallet_test
pallet_test:
	./scripts/pallet_test.sh

.PHONY: run
run:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-06-x86_64-apple-darwin cargo +nightly-2020-10-06 run --release -- --dev --tmp

.PHONY: build
build:
	./scripts/build.sh

.PHONY: dev
dev:
	./scripts/build_dev.sh


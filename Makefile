coverage_report: data/output/coverage/index.html

open_coverage: data/output/coverage/index.html
	open $(<)

.PHONY: force
force:

data/output/lcov.info: force
	export CARGO_INCREMENTAL=0; \
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"; \
	cargo clean; \
	cargo build --verbose $$CARGO_OPTIONS; \
	cargo test --verbose $$CARGO_OPTIONS; \
	grcov target/debug -t lcov --llvm --branch --ignore-not-existing -o $(@);

bar: data/output/coverage/index.html

data/output/coverage/index.html: data/output/lcov.info
	genhtml -o $(dir $(@)) --show-details --highlight --ignore-errors source --legend $(<)
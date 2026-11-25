.PHONY: lint test clean

CRATES := -p ast-grep -p ast-grep-core -p ast-grep-config -p ast-grep-language -p ast-grep-lsp

lint:
	cargo clippy --locked $(CRATES) --all-targets -- --deny warnings --allow unused-must-use --allow clippy::needless-bool

test:
	cargo test --locked $(CRATES)

clean:
	@if test -L ./target || { test -e ./target && ! test -d ./target; }; then \
	  printf '%s\n' 'make clean: ./target must be a local build directory' >&2; exit 1; \
	fi
	rm -rf -- ./target

.PHONY: setup
setup:
	rustup target add thumbv7em-none-eabihf

.PHONY: examples
examples:
	(cd examples/flash_led_lptmr; cargo build)

check_master:
	@echo "check if on master"
	git branch | grep \* | cut -d ' ' -f2 | grep -q master

check_tag:
	@echo "check if HEAD is tagged"
	git describe --tags --exact-match HEAD

.PHONY: publish
publish: check_master check_tag
	cargo publish

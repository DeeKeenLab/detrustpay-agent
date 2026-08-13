.DEFAULT_GOAL := help

PROGRAM_MANIFEST := program/programs/trustpay/Cargo.toml
PROGRAM_OUTPUT := program/target/deploy

.PHONY: help program-check program-test program-build repo-status

help: ## Show available commands.
	@awk 'BEGIN { FS = ":.*## "; print "Available targets:" } /^[a-zA-Z0-9][a-zA-Z0-9_-]+:.*## / { printf "  %-24s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

program-check: ## Type-check the Anchor program for the host target.
	cargo check --manifest-path $(PROGRAM_MANIFEST)

program-test: ## Run Rust unit tests for the program crate.
	cargo test --manifest-path $(PROGRAM_MANIFEST)

program-build: ## Build the Solana SBF program artifact.
	cargo build-sbf --manifest-path $(PROGRAM_MANIFEST) --sbf-out-dir $(PROGRAM_OUTPUT)

repo-status: ## Show the concise Git state and ignored-file summary.
	git status --short
	git count-objects -vH

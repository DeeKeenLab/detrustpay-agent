.DEFAULT_GOAL := help

PROGRAM_MANIFEST := program/programs/trustpay/Cargo.toml
PROGRAM_OUTPUT := program/target/deploy

.PHONY: help install build test public-audit json-check program-install program-check program-test program-build program-lifecycle-test repo-status

help: ## Show available commands.
	@awk 'BEGIN { FS = ":.*## "; print "Available targets:" } /^[a-zA-Z0-9][a-zA-Z0-9_-]+:.*## / { printf "  %-24s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

install: ## Install agent workspace dependencies from the lockfile.
	npm ci

build: ## Compile the public agent SDK packages.
	npm run build

test: ## Run the public agent SDK tests.
	npm test

public-audit: ## Reject tracked secrets, sensitive filenames, and oversized files.
	./scripts/check-public-repo.sh

json-check: ## Parse committed JSON configuration and schema files.
	git ls-files --cached --others --exclude-standard -- '*.json' | xargs -n 1 node -e 'JSON.parse(require("fs").readFileSync(process.argv[1], "utf8"))'

program-install: ## Install the program lifecycle-test dependencies.
	cd program && yarn install --frozen-lockfile

program-check: ## Type-check the Anchor program for the host target.
	cargo check --manifest-path $(PROGRAM_MANIFEST)

program-test: ## Run Rust unit tests for the program crate.
	cargo test --manifest-path $(PROGRAM_MANIFEST)

program-build: ## Build the Solana SBF program artifact.
	cargo build-sbf --manifest-path $(PROGRAM_MANIFEST) --sbf-out-dir $(PROGRAM_OUTPUT)

program-lifecycle-test: ## Run the fresh-validator dual-order lifecycle.
	cd program && yarn test

repo-status: ## Show the concise Git state and ignored-file summary.
	git status --short
	git count-objects -vH

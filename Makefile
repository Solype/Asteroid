default: help

##@ Helpers
.PHONY: help
help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Build
.PHONY: build
build:  ## Build the project
	cargo build
	mv target/debug/AST3ROID .

.PHONY: run
run: ## Run the project
	cargo build
	mv target/debug/AST3ROID .

##@ Cleanup
.PHONY: clean
clean: ## Clean the directory
	rm -rf target
	rm AST3ROID

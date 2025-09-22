default: help

##@ Helpers
.PHONY: help
help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Auto Install
.PHONY: install
install: ## Full installation of the project
	@if ! command -v rustup &> /dev/null; then \
  		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
  	fi
	@make clean
	@make assets
	@make build

##@ Build
.PHONY: build
build:  ## Build the project
	cargo build
	mv target/debug/bevy_game .

.PHONY: run
run: ## Run the project
	cargo build
	mv target/debug/bevy_game .

##@ Cleanup
.PHONY: clean
clean: ## Clean the directory
	rm -rf target
	rm bevy_game

##@ Assets
.PHONY: assets
assets: ## Clone or update the AsteroidAssets repository into assets/
	@if [ -d assets/.git ]; then \
		cd assets && git pull origin master; \
	else \
		rm -rf assets; \
		git clone https://github.com/Mael-RABOT/AsteroidAssets.git assets; \
	fi

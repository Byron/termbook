EXE=target/debug/termbook

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

always:

$(EXE): $(shell find termbook-cli -name *.rs) $(shell find termbook -name *.rs) $(shell find . -name Cargo.toml)
	cargo build

##@ Testing

stateless-journey-tests: $(EXE) ## Run only stateless journey
	tests/book.sh $<

lint-scripts: ## run shellcheck on all shell scripts
	find . -not \( -path '*target/*' -or -path "*cargo*" \) -name '*.sh' -type f | while read -r sf; do shellcheck -x "$$sf"; done

##@ Maintenance

update-homebrew: ## wait for the current version to be ready on github and update homebrew file
	@set -ex; ./bin/update-homebrew-formula.sh $$(git tag | tail -1) ./pkg/brew/termbook.rb.in ./pkg/brew/termbook.rb

	
docs: $(EXE) ## Build the documentation with the debug binary
	PATH="$(dir $<):$$PATH" $< build doc
	
asciinema-no-upload: termbook.cast ## record the default eye-candy video to a file
	
asciinema-upload: termbook.cast ## record the intro video and upload it
	asciinema upload $<

termbook.cast: $(EXE)
	PATH="$(dir $<):$$PATH" \
	asciinema rec \
		--title 'An Introduction to termbook (https://byron.github.io/termbook)' \
		-c '$< play doc Introduction Command-Line* 2>/dev/null' \
		$@

EXE=target/debug/termbook

help:
	$(info Available Targets)
	$(info - Testing -----------------------------------------------------------------------------------------------------)
	$(info lint-scripts            | Run journey tests using a pre-built linux binary)
	$(info stateless-journey-tests | Run only stateless journey)
	$(info asciinema-no-upload     | record the default eye-candy video to a file)
	$(info asciinema-upload        | record the intro video and upload it)
	$(info docs                    | Build the documentation with the debug binary)

always:

$(EXE): $(shell find src -name *.rs)
	cargo build

stateless-journey-tests: $(EXE)
	tests/book.sh $<

lint-scripts:
	find . -not \( -path '*target/*' -or -path "*cargo*" \) -name '*.sh' -type f | while read -r sf; do shellcheck -x "$$sf"; done
	
docs: $(EXE)
	PATH="$(dir $<):$$PATH" $< build doc
	
termbook.cast: $(EXE)
	PATH="$(dir $<):$$PATH" \
	asciinema rec \
		--title 'An Introduction to termbook (https://byron.github.io/termbook)' \
		-c '$< play doc Introduction Command-Line* 2>/dev/null' \
		$@
	
asciinema-no-upload: termbook.cast
	
asciinema-upload: termbook.cast
	asciinema upload $<

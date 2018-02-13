EXE=target/debug/termbook

help:
	$(info Available Targets)
	$(info - Testing -----------------------------------------------------------------------------------------------------)
	$(info lint-scripts            | Run journey tests using a pre-built linux binary)
	$(info stateless-journey-tests | Run only stateless journey)
	$(info docs										 | Build the documentation with the debug binary)

always:

$(EXE): always
	cargo build

stateless-journey-tests: $(EXE)
	tests/book.sh $<

lint-scripts:
	find . -not \( -path '*target/*' -or -path "*cargo*" \) -name '*.sh' -type f | while read -r sf; do shellcheck -x "$$sf"; done
	
docs: $(EXE)
	PATH="$(dir $<):$$PATH" $< build doc

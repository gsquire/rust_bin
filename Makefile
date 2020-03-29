build:
	cargo +nightly build

clean:
	cargo +nightly clean
	\rm -f routes.html

docs:
	pandoc -f markdown -t html5 -o "routes.html" "routes.md"

release: docs
	gcloud builds submit --tag gcr.io/rust-bin/rustbin

test:
	cargo +nightly test

.PHONY: build docs release test

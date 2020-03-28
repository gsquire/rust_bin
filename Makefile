docs:
	pandoc -f markdown -t html5 -o "routes.html" "routes.md"

.PHONY: docs

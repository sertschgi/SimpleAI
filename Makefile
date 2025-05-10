DIOXUS := dx

all: build-web build-desktop

build-web:
	@echo "Building for WebAssembly..."
	$(DIOXUS) build --release --package simple-ai-web --platform web
build-desktop:
	@echo "Building for Desktop..."
	$(DIOXUS) build --release --package simple-ai-desktop --platform desktop

run-web:
	@echo "Running Web version..."
	$(DIOXUS) serve --package simple-ai-web --platform web

run-desktop: 
	@echo "Running Desktop version..."
	$(DIOXUS) serve --package simple-ai-desktop --platform desktop

clean:
	@echo "Cleaning the project..."
	$(DIOXUS) clean

.PHONY: all build-web build-desktop run-web run-desktop clean

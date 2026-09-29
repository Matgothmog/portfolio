.PHONY: build clean serve

build:
	python3 build.py

clean:
	rm -rf dist

serve: build
	python3 -m http.server 8000 -d dist

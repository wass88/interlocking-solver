.PHONY: setup tailwindcss start

setup:
	mkdir -p ./static/src
	wget https://unpkg.com/htmx.org@1.7.0/dist/htmx.js -O ./static/src/htmx.js
	make css

css:
	tailwindcss -i ./static/src/main.css -o ./static/dist/main.css --minify

start:
	./server.py
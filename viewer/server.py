#!/usr/bin/env python3
from flask import Flask, render_template, request, send_from_directory
from flask_assets import Bundle, Environment

from puzzles import get_puzzles, puzzles_dir

app = Flask(__name__)
assets = Environment(app)
css = Bundle("src/main.css", output="dist/main.css")
js = Bundle("src/*.js", output="dist/main.js")
assets.register("js", js)
assets.register("css", css)
css.build()
js.build()


@app.route("/")
def index():
    return render_template("index.html", puzzles=get_puzzles())


@app.route("/puzzles")
def puzzles():
    return render_template("puzzles.html", puzzles=get_puzzles())


@app.route('/puzzle-static/<path:path>')
def send_report(path):
    return send_from_directory(puzzles_dir, path)


if __name__ == "__main__":
    app.run(debug=True, port=12313)

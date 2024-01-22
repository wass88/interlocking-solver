from pathlib import Path
from re import findall

source_path = Path(__file__).resolve()
root_dir = source_path.parent.parent
puzzles_dir = root_dir / "puzzles"


def get_puzzles():
    return [
        load_puzzle(puzzle)
        for puzzle in puzzles_dir.glob("**/*.pcad")
    ]


def load_puzzle(puzzle):
    moves = read_moves(puzzle)
    return {
        "filename": puzzle.name,
        "image": f"puzzle-static/{puzzle.parent.name}/{puzzle.stem}.png",
        "dir": puzzle.parent.name,
        "moves": moves,
    }


def read_moves(puzzle):
    with open(puzzle, "r") as f:
        for line in f:
            if line.startswith("//["):
                return parse_moves(line)


def parse_moves(moves):
    return "".join(move_to_s(move) for move in findall(r"([A-Za-z]+)\((\[?(?:\d+(?:, )?)*\]?),(?:[^)]*)\)", moves))


def move_to_s(move):
    ty, pieces = move
    if ty == "Shift":
        return f">{'_'.join(pieces[1:-1].split(', '))}"
    if ty == "Remove":
        return f"!{pieces}"
    return f"???{move}"


if __name__ == "__main__":
    print(get_puzzles())

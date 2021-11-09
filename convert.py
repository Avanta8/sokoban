import itertools
import sys


def convert(text: str) -> str:
    lines = text.splitlines()
    puzzles = []
    current_puzzle = []
    for line in lines:
        if "#" in line:
            current_puzzle.append(line)
        else:
            if current_puzzle:
                puzzles.append("\n".join(current_puzzle))
                current_puzzle = []

    return "\n\n".join(puzzles)


def main():
    argv = sys.argv

    if len(argv) != 3:
        print("argv: input.txt output.txt")
        return

    input_filename = sys.argv[1]
    output_filename = sys.argv[2]

    with open(input_filename) as f:
        text = f.read()

    output = convert(text)

    with open(output_filename, "w") as f:
        f.write(output)


if __name__ == "__main__":
    main()

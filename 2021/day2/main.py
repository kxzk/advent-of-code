import sys

# aim -> 0
# down X -> increases aim by X units
# up X -> decreases aim by X units
# forward X ->
#   increases horizontal position by X units
#   increases depth by (aim * X units)


def main() -> None:
    AIM = 0
    DEPTH = 0
    HORIZONTAL = 0

    with open("src/input.txt", "r") as f:
        d = f.read()

        l = d.splitlines()

        for line in l:
            cmd, val = line.split()

            match cmd:
                case "forward":
                    HORIZONTAL += int(val)
                    DEPTH += AIM * int(val)
                case "down":
                    AIM += int(val)
                case "up":
                    AIM -= int(val)
                case _:
                    raise ValueError(f"unknown_command={cmd}")

    print(f"result={DEPTH * HORIZONTAL}")


if __name__ == "__main__":
    sys.exit(main())

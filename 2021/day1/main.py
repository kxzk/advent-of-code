import sys
from collections import deque


def main() -> None:
    with open("src/input.txt", "r") as f:
        d = f.read()

        I = 0

        first = deque(maxlen=3)
        second = deque(maxlen=3)

        is_first_iter = True

        for l in d.splitlines():
            v = int(l.strip())

            if is_first_iter:
                first.append(v)
                is_first_iter = False
                continue

            if len(first) < 3 and len(second) < 3:
                first.append(v)
                second.append(v)
            elif len(first) == 3 and len(second) == 2:
                second.append(v)

                sum_first = sum(first)
                sum_second = sum(second)

                if sum_second > sum_first:
                    I += 1

                first = second
                v1, v2 = second[1], second[2]
                second = deque([v1, v2], maxlen=3)

        print(I)


if __name__ == "__main__":
    sys.exit(main())

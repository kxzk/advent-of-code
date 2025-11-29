import sys
from pathlib import Path

# life support rating
# (oxy gen generator rating) * (CO2 scrubber rating)
#
# [oxygen generator rating]
# MOST COMMON in current bit position (keep only numbers with that bit in that position)
# if 0 and 1 are equally common, keep values with a 1 in that
#
# [CO2 scrubber rating]
# LEAST COMMON in current bit position (keep only numbers with that bit in that position)
# if 0 and 1 are equally common, keep values with a 0 in that

# for each idx which value is most/least common (if any)
# progressive filtering based on that
# until length(candidates) == 1


def filter_by_bit_criteria(numbers: list[str], keep_most_common: bool) -> str:
    candidates = numbers.copy()
    bit_position = 0

    while len(candidates) > 1:
        ones = sum(1 for n in candidates if n[bit_position] == "1")
        zeros = len(candidates) - ones

        if keep_most_common:
            target_bit = "1" if ones >= zeros else "0"
        else:
            target_bit = "0" if zeros <= ones else "1"

        candidates = [n for n in candidates if n[bit_position] == target_bit]
        bit_position += 1

    return candidates[0]


def main() -> None:
    F = Path("src/input.txt").read_text().splitlines()

    oxygen = filter_by_bit_criteria(F, keep_most_common=True)
    co2 = filter_by_bit_criteria(F, keep_most_common=False)

    oxygen_dec = int(oxygen, 2)
    co2_dec = int(co2, 2)

    print(f"result={oxygen_dec * co2_dec}")


if __name__ == "__main__":
    sys.exit(main())

import sys


label_to_hex = {
    "A": "d",
    "K": "c",
    "Q": "b",
    "J": "a",
    "T": "9",
    "9": "8",
    "8": "7",
    "7": "6",
    "6": "5",
    "5": "4",
    "4": "3",
    "3": "2",
    "2": "1",
}


def get_hand_type(s: str):
    h = {}
    for c in s:
        h[c] = s.count(c)

    if len(h.keys()) == 1:
        # Five of a kind
        return "7"
    if len(h.keys()) == 2:
        if max(h.values()) == 4:
            # Four of a kind
            return "6"
        else:
            # Full house
            return "5"
    if len(h.keys()) == 3:
        if max(h.values()) == 3:
            # Three of a kind
            return "4"
        else:
            # Two pairs
            return "3"
    if len(h.keys()) == 4:
        # One pair
        return "2"
    # High card
    return "1"


def hand_to_hex(s: str) -> int:
    hex_s = get_hand_type(s)
    for c in s:
        hex_s += label_to_hex[c]
    return int(hex_s, 16)


if __name__ == "__main__":
    hands = [line.split() for line in sys.stdin]
    hands = sorted(hands, key=lambda x: hand_to_hex(x[0]))
    score = 0
    for i, (hand, s) in enumerate(hands):
        score += (i + 1) * int(s)
        # print(f"hand: {hand}, score: {s}, multiplier: {i+1}, result: {(i+1) * int(s)}")
        # print(f"hand_type: {get_hand_type(hand)}, hex: {hex(hand_to_hex(hand))}, int: {hand_to_hex(hand)}")
    print(score)

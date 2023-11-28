MOD = 10**9 + 7
DIGITS_COUNT = 10
MOVES = [
    [4, 6],
    [6, 8],
    [7, 9],
    [4, 8],
    [0, 3, 9],
    [],
    [0, 1, 7],
    [2, 6],
    [1, 3],
    [2, 4],
]

class Solution:
    def knightDialer(self, n):
        assert(n >= 1)
        assert(n <= 5000)

        last = [1] * DIGITS_COUNT
        
        for _ in range(1,n):
            next = [0] * DIGITS_COUNT
            for this_digit in range(DIGITS_COUNT):
                for prev_digit in MOVES[this_digit]:
                    next[this_digit] += last[prev_digit]
                    next[this_digit] %= MOD
            last = next
        
        return sum(last) % MOD

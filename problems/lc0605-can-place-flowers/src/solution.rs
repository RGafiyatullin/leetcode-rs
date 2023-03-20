pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut flowerbed = flowerbed;

        for i in 0..flowerbed.len() {
            if n == 0 {
                break
            }

            let next_ok = flowerbed.get(i + 1).copied().filter(|&v| v == 1).is_none();
            let prev_ok = i
                .checked_sub(1)
                .and_then(|i| flowerbed.get(i))
                .copied()
                .filter(|&v| v == 1)
                .is_none();

            if flowerbed[i] == 0 && next_ok && prev_ok {
                n -= 1;
                flowerbed[i] = 1;
            }
        }

        n == 0
    }
}

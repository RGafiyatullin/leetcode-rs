pub struct Solution;

const SEATS_PER_GROUP: usize = 2;
const SEAT: u8 = b'S';
const MOD: usize = 1_000_000_000 + 7;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor = corridor.as_bytes();

        let mut seats = 0;
        let mut plants = 0;
        let mut acc = 1;

        for ch in corridor.iter().copied() {
            match (ch == SEAT, seats == SEATS_PER_GROUP) {
                (true, true) => {
                    acc *= plants + 1;
                    acc %= MOD;

                    plants = 0;
                    seats = 1;
                },
                (true, false) => {
                    seats += 1;
                },
                (false, true) => {
                    plants += 1;
                }
                (false, false) => {},
            }
        }

        if seats != 2 {
            acc *= 0;
        }
        
        acc as i32
    }
}

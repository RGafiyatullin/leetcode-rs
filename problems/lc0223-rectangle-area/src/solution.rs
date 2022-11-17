pub struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        use std::cmp;

        // assert!(ax1 <= ax2);
        // assert!(ay1 <= ay2);
        // assert!(bx1 <= bx2);
        // assert!(by1 <= by2);

        let area_a = (ax2 - ax1) * (ay2 - ay1);
        let area_b = (bx2 - bx1) * (by2 - by1);

        // eprintln!("
        //     A: {}:{} — {}:{} => {}
        //     B: {}:{} — {}:{} => {}
        // ",
        //     ax1, ay1, ax2, ay2, area_a,
        //     bx1, by1, bx2, by2, area_b,
        // );

        let cx2 = cmp::min(ax2, bx2);
        let cy2 = cmp::min(ay2, by2);

        let cx1 = cmp::max(ax1, bx1);
        let cy1 = cmp::max(ay1, by1);

        let cw = cmp::max(cx2 - cx1, 0);
        let ch = cmp::max(cy2 - cy1, 0);

        let area_c = cw * ch;

        // eprintln!("
        //     C: {}:{} — {}:{} => {}
        // ",
        //     cx1, cy1, cx2, cy2, area_c,
        // );

        area_a + area_b - area_c
    }
}

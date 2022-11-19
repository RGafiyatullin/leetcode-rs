pub struct Solution;

type Point = Vec<i32>;

impl Solution {
    // Dear Leetcode, if you'd run the test cases using `--release`, I wouldn't have to manually
    // inline everything like an idiot :(
    #[inline(always)]
    pub fn outer_trees(mut points: Vec<Point>) -> Vec<Point> {
        if let Some((start_idx, start_p)) =
            points.iter_mut().enumerate().min_by_key(|(_idx, p)| (p[0], p[1]))
        {
            let start_p = std::mem::take(start_p);

            let mut infos = build_infos(start_idx, &start_p, points.iter());
            sort_infos(&mut infos);

            // eprintln!("start: [{}] => {:?}", start_idx, start_p);
            // eprintln!("points: {:?}", points);
            // eprintln!("infos: {:?}", infos.iter().map(|i| &points[i.idx]).collect::<Vec<_>>());

            let mut out = Vec::with_capacity(points.len());
            out.push(start_p);

            for info in infos {
                let p = std::mem::take(&mut points[info.idx]);

                push(&mut out, p);
            }

            out
        } else {
            Default::default()
        }
    }
}

#[derive(Debug)]
struct Info {
    idx: usize,
    angle: (i64, i64),
    distance: i64,
}

#[inline(always)]
fn build_infos<'a>(
    start_idx: usize,
    start_p: &[i32],
    points: impl IntoIterator<Item = &'a Point>,
) -> Vec<Info> {
    let (start_x, start_y) = (start_p[0], start_p[1]);
    points
        .into_iter()
        .enumerate()
        .filter(|&(idx, _)| idx != start_idx)
        .map(|(idx, p)| {
            let (x, y) = (p[0] - start_x, p[1] - start_y);
            let x = x as i64;
            let y = y as i64;
            Info { idx, angle: (y, x), distance: x * x + y * y }
        })
        .collect()
}

#[inline(always)]
fn sort_infos(infos: &mut [Info]) {
    infos.as_mut().sort_unstable_by(|this, that| {
        let this_angle = this.angle.0 * that.angle.1;
        let that_angle = that.angle.0 * this.angle.1;

        (this_angle, this.distance).cmp(&(that_angle, that.distance))
    });

    if let Some(last) = infos.last() {
        let angle = last.angle;
        let mut idx = infos.len() - 1;

        for i in (0..infos.len()).rev() {
            let a = infos[i].angle;
            if a.0 * angle.1 == angle.0 * a.1 {
                idx = i;
            } else {
                break
            }
        }

        infos[idx..].reverse();
    }
}

#[inline(always)]
fn push(out: &mut Vec<Point>, candidate: Vec<i32>) {
    out.push(candidate);
    // eprintln!("PUSH {:?}", candidate);

    while !curves_ccw(out) {
        rm_second_to_last(out);
    }
}

#[inline(always)]
fn curves_ccw(out: &mut Vec<Point>) -> bool {
    // eprintln!("EVAL");
    if out.len() <= 2 {
        // eprintln!("len <= 2 — OK");
        true
    } else {
        let last = &out[(out.len() - 3)..];
        let p1 = &last[0];
        let p2 = &last[1];
        let p3 = &last[2];

        let v1 = (p2[1] - p1[1], p2[0] - p1[0]);
        let v2 = (p3[1] - p2[1], p3[0] - p2[0]);

        let a1 = v1.0 * v2.1;
        let a2 = v2.0 * v1.1;

        // eprintln!(
        //     "{}:{}->{}:{}->{}:{} | {:?}->{:?} | {}->{} | OK: {}",
        //     p1[0],
        //     p1[1],
        //     p2[0],
        //     p2[1],
        //     p3[0],
        //     p3[1],
        //     v1,
        //     v2,
        //     a1,
        //     a2,
        //     a2 >= a1
        // );

        a2 >= a1
    }
}

#[inline(always)]
fn rm_second_to_last(out: &mut Vec<Point>) {
    // eprintln!("POP");
    let last = out.pop().expect("last missing");
    let _ = out.pop().expect("second to last missing");
    out.push(last);
}

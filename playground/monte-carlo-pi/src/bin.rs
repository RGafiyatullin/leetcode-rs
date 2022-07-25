use monte_carlo_pi::Calculator;

fn main() {
    let mut calc = Calculator::new();
    let mut prev_diff = 1.0;

    for i in 1..usize::MAX {
        calc.tick();
        let pi = calc.pi();

        if i % 100 == 0 {
            let diff = (pi - std::f64::consts::PI).abs() / std::f64::consts::PI;

            if diff < prev_diff {
                prev_diff = diff;
            } else {
                eprintln!("[DONE] iterations: {}, diff: {}", i, diff);
                break;
            }

            eprintln!("iterations: {}, diff: {}", i, diff);
        }

    }
}

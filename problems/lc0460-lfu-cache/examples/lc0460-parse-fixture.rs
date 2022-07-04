use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let line2 = lines.next().unwrap().unwrap();

    let commands = serde_json::from_str::<Vec<String>>(line1.as_str()).unwrap();
    let args = serde_json::from_str::<Vec<Vec<i32>>>(line2.as_str()).unwrap();

    commands
        .into_iter()
        .zip(args)
        .map(|(c, args)| {
            let mut s = c.replace("LFUCache", "init");
            for a in args {
                s.push_str(" ");
                s.push_str(a.to_string().as_str());
            }
            s
        })
        .for_each(|s| println!("{}", s));
}

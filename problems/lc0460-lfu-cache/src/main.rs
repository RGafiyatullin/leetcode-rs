use std::io::BufRead;
use std::str::FromStr;

mod solution_2;
use solution_2::LFUCache;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy)]
enum Command {
    Init(i32),
    Get(i32),
    Put(i32, i32),
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter.next().ok_or("empty command")?;
        let arg = iter.next().ok_or("empty argument")?;
        let arg = arg.parse::<i32>().map_err(|e| e.to_string())?;
        match command {
            "init" => Ok(Command::Init(arg)),
            "get" => Ok(Command::Get(arg)),
            "put" => Ok(Command::Put(
                arg,
                iter.next()
                    .map(ToOwned::to_owned)
                    .unwrap_or(arg.to_string())
                    .parse::<i32>()
                    .map_err(|e| e.to_string())?,
            )),
            _ => Err("invalid command".to_string()),
        }
    }
}

impl Command {
    fn apply(&self, cache: &mut LFUCache) -> Option<i32> {
        match *self {
            Command::Init(capacity) => {
                *cache = LFUCache::new(capacity);
                None
            },
            Command::Get(key) => Some(cache.get(key)),
            Command::Put(key, value) => {
                cache.put(key, value);
                None
            },
        }
    }
}

fn main() {
    let mut cache = LFUCache::new(1);

    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Failed to read from stdin");

        let command = match line.parse::<Command>() {
            Ok(command) => command,
            Err(e) => {
                eprintln!("{}", e);
                continue
            },
        };
        eprintln!("{:?}", command);

        let t0 = std::time::Instant::now();
        let result = command.apply(&mut cache);
        let elapsed = t0.elapsed();

        // eprintln!("{:#?}", cache);
        cache.dump();
        println!("> {:?} [{}ns]\t{:?}", result, elapsed.as_nanos(), command)
    }
}

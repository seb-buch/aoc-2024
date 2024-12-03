use std::fs::read_to_string;
use std::io;
use std::time::Duration;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn get_input_data(day: &str) -> io::Result<String> {
    let filename = format!("../data/input-{}.txt", day);
    read_to_string(filename)
}

#[macro_export]
macro_rules! time_function {
    ($expression:expr) => {{
        let start = std::time::Instant::now();
        let result = $expression;
        let end = start.elapsed();
        (result, end)
    }};
}

pub fn pretty_duration(duration: Duration) -> String {
    const UNITS: [&str; 4] = ["ns", "us", "ms", "s"];

    let mut duration_ns = duration.as_nanos();
    let mut unit = 0;
    while duration_ns > 1000 {
        duration_ns /= 1000;
        unit += 1;
        if unit >= UNITS.len() {
            break;
        }
    }

    format!("{} {}", duration_ns, UNITS[unit])
}

pub fn check_result(actual: &str, expected: &str) {
    if actual != expected {
        match expected {
            "TODO" => println!("⚠️ No known solution... Can not check"),
            _ => println!(
                "❌ Solution is invalid! (expected: {}, actual: {})",
                expected, actual
            ),
        }
        return;
    }
    println!("✅ Solution is valid!")
}

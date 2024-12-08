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
    const PERFORMANCE_EMOJI: [&str; 4] = ["🚀", "🏎️", "🐎", "🐌"];

    let mut duration_ns: f64 = duration.as_nanos() as f64;
    let mut unit = 0;
    while duration_ns > 1000f64 {
        duration_ns /= 1000f64;
        unit += 1;
        if unit >= UNITS.len() {
            break;
        }
    }

    format!(
        "{:.3} {} {}",
        duration_ns, UNITS[unit], PERFORMANCE_EMOJI[unit]
    )
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

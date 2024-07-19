use std::time::Instant;

fn main() {
    let start_time: Instant = Instant::now();
    let mut count: i32 = 0;
    let target: i32 = 500_000_000;
    while count < target {
        count += 1;
    }
    let elapsed_time: f64 = start_time.elapsed().as_secs_f64();
    println!("Counting to {} took {} seconds", target, elapsed_time);
}


use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    let sum_ns: Vec<u64> = (0..=1250).step_by(15).collect();
    let mut sum_iter_rows: Vec<(u64, f64)> = Vec::new();

    for &n in &sum_ns {
        let reps = 500;
        let start = Instant::now();
        for _ in 0..reps {
            sum_iterative(n);
        }
        let avg = start.elapsed().as_nanos() as f64 / reps as f64;
        sum_iter_rows.push((n, avg));
    }

    write_csv("suma_iterativa", &sum_iter_rows).unwrap();

    let mut sum_rec_rows: Vec<(u64, f64)> = Vec::new();
    for &n in &sum_ns {
        let reps = 500;
        let start = Instant::now();
        for _ in 0..reps {
            sum_recursive(n);
        }
        let avg = start.elapsed().as_nanos() as f64 / reps as f64;
        sum_rec_rows.push((n, avg));
    }
    write_csv("suma_recursiva", &sum_rec_rows).unwrap();

    let fib_ns: Vec<u64> = (0..=25).collect();
    let mut fib_iter_rows: Vec<(u64, f64)> = Vec::new();
    for &n in &fib_ns {
        let reps = 500;
        let start = Instant::now();
        for _ in 0..reps {
            fib_iterative(n);
        }
        let avg = start.elapsed().as_nanos() as f64 / reps as f64;
        fib_iter_rows.push((n, avg));
    }
    write_csv("fibonacci_iterativo", &fib_iter_rows).unwrap();

    let mut fib_rec_rows: Vec<(u64, f64)> = Vec::new();
    for &n in &fib_ns {
        let reps = match n {
            0..=15 => 500,
            16..=20 => 50,
            21..=23 => 10,
            _ => 3,
        };
        let start = Instant::now();
        for _ in 0..reps {
            fib_recursive(n);
        }
        let avg = start.elapsed().as_nanos() as f64 / reps as f64;
        fib_rec_rows.push((n, avg));
    }
    write_csv("fibonacci_recursivo", &fib_rec_rows).unwrap();
}

fn sum_iterative(n: u64) -> u64 {
    let mut acc: u64 = 0;
    for i in 1..=n {
        acc += i;
    }
    acc
}

fn sum_recursive(n: u64) -> u64 {
    if n == 0 { 0 } else { n + sum_recursive(n - 1) }
}

fn fib_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

fn write_csv(name: &str, rows: &[(u64, f64)]) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.csv", name))?;
    writeln!(file, "n,avg_time_ns")?;
    for (n, t) in rows {
        writeln!(file, "{},{:.2}", n, t)?;
    }
    Ok(())
}

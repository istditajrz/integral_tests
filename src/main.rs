#![feature(test)]
extern crate test;
#[cfg(test)]
use test::Bencher;

#[cfg(test)]
#[bench]
fn bench_integrate(b: &mut Bencher) {
    let f = |x: f64| (100.0 * x).tan();
    b.iter(||
        integrate(f, -100.0, 100.0, 1_000_000_000)
    );
}

fn integrate<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64
{
    let digits = 10_f64.powi(16);
    let h = (b - a) / (n as f64);
    let mut total = 0.0;
    for i in (0..n).map(|x| 
        (h / 2f64 + ((x as f64) * h))
    ) {
        total = (((f(i) * h).abs() + total) * digits).round() / digits;
    }
    total
}

fn main() {
    let f = |x: f64| x.powi(2);
    println!("{}", integrate(f, 0.0, 1.0, 1_000_000));
}

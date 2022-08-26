// euclidean and extended euclidean algorithms
// for finding the greatest common divisor (divisor communis maximus)

fn gcd(a: i32, b: i32) -> i32 {
    match a % b {
        0 => b,
        r => gcd(b, r)
    }
}

fn egcd(a: i32, b: i32) -> (i32, i32) {
    let mut x1 = 1; let mut y1 = 0;
    let mut x2 = 0; let mut y2 = 1;
    let mut p = x1 * a + y1 * b;
    let mut q = x2 * a + y2 * b;

    while p % q != 0 {
        let n = p / q;
        let x1_prev = x1;
        let y1_prev = y1;
        x1 = x2;
        y1 = y2;
        x2 = x1_prev - n * x2;
        y2 = y1_prev - n * y2;
        p = x1 * a + y1 * b;
        q = x2 * a + y2 * b;
    }
    (x2, y2)
}

fn main() {
    let a = 512;
    let b = 12;
    let e = gcd(a, b);
    let (x, y) = egcd(a, b);
    println!("{a}*{x} + {b}*{y} = {e}");
}


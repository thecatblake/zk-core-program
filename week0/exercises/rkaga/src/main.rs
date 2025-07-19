fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let mut x = 1i64;
    let mut y = 0i64;
    let mut z = 0i64;
    let mut w = 1i64;
    while a % b != 0 {
        (x, z) = (z, x - z * (a / b));
        (y, w) = (w, y - w * (a / b));
        (a, b) = (b, a % b);
    };
    (b, z, w)
}

fn mod_inv(x: i64, m: i64) -> i64 {
    let (_, s, _) = extended_gcd(x, m);
    s
}

fn main() {
    println!("{:?}", (3 - 2) % 5);
    println!("{:?}", (3 * mod_inv(2, 5)) % 5);
    println!("{:?}", mod_inv(7 * 5, 13));
}

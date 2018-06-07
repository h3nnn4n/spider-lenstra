fn divmod(a: i64, b: i64) -> (i64, i64) {
    let q = a / b;
    let r = a % b;

    (q, r)
}

fn modular_inv(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (1, 0, a);
    }

    let (q, r) = divmod(a, b);
    let (x, y, g) = modular_inv(b, r);

    (x, x - q * y, g)
}

fn elliptic_add(p: (i64, i64, i64), q: (i64, i64, i64), a: i64, b: i64, m: i64) -> (i64, i64, i64) {
    if p.2 == 0 {
        return q;
    }

    if q.2 == 0 {
        return p;
    }

    let num;
    let denom;

    if p.0 == q.0 {
        if (p.1 + q.1) % m == 0 {
            return (0, 1, 0);
        }

        num = (3 * p.0 * p.0 + a) % m;
        denom = (2 * p.1) % m;
    } else {
        num = (q.1 - p.1) % m;
        denom = (q.0 - p.0) % m;
    }

    let (inv, _, g) = modular_inv(denom, m);

    if g > 1 {
        return (0, 0, denom);
    }

    let z = (num * inv * num * inv - p.0 - q.0) % m;

    (z, (num * inv * (p.0 - z) - p.1) % m, 1)
}

fn elliptic_mul(mut k: i64, mut p: (i64, i64, i64), a: i64, b: i64, m: i64) -> (i64, i64, i64) {
    let mut r = (0, 1, 0);

    while k > 0 {
        if p.2 > 1 {
            return p;
        }

        if k % 2 == 1 {
            r = elliptic_add(p, r, a, b, m);
        }

        k = k / 2;
        p = elliptic_add(p, p, a, b, m);
    }

    r
}

fn main() {
    println!("Hello, world!");
}

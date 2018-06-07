extern crate rand;
use rand::Rng;

fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}

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

fn primes(limit: usize) -> Vec<i64> {
    let max = (1.0 + limit as f64 / (limit as f64).ln()) as usize;
    let mut prime_list: Vec<i64> = Vec::new();
    let mut primes = Vec::with_capacity(max as usize);
    let mut items_pushed = 0;

    loop {
        primes.push(true);
        items_pushed += 1;
        if items_pushed == max {
            break;
        }
    }

    primes[0] = false;
    if max > 1 {
        primes[1] = false;
    }

    for i in 0..max {
        if primes[i] {
            let mut mult = i << 1;
            while mult < max {
                primes[mult] = false;
                mult += i;
            }
        }
    }

    for (n, &prime) in primes.iter().enumerate() {
        if prime {
            prime_list.push(n as i64);
        }
    }

    prime_list
}

fn lenstra_(n: i64, limit: i64) -> Option<i64> {
    let mut g = n;
    let mut rng = rand::thread_rng();

    let mut q = (0, 0, 0);
    let mut a = 0;
    let mut b = 0;

    while g == n {
        q = (rng.gen_range(0, n - 1), rng.gen_range(0, n - 1), 1);
        a = rng.gen_range(0, n - 1);

        b = (q.1 * q.1 - q.0 * q.0 * q.0 - a * q.0) % n;
        g = gcd(4 * a * a * a + 27 * b * b, n);
    }

    if g > 1 {
        return Some(g);
    }

    for &p in primes(limit as usize).iter() {
        let mut pp = p;
        while pp < limit {
            q = elliptic_mul(p, q, a, b, n);

            if q.2 > 1 {
                return Some(gcd(q.2, n));
            }

            pp = p * pp;
        }
    }

    None
}

fn lenstra(n: i64, limit: i64) -> i64 {
    for _ in 0..10 {
        match lenstra_(n, limit) {
            Some(m) => return m,
            None => (),
        }
    }

    -1
}

fn factors_of(n: i64, limit: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();

    factors.push(1);
    factors.push(n);

    for _ in 0..500 {
        let q = match lenstra_(n, limit) {
            Some(m) => m,
            None => 1,
        };

        if !factors.contains(&q) {
            factors.push(q);
        }
    }

    factors.sort();
    factors
}

fn main() {
    let n = 1271;
    println!("Factors of {:?}: {:?}", n, factors_of(n, 1000));
}

#[cfg(test)]
mod tests {

    #[test]

    fn factors_of() {
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];

        for i in primes.iter() {
            for j in primes.iter() {
                let n = *i * *j;

                if n % 2 == 0 || n % 3 == 0 {
                    continue;
                }

                let factors = super::factors_of(n, 1000);

                assert!(factors.contains(&i), " {:?} {:?} {:?}", n, i, factors);
                assert!(factors.contains(&j), " {:?} {:?} {:?}", n, j, factors);

                if i == j {
                    assert!(factors.len() >= 3);
                } else {
                    assert!(factors.len() >= 4);
                }
            }
        }
    }
}

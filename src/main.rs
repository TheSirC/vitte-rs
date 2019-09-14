use rand::prelude::*;

//'a' is space allocated for the hand
//'n' is the size of the hand
//'N' is the upper bound on the random card values
fn vitter(mut a: Vec<i64>, mut n: i64, mut N: i64) {
    let mut rng = rand::thread_rng();
    let mut nr = n as f64;
    let Nr = N as f64;

    // Candidates to const transformation
    let i = 0;
    let mut j = 1_i64;
    let t: i64;
    let mut qu1: i64 = 1 - n + N;
    let negative_alpha_inv: i64 = -13;
    let n_min_inv = (nr - 1.0).powf(-1.0);
    let mut threshold: i64 = -negative_alpha_inv * n;

    let top: f64;
    let mut bottom: f64;
    let mut limit: i64;
    let mut S: i64;

    let vprime = |mut rng: rand::rngs::ThreadRng, nr: f64| rng.gen::<f64>().powf(nr.powf(-1.0));
    let mut Vprime: f64;

    while (n > 1 && threshold < N) {
        loop {
            let mut X: f64 = Nr * (1.0 - vprime(rng, nr));
            S = X.floor() as i64;
            // FIXME A float to integer comparison is made, is it legit ?
            while S < qu1 {
                X = Nr * (1.0 - vprime(rng, nr));
                S = X.floor() as i64;
            }
            let y1 = (rng.gen::<f64>() * Nr / qu1 as f64).powf(n_min_inv);
            Vprime = y1 * (-X / Nr + 1.0) * (qu1 as f64 / (qu1 as f64 - S as f64));
            if Vprime <= 1.0 {
                break;
            }
            let mut top = Nr - 1.0;
            if nr - 1.0 > S as f64 {
                bottom = Nr - nr;
                limit = N - S;
            } else {
                bottom = Nr - S as f64 - 1.0;
                limit = qu1;
            }
            let mut y2 = 1.0;
            for t in (N - 1..=limit) {
                y2 *= top / bottom;
                top -= 1.0;
                bottom -= 1.0;
            }

            if (Nr / (Nr - X) >= y1 * y2.powf(n_min_inv)) {
                Vprime = rng.gen::<f64>().powf(n_min_inv);
                break;
            }

            Vprime = vprime(rng, nr);
        }
        j += S + 1;

        a[i + 1] = j;
        N = N - 1 - S;
        n -= 1;
        nr -= 1.0;

        qu1 -= S;
        threshold += negative_alpha_inv;

        match n > 1 {
            true => method_a(&mut a, n, N, j),
            _ => {
                S = (Nr * Vprime).floor() as i64;
                j += S + 1;
                a[i + 1] = j;
            }
        }
    }
}

fn method_a(a: &mut Vec<i64>, n: i64, mut N: i64, mut j: i64) {
    let mut rng = rand::thread_rng();
    let mut S: i64 = 0;
    let i: usize = 0;
    let mut top = (N - n) as f64;
    while n >= 2 {
        let V = rng.gen::<f64>();
        let mut quot = top / (N as f64);
        while quot > V {
            S += 1;
            top -= 1.0;
            N -= 1;
            quot *= top / (N as f64);
        }
        S = N * rng.gen::<i64>();
        j += S + 1;
        a[i + 1] = j;
    }
}

fn main() {
    let v: Vec<i64> = (1..30).collect();
    let l = v.len() as i64;
    vitter(v.clone(), 5, l);
    println!("{:?}", v);
}

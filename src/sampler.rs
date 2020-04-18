use rand::prelude::*;
use std::iter::Iterator;

/// The problem is to draw a random sample of size n without replacement from a collection containing N records;
/// the n records must appear in the same order in the sample as they do in the collection.
/// Another formulation is to form a sorted random set of n elements from {1, 2, ... , N}.
/// The sample size n is typically very small relative to the collection size N.
pub trait Sampler<T: Iterator> {
    fn sample<'s>(self, n: usize, N: usize, alpha: usize)
        -> Box<dyn Iterator<Item = T::Item> + 's>;
    fn method_d(n: usize, N: usize, alpha: usize) -> Vec<bool>;
    fn method_a(rng: ThreadRng, n: usize, N: usize, output: Vec<bool>) -> Vec<bool>;
}

impl<T> Sampler<T> for T
where
    T: Iterator + 'static,
{
    fn sample<'s>(
        self,
        n: usize,
        N: usize,
        alpha: usize,
    ) -> Box<dyn Iterator<Item = T::Item> + 's> {
        // Generating the booleans representing the elements that are sampled
        let mut filtered = Self::method_d(n, N, alpha);
        filtered.reverse();
        Box::new(self.filter(move |_| {
            if let Some(decision) = filtered.pop() {
                decision
            } else {
                false
            }
        }))
    }

    fn method_d(mut n: usize, mut N: usize, alpha: usize) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        let mut output = Vec::with_capacity(n);
        // Enum encoding the step where are at in the computation.
        enum State {
            S1,
            S2,
            S3,
            S4,
            S5,
        }
        let mut step = State::S1;
        // Variables linked to the sampling.
        let mut qu1 = N - n + 1;
        let n_min_inv: f64 = 1.0 / (n as f64 - 1.0);
        let mut U: f64 = rng.gen();
        let mut y1: f64 = (U * N as f64 / qu1 as f64).powf(n_min_inv as f64);
        let beta = rand_distr::Beta::new(1.0, n as f64).unwrap();
        let mut X: f64 = rng.sample(beta);
        // As long as we have samples to pick...
        while n > 1 {
            match step {
                // Step 1. If n >= alpha*N, use Method A to finish the sampling and then terminate.
                State::S1 => {
                    if n >= alpha * N {
                        return Self::method_a(rng, n, N, output);
                    } else {
                        step = State::S2;
                    }
                }
                // Step 2. [Generate U and X]
                State::S2 => {
                    // Generate independently a uniform random variate U from the unit interval.
                    U = rng.gen();

                    // A good choice for X is the beta distribution scaled to the interval [0, N] with parameters a = 1, b = n.
                    // X is regenerated until X < N - n + 1.
                    X = loop {
                        let X = N as f64 * rng.sample(beta); // N is there to stretch the Beta distribution from [0, 1] to [0, N]
                        if X < qu1 as f64 {
                            break X;
                        }
                    };

                    step = State::S3;
                }
                // Step 3. [Accept?] If U <= h(⌊X⌋)/cg(X), then set S := ⌊X⌋ and go to Step D5.
                State::S3 => {
                    y1 = (U * N as f64 / qu1 as f64).powf(n_min_inv as f64);
                    let Vp = y1 * (-X / N as f64 + 1.0) * (1.0 / (1.0 - X.trunc() / (qu1 as f64)));
                    if Vp <= 1.0 {
                        step = State::S5;
                    } else {
                        step = State::S4;
                    }
                }
                // Step 4. [Accept?] If U >= f(⌊X⌋)/cg(X)
                State::S4 => {
                    let mut bottom;
                    let mut top = N as f64 - 1.0;
                    let limit;
                    let mut y2 = 1.0;
                    if n - 1 > (X.trunc() as usize) {
                        bottom = (N - n) as f64;
                        limit = N - (X.trunc() as usize);
                    } else {
                        bottom = (N as f64 - X.trunc() - 1.0) as f64;
                        limit = qu1;
                    }
                    for _ in (N - 1)..=limit {
                        y2 *= top / bottom;
                        top -= 1.0;
                        bottom -= 1.0;
                    }
                    if 1.0 / (1.0 - X / (N as f64)) >= y1 * y2.powf(n_min_inv as f64) {
                        step = State::S5;
                    } else {
                        // then set S := ⌊X⌋. Otherwise, return to Step 2.
                        step = State::S2;
                    }
                }
                // Step 5. [Select the (S + 1)st record.] Skip over the next S records in the file and then the following one for the sample.
                State::S5 => {
                    // Set iV := N - S - 1 and n := n - 1.
                    n -= 1;
                    N -= (X - 1.0).trunc() as usize;
                    qu1 -= X.trunc() as usize;
                    // Return to Step 1 if n > 0.
                    step = State::S1;
                    let mut rejected: Vec<bool> = [false].repeat(X.trunc() as usize);
                    output.append(&mut rejected);
                    output.push(true);
                }
            };
        }
        output
    }

    fn method_a(
        mut rng: ThreadRng,
        mut n: usize,
        mut N: usize,
        mut output: Vec<bool>,
    ) -> Vec<bool> {
        let mut top = N - n;
        while n >= 2 {
            let mut S = 0;
            let mut quot = top as f64 / N as f64;
            while quot > rng.gen() {
                S += 1;
                top -= 1;
                N -= 1;
                quot *= top as f64 / N as f64;
            }
            N -= 1;
            n -= 1;
            let mut rejected = [false].repeat(S);
            output.append(&mut rejected);
            output.push(true);
        }
        let S: usize = ((N as f64).round() * rng.gen::<f64>() as f64).trunc() as usize;
        let mut rejected = [false].repeat(S);
        output.append(&mut rejected);
        output.push(true);
        output
    }
}

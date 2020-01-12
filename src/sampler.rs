use rand::prelude::*;
// The problem is to draw a random sample of size n without replacement from a collection containing N records;
// the n records must appear in the same order in the sample as they do in the collection.
// Another formulation is to form a sorted random set of n elements from {1, 2, ... , N}.
// The sample size n is typically very small relative to the collection size N.
pub trait Sampler {
    // Associated type corresponding to the collection we are trying to get a sample from.
    type Collection;
    // Associated function doing the sampling on the collection.
    // alpha is the parameter that decides when to use Algorithm A instead of the acceptance-rejection method.
    fn sample(&mut self, mut n: usize, mut N: usize, alpha: usize) -> Self::Collection;

    fn method_a(
        &mut self,
        rng: ThreadRng,
        mut output: Self::Collection,
        mut n: usize,
        mut N: usize,
    ) -> Self::Collection;
}

impl<T: Copy> Sampler for Vec<T> {
    type Collection = Vec<T>;
    fn sample(&mut self, mut n: usize, mut N: usize, alpha: usize) -> Self::Collection {
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
        let mut selected_sample: usize = 0;
        let mut qu1 = N - n + 1;
        let mut nmininv: f64 = 1.0 / (n as f64 - 1.0);
        let mut U: f64 = rng.gen();
        let mut y1: f64 = (U * N as f64 / qu1 as f64).powf(nmininv as f64);
        let mut X: f64 = rng.sample(rand_distr::Beta::new(1.0, n as f64).unwrap());
        // As long as we have samples to pick...
        while n > 1 {
            let sample = match step {
                State::S1 => {
                    // Step 1. If n>= alpha*N, use Method A to finish the sampling and then terminate.
                    if n >= alpha * N {
                        return self.method_a(rng, output, n, N);
                    } else {
                        step = State::S2;
                        None
                    }
                }
                State::S2 => {
                    // Step 2. [Generate U and X]
                    // Generate independently a uniform random variate U from the unit interval.
                    // TODO: Use the genericity of the gen function to abstract sampling over the collection.
                    U = rng.gen();

                    // A good choice for X is the beta distribution scaled to the interval [0, N] with parameters a = 1, b = n.
                    // X is regenerated until X < N - n + 1.
                    X = loop {
                        let X =
                            N as f64 * rng.sample(rand_distr::Beta::new(1.0, n as f64).unwrap());
                        if X < qu1 as f64 {
                            break X;
                        }
                    };

                    step = State::S3;
                    None
                }
                State::S3 => {
                    // Step 3. [Accept?] If U I h(LXJ)/cg(X), then set S := LXJ and go to Step D5.
                    y1 = (U * N as f64 / qu1 as f64).powf(nmininv as f64);
                    let Vp = y1 * (-X / N as f64 + 1.0) * (1.0 / (1.0 - X.trunc() / (qu1 as f64)));
                    if Vp <= 1.0 {
                        step = State::S5;
                        None
                    } else {
                        step = State::S4;
                        None
                    }
                }
                State::S4 => {
                    let mut bottom;
                    let mut top = N as f64 - 1.0;
                    let limit;
                    let mut y2 = 1.0;
                    // Step 4. [Accept?] If U I f(LXJ)/cg(X)
                    if n - 1 > (X.trunc() as usize) {
                        bottom = (N - n) as f64;
                        limit = N - (X.trunc() as usize);
                    } else {
                        bottom = (N as f64 - X.trunc() - 1.0) as f64;
                        limit = qu1;
                    }
                    for t in (N - 1)..=limit {
                        y2 *= top / bottom;
                        top -= 1.0;
                        bottom -= 1.0;
                    }
                    if 1.0 / (1.0 - X / (N as f64)) >= y1 * y2.powf(nmininv as f64) {
                        step = State::S5;
                        None
                    } else {
                        // then set S := LXJ. Otherwise, return to Step 2.
                        step = State::S2;
                        None
                    }
                }
                State::S5 => {
                    // Step 5. [Select the (S + 1)st record.] Skip over the next S records in the file and then the following one for the sample.
                    // Set iV := N - S - 1 and n := n - 1.
                    n -= 1;
                    N -= (X - 1.0).trunc() as usize;
                    qu1 -= X.trunc() as usize;
                    // Return to Step 1 if n > 0.
                    step = State::S1;
                    selected_sample += X.trunc() as usize + 1;
                    Some(self[selected_sample])
                }
            };
            if let Some(sample) = sample {
                output.push(sample);
            }
        }
        output
    }

    fn method_a(
        &mut self,
        mut rng: ThreadRng,
        mut output: Self::Collection,
        mut n: usize,
        mut N: usize,
    ) -> Self::Collection {
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
            output.push(self[S + 1]);
        }
        let S: usize = ((N as f64).round() * rng.gen::<f64>() as f64).trunc() as usize;
        output.push(self[S + 1]);

        output
    }
}

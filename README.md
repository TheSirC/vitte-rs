# Vitte-rs : A way to sample from your collection

This library is a Rust port of Method D from __"An Efficient Algorithm for
Sequential Random Sampling", Jeffrey Scott Vitter, ACM Transactions on Mathematical Software, 13(1), March 1987, 58-67__

## Description

To quote the paper itself :

> The problem is to draw a random sample of size n without replacement from a file containing N records; the n records must appear in the same order in the sample as they do in the file. Another formulation is to form a sorted random set of n elements from (1, 2, . . . , N). The sample size n is typically very small relative to the file size N.

In other words : "How to sample in order a collection as fast as possible ?"

## Usage

``` rust
use vitte_rs::sampler::Sampler;

fn main() {
    let size_of_collection = 1_000_000_000;
    let size_of_sampled = 1_000_000;
    let v: Vec<u64> = (1..size_of_collection).collect();
    let l = v.len();
    let p = v
        .into_iter()
        .sample(size_of_sampled, l, 13)
        .collect::<Vec<_>>();
    println!("{:?}", p);
}
```

## Etymology

The name of the crate is a pun with the name of the original researcher (Jeffrey Scott Vitter) and the french word for quickly ("vite").


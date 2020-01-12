mod sampler;
use sampler::Sampler;
fn main() {
    let mut v: Vec<u64> = (1..30000).collect();
    let mut l = v.len();
    println!(
        "{:?}",
        v.sample(700, l, 13)
            .windows(2)
            .map(|s| s[1] - s[0])
            .collect::<Vec<_>>()
    );
}

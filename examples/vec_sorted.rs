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

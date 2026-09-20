use app::ListSet;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn generate_random_set(size: usize) -> ListSet<i32> {
    let mut set = ListSet::new();
    let mut rng = rand::thread_rng();
    while set.elements.len() < size {
        set.insert(rng.gen_range(1..100_000));
    }
    set
}

fn bench_search(c: &mut Criterion) {
    let sizes = [100, 500, 1000, 5000, 10000];

    for size in sizes {
        let set = generate_random_set(size);
        let existing_item = set.elements[size / 2];
        let missing_item = -1;

        c.bench_function(&format!("Search Existing (size: {})", size), |b| {
            b.iter(|| set.search(black_box(&existing_item)))
        });

        c.bench_function(&format!("Search Missing (size: {})", size), |b| {
            b.iter(|| set.search(black_box(&missing_item)))
        });
    }
}

fn bench_union(c: &mut Criterion) {
    let sizes = [100, 500, 1000, 2000];

    for size in sizes {
        let set_a = generate_random_set(size);
        let set_b = generate_random_set(size);

        c.bench_function(&format!("Union (size: {})", size), |b| {
            b.iter(|| set_a.union(black_box(&set_b)))
        });
    }
}

criterion_group!(benches, bench_search, bench_union);
criterion_main!(benches);

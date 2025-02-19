use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use ssg::metatags::generate_metatags;

pub fn criterion_benchmark(c: &mut Criterion) {
    let meta = vec![
        ("description".to_owned(), "My web page".to_owned()),
        ("author".to_owned(), "John Doe".to_owned()),
        (
            "viewport".to_owned(),
            "width=device-width, initial-scale=1.0".to_owned(),
        ),
        ("robots".to_owned(), "noindex, nofollow".to_owned()),
    ];

    c.bench_function("generate metatags", |b| {
        b.iter(|| {
            let result = generate_metatags(black_box(&meta));
            assert!(result.contains("<meta name=\"description\" content=\"My web page\">"));
            assert!(result.contains("<meta name=\"author\" content=\"John Doe\">"));
            assert!(result.contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"));
            assert!(result.contains("<meta name=\"robots\" content=\"noindex, nofollow\">"));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

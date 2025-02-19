use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use ssg::frontmatter::extract;

#[cfg(test)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let content = "---\n\
                   title: My Title\n\
                   date: 2000-01-01\n\
                   description: My Description\n\
                   keywords: foo, bar, baz\n\
                   permalink: /my-permalink\n\
                   layout: page\n\
                   ---\n\
                   My content";
    c.bench_function("extract front matter", |b| {
        b.iter(|| {
            let result = extract(black_box(content));
            assert!(
                result.contains_key("title"),
                "title not found in front matter"
            );
            assert!(
                result.contains_key("date"),
                "date not found in front matter"
            );
            assert!(
                result.contains_key("description"),
                "description not found in front matter"
            );
            assert!(
                result.contains_key("keywords"),
                "keywords not found in front matter"
            );
            assert!(
                result.contains_key("permalink"),
                "permalink not found in front matter"
            );
            assert!(
                result.contains_key("layout"),
                "layout not found in front matter"
            );
            assert!(
                result.contains_key("---"),
                "--- not found in front matter"
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

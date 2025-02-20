use criterion::{black_box, Criterion};
use ssg::json::{manifest, ManifestOptions};

#[cfg(test)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let options = ManifestOptions {
        background_color: "#ffffff".to_owned(),
        description: "My Web App".to_owned(),
        dir: "ltr".to_owned(),
        display: "standalone".to_owned(),
        icons: "icons/icon-512x512.png".to_owned(),
        identity: "My Web App".to_owned(),
        lang: "en-US".to_owned(),
        name: "My Web App".to_owned(),
        orientation: "portrait".to_owned(),
        scope: "/".to_owned(),
        short_name: "My App".to_owned(),
        start_url: "/index.html".to_owned(),
        theme_color: "#ffffff".to_owned(),
    };

    c.bench_function("generate manifest", |b| {
        b.iter(|| {
            let result = manifest(black_box(&options));
            assert!(
                result.contains("\"background_color\": \"#ffffff\"")
            );
            assert!(result.contains("\"description\": \"My Web App\""));
            assert!(result.contains("\"dir\": \"ltr\""));
            assert!(result.contains("\"display\": \"standalone\""));
            assert!(result
                .contains("\"icons\": \"icons/icon-512x512.png\""));
            assert!(result.contains("\"identity\": \"My Web App\""));
            assert!(result.contains("\"lang\": \"en-US\""));
            assert!(result.contains("\"name\": \"My Web App\""));
            assert!(result.contains("\"orientation\": \"portrait\""));
            assert!(result.contains("\"scope\": \"/\""));
            assert!(result.contains("\"short_name\": \"My App\""));
            assert!(result.contains("\"start_url\": \"/index.html\""));
            assert!(result.contains("\"theme_color\": \"#ffffff\""));
        })
    });
}

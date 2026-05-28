use criterion::{criterion_group, criterion_main, Criterion};
use graphitepdf_kit::*;
use std::io::Cursor;

fn bench_simple_doc(c: &mut Criterion) {
    c.bench_function("simple pdf doc", |b| {
        b.iter(|| {
            let text = TextBuilder::new()
                .font("F1", 16.0)
                .position(100.0, 700.0)
                .text("hello")
                .finish();
            let doc = DocumentBuilder::new().with_page(PageSize::A4, text);
            let mut buf = Cursor::new(Vec::new());
            doc.write(&mut buf).unwrap();
            buf.into_inner()
        })
    });
}

fn bench_graphics(c: &mut Criterion) {
    c.bench_function("canvas/graphics", |b| {
        b.iter(|| {
            Canvas::new()
                .fill_color(Color::RED)
                .rect(100.0, 600.0, 200.0,50.0)
                .fill()
                .finish()
        })
    });
}

criterion_group!(benches, bench_simple_doc, bench_graphics);
criterion_main!(benches);

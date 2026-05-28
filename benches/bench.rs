use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_bam_reheader(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-bam-reheader");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bam = manifest.join("tests/golden/reads.sam");
    let hdr = manifest.join("tests/golden/new_header.sam");
    c.bench_function("rsomics-bam-reheader golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([hdr.to_str().unwrap(), bam.to_str().unwrap(), "-o", "/dev/null"])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_bam_reheader);
criterion_main!(benches);

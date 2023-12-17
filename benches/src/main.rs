use criterion::{criterion_group, criterion_main, Criterion};

// const GLTF_SPONZA_STRING: &str =
//     include_str!("../../../../glTF-Sample-Models/2.0/Sponza/glTF/Sponza.gltf");
// const GLTF_METAL_ROUGH_SPHERES_STRING: &str = include_str!(
//     "../../../../glTF-Sample-Models/2.0/MetalRoughSpheres/glTF/MetalRoughSpheres.gltf"
// );
// const GLTF_BUGGY_STRING: &str =
//     include_str!("../../../../glTF-Sample-Models/2.0/Buggy/glTF/Buggy.gltf");
const GLTF_EMBEDDED_METAL_ROUGH_SPHERES_STRING: &str = include_str!(
    "../../../../glTF-Sample-Models/2.0/MetalRoughSpheres/glTF-Embedded/MetalRoughSpheres.gltf"
);
const GLTF_EMBEDDED_BUGGY_STRING: &str =
    include_str!("../../../../glTF-Sample-Models/2.0/Buggy/glTF-Embedded/Buggy.gltf");

fn test_competition_embedded_buggy_string() {
    let gltf = gltf::Gltf::from_slice(GLTF_EMBEDDED_BUGGY_STRING.as_bytes())
        .expect("Could not parse gltf");
}
fn test_competition_embedded_metal_rough_spheres_string() {
    let gltf = gltf::Gltf::from_slice(GLTF_EMBEDDED_METAL_ROUGH_SPHERES_STRING.as_bytes())
        .expect("Could not parse gltf");
}

fn test_serde_embedded_buggy_string() {
    let gltf = minigltf::gltf_json::Gltf::from_gltf_str(GLTF_EMBEDDED_BUGGY_STRING)
        .expect("Could not parse gltf");
}
fn test_serde_embedded_metal_rough_spheres_string() {
    let gltf = minigltf::gltf_json::Gltf::from_gltf_str(GLTF_EMBEDDED_METAL_ROUGH_SPHERES_STRING)
        .expect("Could not parse gltf");
}

fn competition_criterion_benchmark(c: &mut Criterion) {
    c.bench_function("competition_embedded_buggy", |b| {
        b.iter(|| test_competition_embedded_buggy_string())
    });
    c.bench_function("competition_embedded_metal_rough_spheres", |b| {
        b.iter(|| test_competition_embedded_metal_rough_spheres_string())
    });
}

fn serde_criterion_benchmark(c: &mut Criterion) {
    c.bench_function("serde_embedded_buggy", |b| {
        b.iter(|| test_serde_embedded_buggy_string())
    });
    c.bench_function("serde_embedded_metal_rough_spheres", |b| {
        b.iter(|| test_serde_embedded_metal_rough_spheres_string())
    });
}

criterion_group!(
    benches,
    serde_criterion_benchmark,
    competition_criterion_benchmark
);
criterion_main!(benches);

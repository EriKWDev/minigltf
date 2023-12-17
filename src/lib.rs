pub mod gltf_json;

#[doc(inline)]
pub use gltf_json::*;

#[cfg(test)]
mod sponza_test {
    use super::*;

    const GLTF_SPONZA_STRING: &str =
        include_str!("../../../glTF-Sample-Models/2.0/Sponza/glTF/Sponza.gltf");

    #[test]
    fn test_sponza_string() {
        Gltf::from_gltf_str(GLTF_SPONZA_STRING).expect("Could not parse gltf from chars");
    }

    #[test]
    fn test_sponza_from_path() {
        Gltf::from_path("../../glTF-Sample-Models/2.0/Sponza/glTF/Sponza.gltf")
            .expect("Could not parse path");
    }
}

#[cfg(test)]
mod specific_test {
    use super::*;

    const GLTF_STRING: &str = include_str!(
        "../../../glTF-Sample-Models/2.0/LightsPunctualLamp/glTF/LightsPunctualLamp.gltf"
    );

    #[test]
    fn test_specific() {
        Gltf::from_gltf_str(GLTF_STRING).expect("Could not parse gltf from chars");
    }
}

#[cfg(test)]
mod mega_test {
    use super::*;

    #[test]
    fn test_mega() {
        fn visit_dirs<T>(
            dir: impl AsRef<std::path::Path>,
            cb: fn(&std::fs::DirEntry) -> T,
        ) -> std::io::Result<Vec<T>> {
            let dir = dir.as_ref();
            let mut result = vec![];

            if dir.is_dir() {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        result.extend(visit_dirs(&path, cb)?);
                    } else {
                        result.push(cb(&entry));
                    }
                }
            }

            Ok(result)
        }

        let total_start = std::time::Instant::now();

        let mut results = visit_dirs("../../glTF-Sample-Models/2.0", |file| {
            let path = file.path().to_owned();

            std::thread::spawn(move || match path.extension() {
                Some(ext) if ext == "gltf" => {
                    let it = std::fs::read_to_string(&path).unwrap();

                    let start = std::time::Instant::now();
                    let mut gltf = Gltf::from_gltf_str(it.as_str());
                    for _ in 0..2 {
                        gltf = Gltf::from_gltf_str(it.as_str());
                    }
                    let delta = start.elapsed().div_f64(3.0);

                    let path: std::path::PathBuf = path.iter().skip(5).collect();
                    let res = Some((delta, path, gltf));

                    res
                }
                _ => None,
            })
        })
        .unwrap();

        let mut failed = vec![];
        let mut succeeded = vec![];

        let n = results.len();
        for (i, res) in results.drain(..).enumerate() {
            print!("Parsing {:4}/{:4}...\r", i + 1, n);

            match res.join().unwrap() {
                Some((delta, path, Ok(it))) => {
                    succeeded.push((delta, path, it));
                }
                Some((delta, path, Err(it))) => {
                    failed.push((delta, path, it));
                }
                None => {}
            }
        }
        let total_delta = total_start.elapsed();
        println!("");

        succeeded.sort_by_key(|it| it.0);
        failed.sort_by_key(|it| it.0);

        for (delta, path, _it) in &succeeded {
            let path = path.as_path().display();
            println!("({delta:?}) test {path} ... ok");
        }
        println!("");

        for (delta, path, err) in &failed {
            let path = path.as_path().display();
            println!("({delta:?}) test {path} ... FAILED");
            println!("reason: '{err:?}'");
        }
        println!("total time: {total_delta:?}");
        println!("");

        assert!(failed.is_empty());
    }
}

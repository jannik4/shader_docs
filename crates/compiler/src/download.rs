use crate::CompilerBackend;
use cargo_metadata::{MetadataCommand, Package};
use docs::Version;
use regex::Regex;
use reqwest::blocking::Client;
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use tar::Archive;

pub struct ShaderSource {
    pub path: PathBuf,
    pub source: String,
    pub shader_defs: HashSet<String>,
    pub docsrs_url: String,
}

pub fn download_shaders(
    root_crate_name: &str,
    root_crate_version: &Version,
    package_filter: impl Fn(&str) -> bool,
    cache_path: &Path,
    backend: CompilerBackend,
) -> Result<Vec<ShaderSource>, Box<dyn std::error::Error>> {
    let manifest_path = download_crate(cache_path, root_crate_name, root_crate_version)?;
    let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

    let mut shaders = Vec::new();

    for package in &metadata.packages {
        if package.name == "naga_oil" && package.version.minor != backend.naga_oil_minor() {
            println!(
                "Warning: naga_oil version mismatch: compiling with {}, found {}",
                backend.naga_oil_minor(),
                package.version.minor
            );
        }

        if package_filter(&package.name) {
            let crate_path = package
                .manifest_path
                .parent()
                .unwrap()
                .to_path_buf()
                .into_std_path_buf();

            let mut dirs = vec![crate_path.clone()];
            while let Some(parent) = dirs.pop() {
                for entry in fs::read_dir(parent)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        dirs.push(path);
                    } else if path.is_file() && path.extension() == Some("wgsl".as_ref()) {
                        let source = fs::read_to_string(&path)?;
                        let shader_defs = find_defs(&source);

                        let docsrs_url = {
                            let mut url = format!(
                                "https://docs.rs/crate/{}/{}/source",
                                package.name, package.version
                            );

                            let local = path.strip_prefix(&crate_path)?;
                            for segment in local.components() {
                                url.push('/');
                                url.push_str(&segment.as_os_str().to_string_lossy());
                            }

                            url
                        };

                        let source = fix_bevy_14139(source, package);

                        shaders.push(ShaderSource {
                            path,
                            source,
                            shader_defs,
                            docsrs_url,
                        });
                    }
                }
            }
        }
    }

    Ok(shaders)
}

fn download_crate(
    cache_path: &Path,
    name: &str,
    version: &Version,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let crate_path = cache_path.join(format!("{name}@{version}"));
    let manifest_path = crate_path.join(format!("{name}-{version}/Cargo.toml"));

    if manifest_path.exists() {
        return Ok(manifest_path);
    }

    let client = Client::builder().no_gzip().build()?;
    let url = crates_io_download_url(name, version);

    let response = client.get(url).send()?;
    let decoded = flate2::read::GzDecoder::new(response);

    let mut archive = Archive::new(decoded);
    archive.unpack(&crate_path)?;

    // Add empty workspace section to manifest, so that cargo does not complain about any workspace
    let mut file = OpenOptions::new().append(true).open(&manifest_path)?;
    file.write_all(b"\n[workspace]\n")?;

    Ok(manifest_path)
}

fn crates_io_download_url(name: &str, version: &Version) -> String {
    format!("https://static.crates.io/crates/{name}/{name}-{version}.crate")
}

fn find_defs(source: &str) -> HashSet<String> {
    fn ifdef_regex() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        fn init() -> Regex {
            Regex::new(r"^\s*#\s*(else\s+)?\s*ifdef\s+([\w|\d|_]+)").unwrap()
        }
        RE.get_or_init(init)
    }
    fn ifndef_regex() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        fn init() -> Regex {
            Regex::new(r"^\s*#\s*(else\s+)?\s*ifndef\s+([\w|\d|_]+)").unwrap()
        }
        RE.get_or_init(init)
    }
    fn ifop_regex() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        fn init() -> Regex {
            Regex::new(r"^\s*#\s*(else\s+)?\s*if\s+([\w|\d|_]+)\s*([=!<>]*)\s*([-\w|\d]+)").unwrap()
        }
        RE.get_or_init(init)
    }
    fn def_regex() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        fn init() -> Regex {
            Regex::new(r"#\s*([\w|\d|_]+)").unwrap()
        }
        RE.get_or_init(init)
    }
    fn def_regex_delimited() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        fn init() -> Regex {
            Regex::new(r"#\s*\{([\w|\d|_]+)\}").unwrap()
        }
        RE.get_or_init(init)
    }

    let ifdef_regex = ifdef_regex();
    let ifndef_regex = ifndef_regex();
    let ifop_regex = ifop_regex();
    let _def_regex = def_regex();
    let def_regex_delimited = def_regex_delimited();

    let mut defs = HashSet::new();

    for line in source.lines() {
        if let Some(caps) = ifdef_regex.captures(line) {
            let def = caps.get(2).unwrap().as_str();
            defs.insert(def.to_string());
        }
        if let Some(caps) = ifndef_regex.captures(line) {
            let def = caps.get(2).unwrap().as_str();
            defs.insert(def.to_string());
        }
        if let Some(caps) = ifop_regex.captures(line) {
            let def = caps.get(2).unwrap().as_str();
            defs.insert(def.to_string());
        }
        // FIXME: Too many false positives
        // for caps in def_regex.captures_iter(line) {
        //     let def = caps.get(1).unwrap().as_str();
        //     defs.insert(def.to_string());
        // }
        for caps in def_regex_delimited.captures_iter(line) {
            let def = caps.get(1).unwrap().as_str();
            defs.insert(def.to_string());
        }
    }

    defs
}

// Fixes: https://github.com/bevyengine/bevy/issues/14139
fn fix_bevy_14139(source: String, package: &Package) -> String {
    if !(package.name == "bevy_render" && package.version == Version::new(0, 14, 0)) {
        return source;
    }

    let code = r#"fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let n = vec3(5.0, 3.0, 1.0);
    let k = (n + hsv.x / FRAC_PI_3) % 6.0;
    return hsv.z - hsv.z * hsv.y * max(vec3(0.0), min(k, min(4.0 - k, vec3(1.0))));
}"#;
    if source.contains(code) {
        source.replace(
            code,
            r#"fn hsv_to_rgb(x: f32, y: f32, z: f32) -> vec3<f32> {
    let n = vec3(5.0, 3.0, 1.0);
    let k = (n + x / FRAC_PI_3) % 6.0;
    return z - z * y * max(vec3(0.0), min(k, min(4.0 - k, vec3(1.0))));
}"#,
        )
    } else {
        source
    }
}

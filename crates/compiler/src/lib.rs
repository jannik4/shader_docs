mod common;
mod download;
mod post_process;

#[cfg(feature = "backend_v0_14")]
mod backend_v0_14;

#[cfg(feature = "backend_v0_13")]
mod backend_v0_13;

#[cfg(feature = "backend_v0_11")]
mod backend_v0_11;

#[cfg(feature = "backend_v0_10")]
mod backend_v0_10;

#[cfg(feature = "backend_v0_08")]
mod backend_v0_08;

use docs::{Doc, IndexMap, ShaderDefValue, Version};

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilerBackend {
    #[cfg(feature = "backend_v0_14")]
    V0_14,
    #[cfg(feature = "backend_v0_13")]
    V0_13,
    #[cfg(feature = "backend_v0_11")]
    V0_11,
    #[cfg(feature = "backend_v0_10")]
    V0_10,
    #[cfg(feature = "backend_v0_08")]
    V0_08,
}

impl CompilerBackend {
    fn naga_oil_minor(self) -> u64 {
        match self {
            #[cfg(feature = "backend_v0_14")]
            CompilerBackend::V0_14 => 14,
            #[cfg(feature = "backend_v0_13")]
            CompilerBackend::V0_13 => 13,
            #[cfg(feature = "backend_v0_11")]
            CompilerBackend::V0_11 => 11,
            #[cfg(feature = "backend_v0_10")]
            CompilerBackend::V0_10 => 10,
            #[cfg(feature = "backend_v0_08")]
            CompilerBackend::V0_08 => 8,
        }
    }
}

pub fn compile(
    root_crate_name: &str,
    root_crate_version: Version,
    package_filter: impl Fn(&str) -> bool,
    shader_def_values: IndexMap<String, ShaderDefValue>,
    cache_path: &Path,
    backend: CompilerBackend,
) -> Result<Doc, Box<dyn std::error::Error>> {
    let shader_sources = download::download_shaders(
        root_crate_name,
        &root_crate_version,
        package_filter,
        cache_path,
        backend,
    )?;

    // CompileFn type is necessary to avoid compiler error if no backend is enabled
    let compile: CompileFn = match backend {
        #[cfg(feature = "backend_v0_14")]
        CompilerBackend::V0_14 => backend_v0_14::compile,
        #[cfg(feature = "backend_v0_13")]
        CompilerBackend::V0_13 => backend_v0_13::compile,
        #[cfg(feature = "backend_v0_11")]
        CompilerBackend::V0_11 => backend_v0_11::compile,
        #[cfg(feature = "backend_v0_10")]
        CompilerBackend::V0_10 => backend_v0_10::compile,
        #[cfg(feature = "backend_v0_08")]
        CompilerBackend::V0_08 => backend_v0_08::compile,
    };

    let mut doc = compile(
        root_crate_name,
        root_crate_version,
        shader_def_values.clone(),
        shader_sources,
    )?;

    post_process::post_process(&mut doc);

    for key in shader_def_values.keys() {
        if !doc.root.shader_defs.contains(key) {
            println!("Warning: shader def `{}` was provived but not used", key);
        }
    }

    Ok(doc)
}

type CompileFn = fn(
    &str,
    Version,
    IndexMap<String, ShaderDefValue>,
    Vec<download::ShaderSource>,
) -> Result<Doc, Box<dyn std::error::Error>>;

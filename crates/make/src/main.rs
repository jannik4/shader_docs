use docs::{IndexMap, ShaderDefValue, Version};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile docs
    let cache_path = Path::new("target/shader_docs_cache");
    let docs = vec![
        compiler::compile(
            "bevy",
            Version::new(0, 12, 1),
            |name| name.starts_with("bevy"),
            shader_def_values_0_12(),
            cache_path,
            compiler::CompilerBackend::V0_10,
        )?,
        compiler::compile(
            "bevy",
            Version::new(0, 12, 0),
            |name| name.starts_with("bevy"),
            shader_def_values_0_12(),
            cache_path,
            compiler::CompilerBackend::V0_10,
        )?,
        compiler::compile(
            "bevy",
            Version::new(0, 11, 3),
            |name| name.starts_with("bevy"),
            shader_def_values_0_11(),
            cache_path,
            compiler::CompilerBackend::V0_08,
        )?,
        compiler::compile(
            "bevy",
            Version::new(0, 11, 2),
            |name| name.starts_with("bevy"),
            shader_def_values_0_11(),
            cache_path,
            compiler::CompilerBackend::V0_08,
        )?,
        compiler::compile(
            "bevy",
            Version::new(0, 11, 1),
            |name| name.starts_with("bevy"),
            shader_def_values_0_11(),
            cache_path,
            compiler::CompilerBackend::V0_08,
        )?,
        compiler::compile(
            "bevy",
            Version::new(0, 11, 0),
            |name| name.starts_with("bevy"),
            shader_def_values_0_11(),
            cache_path,
            compiler::CompilerBackend::V0_08,
        )?,
    ];

    // Generate docs
    for doc in docs {
        generator::generate(&doc, Path::new("target/shader_docs"))?;
    }

    Ok(())
}

fn shader_def_values_0_12() -> IndexMap<String, ShaderDefValue> {
    use ShaderDefValue::*;

    [
        ("AVAILABLE_STORAGE_BUFFER_BINDINGS", UInt(3)),
        ("BLEND_MULTIPLY", Bool(true)),
        ("BLEND_PREMULTIPLIED_ALPHA", Bool(true)),
        ("DEFERRED_PREPASS", Bool(true)),
        ("DEPTH_PREPASS", Bool(true)),
        ("ENVIRONMENT_MAP", Bool(true)),
        ("MAX_CASCADES_PER_LIGHT", UInt(4)),
        ("MAX_DIRECTIONAL_LIGHTS", UInt(10)),
        ("MORPH_TARGETS", Bool(true)),
        ("MOTION_VECTOR_PREPASS", Bool(true)),
        ("MULTISAMPLED", Bool(true)),
        ("NORMAL_PREPASS", Bool(true)),
        ("NORMAL_PREPASS_OR_DEFERRED_PREPASS", Bool(true)),
        ("PBR_TRANSMISSION_TEXTURES_SUPPORTED", Bool(true)),
        ("PREPASS_FRAGMENT", Bool(true)),
        ("PREPASS_PIPELINE", Bool(true)),
        ("SKINNED", Bool(true)),
        ("STANDARDMATERIAL_NORMAL_MAP", Bool(true)),
        ("TONEMAPPING_PASS", Bool(true)),
        ("TONEMAP_METHOD_TONY_MC_MAPFACE", Bool(true)),
        ("VERTEX_COLORS", Bool(true)),
        ("VERTEX_NORMALS", Bool(true)),
        ("VERTEX_OUTPUT_INSTANCE_INDEX", Bool(true)),
        ("VERTEX_POSITIONS", Bool(true)),
        ("VERTEX_TANGENTS", Bool(true)),
        ("VERTEX_UVS", Bool(true)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}

fn shader_def_values_0_11() -> IndexMap<String, ShaderDefValue> {
    use ShaderDefValue::*;

    [
        ("AVAILABLE_STORAGE_BUFFER_BINDINGS", UInt(3)),
        ("BLEND_MULTIPLY", Bool(true)),
        ("BLEND_PREMULTIPLIED_ALPHA", Bool(true)),
        ("DEPTH_PREPASS", Bool(true)),
        ("ENVIRONMENT_MAP", Bool(true)),
        ("MAX_CASCADES_PER_LIGHT", UInt(4)),
        ("MAX_DIRECTIONAL_LIGHTS", UInt(10)),
        ("MORPH_TARGETS", Bool(true)),
        ("MOTION_VECTOR_PREPASS", Bool(true)),
        ("MULTISAMPLED", Bool(true)),
        ("NORMAL_PREPASS", Bool(true)),
        ("SKINNED", Bool(true)),
        ("STANDARDMATERIAL_NORMAL_MAP", Bool(true)),
        ("TONEMAPPING_PASS", Bool(true)),
        ("TONEMAP_METHOD_TONY_MC_MAPFACE", Bool(true)),
        ("VERTEX_COLORS", Bool(true)),
        ("VERTEX_TANGENTS", Bool(true)),
        ("VERTEX_UVS", Bool(true)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}

use docs::{IndexMap, ShaderDefValue, Version};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile docs
    let cache_path = Path::new("target/shader_docs_cache");
    let docs = vec![
        compiler::compile(
            "bevy",
            Version::new(0, 15, 0),
            |name| name.starts_with("bevy"),
            shader_def_values_0_15(),
            cache_path,
            compiler::CompilerBackend::V0_15,
        )?, /*
            compiler::compile(
                "bevy",
                Version::new(0, 14, 0),
                |name| name.starts_with("bevy"),
                shader_def_values_0_14(),
                cache_path,
                compiler::CompilerBackend::V0_14,
            )?,
            compiler::compile(
                "bevy",
                Version::new(0, 13, 2),
                |name| name.starts_with("bevy"),
                shader_def_values_0_13(),
                cache_path,
                compiler::CompilerBackend::V0_13,
            )?,
            compiler::compile(
                "bevy",
                Version::new(0, 13, 1),
                |name| name.starts_with("bevy"),
                shader_def_values_0_13(),
                cache_path,
                compiler::CompilerBackend::V0_13,
            )?,
            compiler::compile(
                "bevy",
                Version::new(0, 13, 0),
                |name| name.starts_with("bevy"),
                shader_def_values_0_13(),
                cache_path,
                compiler::CompilerBackend::V0_13,
            )?,
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
            )?,*/
    ];

    // Generate docs
    for doc in docs {
        generator::generate(&doc, Path::new("target/shader_docs"))?;
    }

    Ok(())
}

// TODO: Check list for bevy 0.15
fn shader_def_values_0_15() -> IndexMap<String, ShaderDefValue> {
    use ShaderDefValue::*;

    [
        // From 0.14:
        ("AVAILABLE_STORAGE_BUFFER_BINDINGS", UInt(3)),
        ("BLEND_MULTIPLY", Bool(true)),
        ("BLEND_PREMULTIPLIED_ALPHA", Bool(true)),
        ("DEFERRED_PREPASS", Bool(true)),
        ("DEPTH_PREPASS", Bool(true)),
        ("ENVIRONMENT_MAP", Bool(true)),
        ("IRRADIANCE_VOLUME", Bool(true)),
        ("IRRADIANCE_VOLUMES_ARE_USABLE", Bool(true)),
        ("LIGHTMAP", Bool(true)),
        ("MAX_CASCADES_PER_LIGHT", UInt(4)),
        ("MAX_DIRECTIONAL_LIGHTS", UInt(10)),
        ("MORPH_TARGETS", Bool(true)),
        ("MOTION_VECTOR_PREPASS", Bool(true)),
        // ("MULTISAMPLED", Bool(true)), causes error from 0.14.
        ("NORMAL_PREPASS", Bool(true)),
        ("NORMAL_PREPASS_OR_DEFERRED_PREPASS", Bool(true)),
        ("PBR_TRANSMISSION_TEXTURES_SUPPORTED", Bool(true)),
        ("PREPASS_FRAGMENT", Bool(true)),
        ("PREPASS_PIPELINE", Bool(true)),
        ("SKINNED", Bool(true)),
        ("STANDARD_MATERIAL_CLEARCOAT", Bool(true)),
        ("STANDARD_MATERIAL_DIFFUSE_TRANSMISSION", Bool(true)),
        ("STANDARD_MATERIAL_NORMAL_MAP", Bool(true)),
        ("STANDARD_MATERIAL_SPECULAR_TRANSMISSION", Bool(true)),
        ("TONEMAPPING_LUT_SAMPLER_BINDING_INDEX", UInt(20)),
        ("TONEMAPPING_LUT_TEXTURE_BINDING_INDEX", UInt(20)),
        ("TONEMAP_METHOD_TONY_MC_MAPFACE", Bool(true)),
        ("VERTEX_COLORS", Bool(true)),
        ("VERTEX_NORMALS", Bool(true)),
        ("VERTEX_OUTPUT_INSTANCE_INDEX", Bool(true)),
        ("VERTEX_POSITIONS", Bool(true)),
        ("VERTEX_TANGENTS", Bool(true)),
        ("VERTEX_UVS", Bool(true)),
        ("VERTEX_UVS_A", Bool(true)),
        ("VERTEX_UVS_B", Bool(true)),
        // New for 0.15
        ("DIRECTIONAL_LIGHT_SHADOW_MAP_DEBUG_CASCADES", Bool(true)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}

// TODO: More shader defs available on 0.14
fn shader_def_values_0_14() -> IndexMap<String, ShaderDefValue> {
    use ShaderDefValue::*;

    [
        ("AVAILABLE_STORAGE_BUFFER_BINDINGS", UInt(3)),
        ("BLEND_MULTIPLY", Bool(true)),
        ("BLEND_PREMULTIPLIED_ALPHA", Bool(true)),
        ("DEFERRED_PREPASS", Bool(true)),
        ("DEPTH_PREPASS", Bool(true)),
        ("ENVIRONMENT_MAP", Bool(true)),
        ("IRRADIANCE_VOLUME", Bool(true)),
        ("IRRADIANCE_VOLUMES_ARE_USABLE", Bool(true)),
        ("LIGHTMAP", Bool(true)),
        ("MAX_CASCADES_PER_LIGHT", UInt(4)),
        ("MAX_DIRECTIONAL_LIGHTS", UInt(10)),
        ("MORPH_TARGETS", Bool(true)),
        ("MOTION_VECTOR_PREPASS", Bool(true)),
        // ("MULTISAMPLED", Bool(true)), causes error
        ("NORMAL_PREPASS", Bool(true)),
        ("NORMAL_PREPASS_OR_DEFERRED_PREPASS", Bool(true)),
        ("PBR_TRANSMISSION_TEXTURES_SUPPORTED", Bool(true)),
        ("PREPASS_FRAGMENT", Bool(true)),
        ("PREPASS_PIPELINE", Bool(true)),
        ("SKINNED", Bool(true)),
        ("STANDARD_MATERIAL_CLEARCOAT", Bool(true)),
        ("STANDARD_MATERIAL_DIFFUSE_TRANSMISSION", Bool(true)),
        ("STANDARD_MATERIAL_NORMAL_MAP", Bool(true)),
        ("STANDARD_MATERIAL_SPECULAR_TRANSMISSION", Bool(true)),
        ("TONEMAPPING_LUT_SAMPLER_BINDING_INDEX", UInt(20)),
        ("TONEMAPPING_LUT_TEXTURE_BINDING_INDEX", UInt(20)),
        ("TONEMAP_METHOD_TONY_MC_MAPFACE", Bool(true)),
        ("VERTEX_COLORS", Bool(true)),
        ("VERTEX_NORMALS", Bool(true)),
        ("VERTEX_OUTPUT_INSTANCE_INDEX", Bool(true)),
        ("VERTEX_POSITIONS", Bool(true)),
        ("VERTEX_TANGENTS", Bool(true)),
        ("VERTEX_UVS", Bool(true)),
        ("VERTEX_UVS_A", Bool(true)),
        ("VERTEX_UVS_B", Bool(true)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}

fn shader_def_values_0_13() -> IndexMap<String, ShaderDefValue> {
    use ShaderDefValue::*;

    [
        ("AVAILABLE_STORAGE_BUFFER_BINDINGS", UInt(3)),
        ("BLEND_MULTIPLY", Bool(true)),
        ("BLEND_PREMULTIPLIED_ALPHA", Bool(true)),
        ("DEFERRED_PREPASS", Bool(true)),
        ("DEPTH_PREPASS", Bool(true)),
        ("ENVIRONMENT_MAP", Bool(true)),
        ("IRRADIANCE_VOLUME", Bool(true)),
        ("IRRADIANCE_VOLUMES_ARE_USABLE", Bool(true)),
        ("LIGHTMAP", Bool(true)),
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
        ("STANDARD_MATERIAL_DIFFUSE_TRANSMISSION", Bool(true)),
        ("STANDARD_MATERIAL_NORMAL_MAP", Bool(true)),
        (
            "STANDARD_MATERIAL_SPECULAR_OR_DIFFUSE_TRANSMISSION",
            Bool(true),
        ),
        ("STANDARD_MATERIAL_SPECULAR_TRANSMISSION", Bool(true)),
        ("TONEMAPPING_PASS", Bool(true)),
        ("TONEMAP_METHOD_TONY_MC_MAPFACE", Bool(true)),
        ("VERTEX_COLORS", Bool(true)),
        ("VERTEX_NORMALS", Bool(true)),
        ("VERTEX_OUTPUT_INSTANCE_INDEX", Bool(true)),
        ("VERTEX_POSITIONS", Bool(true)),
        ("VERTEX_TANGENTS", Bool(true)),
        ("VERTEX_UVS", Bool(true)),
        ("VERTEX_UVS_B", Bool(true)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
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

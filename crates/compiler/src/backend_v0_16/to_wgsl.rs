use super::naga::{self, proc::GlobalCtx, Scalar, TypeInner};
use super::{NAGA_OIL_DECORATION_POST, NAGA_OIL_DECORATION_PRE};
use std::collections::HashMap;

pub fn build_ty(
    ty: &naga::Type,
    gctx: GlobalCtx,
    def_paths: &HashMap<String, Vec<String>>,
) -> docs::Type {
    match &ty.name {
        Some(name) => {
            let pre_pos = name.find(NAGA_OIL_DECORATION_PRE);
            let ends_with_post = name.ends_with(NAGA_OIL_DECORATION_POST);

            let name = match (pre_pos, ends_with_post) {
                (Some(pre_pos), true) => name[..pre_pos].to_string(),
                _ => name.clone(),
            };
            let def_path = def_paths.get(&name).cloned();
            docs::Type::Named { name, def_path }
        }
        None => build_ty_inner(&ty.inner, gctx, def_paths),
    }
}

// Copy-pasted and adapted from: naga-0.19.2

pub fn build_ty_inner(
    type_inner: &TypeInner,
    gctx: GlobalCtx,
    def_paths: &HashMap<String, Vec<String>>,
) -> docs::Type {
    let name = match *type_inner {
        TypeInner::Vector { size, scalar } => {
            format!("vec{}<{}>", vector_size_str(size), scalar_kind_str(scalar),)
        }
        TypeInner::Sampler { comparison: false } => "sampler".to_string(),
        TypeInner::Sampler { comparison: true } => "sampler_comparison".to_string(),
        TypeInner::Image {
            dim,
            arrayed,
            class,
        } => {
            // More about texture types: https://gpuweb.github.io/gpuweb/wgsl/#sampled-texture-type
            use naga::ImageClass as Ic;

            let dim_str = image_dimension_str(dim);
            let arrayed_str = if arrayed { "_array" } else { "" };
            let (class_str, multisampled_str, format_str, storage_str) = match class {
                Ic::Sampled { kind, multi } => (
                    "",
                    if multi { "multisampled_" } else { "" },
                    scalar_kind_str(naga::Scalar { kind, width: 4 }),
                    "",
                ),
                Ic::Depth { multi } => ("depth_", if multi { "multisampled_" } else { "" }, "", ""),
                Ic::Storage { format, access } => (
                    "storage_",
                    "",
                    storage_format_str(format),
                    if access.contains(naga::StorageAccess::LOAD | naga::StorageAccess::STORE) {
                        ",read_write"
                    } else if access.contains(naga::StorageAccess::LOAD) {
                        ",read"
                    } else {
                        ",write"
                    },
                ),
            };
            let mut out = format!("texture_{class_str}{multisampled_str}{dim_str}{arrayed_str}");

            if !format_str.is_empty() {
                out += &format!("<{format_str}{storage_str}>");
            }

            out
        }
        TypeInner::Scalar(scalar) => scalar_kind_str(scalar).to_string(),
        TypeInner::Atomic(scalar) => {
            format!("atomic<{}>", scalar_kind_str(scalar))
        }
        TypeInner::Array {
            base,
            size,
            stride: _,
        } => {
            // More info https://gpuweb.github.io/gpuweb/wgsl/#array-types
            // array<A, 3> -- Constant array
            // array<A> -- Dynamic array
            let member_type = &gctx.types[base];
            return match size {
                naga::ArraySize::Constant(size) => docs::Type::ArrayConstant(
                    Box::new(build_ty(member_type, gctx, def_paths)),
                    Some(size.get()),
                ),
                naga::ArraySize::Dynamic => {
                    docs::Type::ArrayDynamic(Box::new(build_ty(member_type, gctx, def_paths)))
                }
            };
        }
        TypeInner::BindingArray { base, size } => {
            // More info https://github.com/gpuweb/gpuweb/issues/2105
            let member_type = &gctx.types[base];
            return match size {
                naga::ArraySize::Constant(size) => docs::Type::BindingArrayConstant(
                    Box::new(build_ty(member_type, gctx, def_paths)),
                    Some(size.get()),
                ),
                naga::ArraySize::Dynamic => docs::Type::BindingArrayDynamic(Box::new(build_ty(
                    member_type,
                    gctx,
                    def_paths,
                ))),
            };
        }
        TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => {
            format!(
                "mat{}x{}<{}>",
                vector_size_str(columns),
                vector_size_str(rows),
                scalar_kind_str(scalar)
            )
        }
        TypeInner::Pointer { base, space } => {
            let (address, maybe_access) = address_space_str(space);
            // Everything but `AddressSpace::Handle` gives us a `address` name, but
            // Naga IR never produces pointers to handles, so it doesn't matter much
            // how we write such a type. Just write it as the base type alone.
            let base = build_ty(&gctx.types[base], gctx, def_paths);
            return match address {
                Some(address) => docs::Type::PointerWithAddressSpace {
                    base: Box::new(base),
                    address_space: address,
                    maybe_access,
                },
                None => base,
            };
        }
        TypeInner::ValuePointer {
            size,
            scalar,
            space,
        } => {
            let (address, maybe_access) = address_space_str(space);
            let base = docs::Type::Named {
                name: match size {
                    Some(size) => {
                        format!("vec{}<{}>", vector_size_str(size), scalar_kind_str(scalar))
                    }
                    None => scalar_kind_str(scalar).to_string(),
                },
                def_path: None,
            };
            return match address {
                Some(address) => docs::Type::PointerWithAddressSpace {
                    base: Box::new(base),
                    address_space: address,
                    maybe_access,
                },
                None => base,
            };
        }
        TypeInner::AccelerationStructure => "acceleration_structure".to_string(),
        TypeInner::Struct { .. } => {
            // TODO: Actually output the struct?
            "struct".to_string()
        }
        TypeInner::RayQuery { .. } => {
            // TODO: ???
            "ray_query".to_string()
        }
    };

    let def_path = def_paths.get(&name).cloned();
    docs::Type::Named { name, def_path }
}

const fn vector_size_str(size: naga::VectorSize) -> &'static str {
    match size {
        naga::VectorSize::Bi => "2",
        naga::VectorSize::Tri => "3",
        naga::VectorSize::Quad => "4",
    }
}

const fn image_dimension_str(dim: naga::ImageDimension) -> &'static str {
    use naga::ImageDimension as IDim;

    match dim {
        IDim::D1 => "1d",
        IDim::D2 => "2d",
        IDim::D3 => "3d",
        IDim::Cube => "cube",
    }
}

const fn scalar_kind_str(scalar: naga::Scalar) -> &'static str {
    use naga::ScalarKind as Sk;

    match scalar {
        Scalar {
            kind: Sk::Float,
            width: 8,
        } => "f64",
        Scalar {
            kind: Sk::Float,
            width: 4,
        } => "f32",
        Scalar {
            kind: Sk::Sint,
            width: 4,
        } => "i32",
        Scalar {
            kind: Sk::Uint,
            width: 4,
        } => "u32",
        Scalar {
            kind: Sk::Sint,
            width: 8,
        } => "i64",
        Scalar {
            kind: Sk::Uint,
            width: 8,
        } => "u64",
        Scalar {
            kind: Sk::Bool,
            width: 1,
        } => "bool",
        _ => unreachable!(),
    }
}

const fn storage_format_str(format: naga::StorageFormat) -> &'static str {
    use naga::StorageFormat as Sf;

    match format {
        Sf::R8Unorm => "r8unorm",
        Sf::R8Snorm => "r8snorm",
        Sf::R8Uint => "r8uint",
        Sf::R8Sint => "r8sint",
        Sf::R16Uint => "r16uint",
        Sf::R16Sint => "r16sint",
        Sf::R16Float => "r16float",
        Sf::Rg8Unorm => "rg8unorm",
        Sf::Rg8Snorm => "rg8snorm",
        Sf::Rg8Uint => "rg8uint",
        Sf::Rg8Sint => "rg8sint",
        Sf::R32Uint => "r32uint",
        Sf::R32Sint => "r32sint",
        Sf::R32Float => "r32float",
        Sf::Rg16Uint => "rg16uint",
        Sf::Rg16Sint => "rg16sint",
        Sf::Rg16Float => "rg16float",
        Sf::Rgba8Unorm => "rgba8unorm",
        Sf::Rgba8Snorm => "rgba8snorm",
        Sf::Rgba8Uint => "rgba8uint",
        Sf::Rgba8Sint => "rgba8sint",
        Sf::Bgra8Unorm => "bgra8unorm",
        Sf::Rgb10a2Uint => "rgb10a2uint",
        Sf::Rgb10a2Unorm => "rgb10a2unorm",
        Sf::Rg11b10Ufloat => "rg11b10float",
        Sf::Rg32Uint => "rg32uint",
        Sf::Rg32Sint => "rg32sint",
        Sf::Rg32Float => "rg32float",
        Sf::Rgba16Uint => "rgba16uint",
        Sf::Rgba16Sint => "rgba16sint",
        Sf::Rgba16Float => "rgba16float",
        Sf::Rgba32Uint => "rgba32uint",
        Sf::Rgba32Sint => "rgba32sint",
        Sf::Rgba32Float => "rgba32float",
        Sf::R16Unorm => "r16unorm",
        Sf::R16Snorm => "r16snorm",
        Sf::Rg16Unorm => "rg16unorm",
        Sf::Rg16Snorm => "rg16snorm",
        Sf::Rgba16Unorm => "rgba16unorm",
        Sf::Rgba16Snorm => "rgba16snorm",
    }
}

const fn address_space_str(
    space: naga::AddressSpace,
) -> (Option<&'static str>, Option<&'static str>) {
    use naga::AddressSpace as As;

    (
        Some(match space {
            As::Private => "private",
            As::Uniform => "uniform",
            As::Storage { access } => {
                if access.contains(naga::StorageAccess::STORE) {
                    return (Some("storage"), Some("read_write"));
                } else {
                    "storage"
                }
            }
            As::PushConstant => "push_constant",
            As::WorkGroup => "workgroup",
            As::Handle => return (None, None),
            As::Function => "function",
        }),
        None,
    )
}

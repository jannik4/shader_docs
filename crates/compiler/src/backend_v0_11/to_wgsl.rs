use super::naga::{self, proc::GlobalCtx, TypeInner};
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

// Copy-pasted and adapted from: naga-0.14.1

pub fn build_ty_inner(
    type_inner: &TypeInner,
    gctx: GlobalCtx,
    def_paths: &HashMap<String, Vec<String>>,
) -> docs::Type {
    use naga::TypeInner as Ti;

    let name =
        match *type_inner {
            Ti::Scalar { kind, width } => kind.to_wgsl(width),
            Ti::Vector { size, kind, width } => {
                format!("vec{}<{}>", size as u32, kind.to_wgsl(width))
            }
            Ti::Matrix {
                columns,
                rows,
                width,
            } => {
                format!(
                    "mat{}x{}<{}>",
                    columns as u32,
                    rows as u32,
                    naga::ScalarKind::Float.to_wgsl(width),
                )
            }
            Ti::Atomic { kind, width } => {
                format!("atomic<{}>", kind.to_wgsl(width))
            }
            Ti::Pointer { base, .. } => {
                let base = &gctx.types[base];
                return docs::Type::Pointer(Box::new(build_ty(base, gctx, def_paths)));
            }
            Ti::ValuePointer { kind, width, .. } => {
                format!("ptr<{}>", kind.to_wgsl(width))
            }
            Ti::Array { base, size, .. } => {
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
            Ti::Struct { .. } => {
                // TODO: Actually output the struct?
                "struct".to_string()
            }
            Ti::Image {
                dim,
                arrayed,
                class,
            } => {
                let dim_suffix = match dim {
                    naga::ImageDimension::D1 => "_1d",
                    naga::ImageDimension::D2 => "_2d",
                    naga::ImageDimension::D3 => "_3d",
                    naga::ImageDimension::Cube => "_cube",
                };
                let array_suffix = if arrayed { "_array" } else { "" };

                let class_suffix = match class {
                    naga::ImageClass::Sampled { multi: true, .. } => "_multisampled",
                    naga::ImageClass::Depth { multi: false } => "_depth",
                    naga::ImageClass::Depth { multi: true } => "_depth_multisampled",
                    naga::ImageClass::Sampled { multi: false, .. }
                    | naga::ImageClass::Storage { .. } => "",
                };

                let type_in_brackets = match class {
                    naga::ImageClass::Sampled { kind, .. } => {
                        // Note: The only valid widths are 4 bytes wide.
                        // The lexer has already verified this, so we can safely assume it here.
                        // https://gpuweb.github.io/gpuweb/wgsl/#sampled-texture-type
                        let element_type = kind.to_wgsl(4);
                        format!("<{element_type}>")
                    }
                    naga::ImageClass::Depth { multi: _ } => String::new(),
                    naga::ImageClass::Storage { format, access } => {
                        if access.contains(naga::StorageAccess::STORE) {
                            format!("<{},write>", format.to_wgsl())
                        } else {
                            format!("<{}>", format.to_wgsl())
                        }
                    }
                };

                format!("texture{class_suffix}{dim_suffix}{array_suffix}{type_in_brackets}")
            }
            Ti::Sampler { .. } => "sampler".to_string(),
            Ti::AccelerationStructure => "acceleration_structure".to_string(),
            Ti::RayQuery => "ray_query".to_string(),
            Ti::BindingArray { base, size, .. } => {
                let member_type = &gctx.types[base];
                return match size {
                    naga::ArraySize::Constant(size) => docs::Type::BindingArrayConstant(
                        Box::new(build_ty(member_type, gctx, def_paths)),
                        Some(size.get()),
                    ),
                    naga::ArraySize::Dynamic => docs::Type::BindingArrayDynamic(Box::new(
                        build_ty(member_type, gctx, def_paths),
                    )),
                };
            }
        };

    let def_path = def_paths.get(&name).cloned();
    docs::Type::Named { name, def_path }
}

trait ScalarKindToWgsl {
    fn to_wgsl(self, width: u8) -> String;
}

impl ScalarKindToWgsl for naga::ScalarKind {
    fn to_wgsl(self, width: u8) -> String {
        let prefix = match self {
            naga::ScalarKind::Sint => "i",
            naga::ScalarKind::Uint => "u",
            naga::ScalarKind::Float => "f",
            naga::ScalarKind::Bool => return "bool".to_string(),
        };
        format!("{}{}", prefix, width * 8)
    }
}

trait StorageFormatToWgsl {
    fn to_wgsl(self) -> &'static str;
}

impl StorageFormatToWgsl for naga::StorageFormat {
    fn to_wgsl(self) -> &'static str {
        use naga::StorageFormat as Sf;
        match self {
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
            Sf::Rg11b10Float => "rg11b10float",
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
}

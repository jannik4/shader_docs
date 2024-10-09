mod to_wgsl;
mod util;

use naga_oil_v0_13 as naga_oil;
use naga_v0_19 as naga;

use crate::{common, download::ShaderSource};
use docs::*;
use naga::TypeInner;
use naga_oil::compose::{
    self, ComposableModuleDescriptor, Composer, ImportDefinition, NagaModuleDescriptor,
    ShaderLanguage,
};
use std::collections::HashMap;
use to_wgsl::build_ty;
use util::{build_address_space, build_binding, build_expression, build_resource_binding};

const NAGA_OIL_DECORATION_PRE: &str = "X_naga_oil_mod_X";
const NAGA_OIL_DECORATION_POST: &str = "X";

pub fn compile(
    root_crate_name: &str,
    root_crate_version: Version,
    shader_def_values: IndexMap<String, ShaderDefValue>,
    shader_sources: Vec<ShaderSource>,
) -> Result<Doc, Box<dyn std::error::Error>> {
    let (shaders, mut composer) = compile_shaders(shader_sources)?;

    let mut doc = common::doc_new(
        root_crate_name.to_string(),
        root_crate_version,
        shader_def_values.clone(),
    );

    for (import_path, shader) in &shaders {
        let (module_path, module) = common::find_or_create_module(&mut doc, import_path);

        module
            .shader_defs
            .extend(shader.source.shader_defs.iter().cloned());
        module.source_url = Some(shader.source.docsrs_url.clone());

        let desc = NagaModuleDescriptor {
            source: &shader.source.source,
            shader_defs: shader_def_values
                .iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        match *value {
                            ShaderDefValue::Bool(value) => compose::ShaderDefValue::Bool(value),
                            ShaderDefValue::Int(value) => compose::ShaderDefValue::Int(value),
                            ShaderDefValue::UInt(value) => compose::ShaderDefValue::UInt(value),
                        },
                    )
                })
                .collect(),
            ..Default::default()
        };
        let naga_module = composer.make_naga_module(desc)?;
        let gctx = naga_module.to_ctx();

        let mut def_paths = HashMap::new();
        for import in &shader.imports {
            let module_path = import
                .import
                .split("::")
                .map(str::to_owned)
                .collect::<Vec<_>>();
            for item in &import.items {
                def_paths.insert(item.clone(), module_path.clone());
            }
        }
        for (_handle, ty) in naga_module.types.iter() {
            if !contains_pre(ty.name.as_deref()) {
                if let TypeInner::Struct { .. } = &ty.inner {
                    if let Some(name) = &ty.name {
                        def_paths.insert(name.clone(), module_path.clone());
                    }
                }
            }
        }

        for (_handle, constant) in naga_module.constants.iter() {
            if !contains_pre(constant.name.as_deref()) {
                module.constants.push(Constant {
                    name: Ident::from(constant.name.clone()),
                    ty: build_ty(&naga_module.types[constant.ty], gctx, &def_paths),
                    init: build_expression(&naga_module.const_expressions[constant.init]),
                    comment: None,
                });
            }
        }

        for (_handle, var) in naga_module.global_variables.iter() {
            if !contains_pre(var.name.as_deref()) {
                module.global_variables.push(GlobalVariable {
                    name: Ident::from(var.name.clone()),
                    space: build_address_space(&var.space),
                    binding: var.binding.as_ref().map(build_resource_binding),
                    ty: build_ty(&naga_module.types[var.ty], gctx, &def_paths),
                    init: var
                        .init
                        .map(|init| build_expression(&naga_module.const_expressions[init])),
                    comment: None,
                });
            }
        }

        for (_handle, ty) in naga_module.types.iter() {
            if !contains_pre(ty.name.as_deref()) {
                if let TypeInner::Struct { members, .. } = &ty.inner {
                    module.structs.push(Struct {
                        name: Ident::from(ty.name.clone()),
                        members: members
                            .iter()
                            .map(|member| StructMember {
                                name: Ident::from(member.name.clone()),
                                ty: build_ty(&naga_module.types[member.ty], gctx, &def_paths),
                                binding: member.binding.as_ref().map(build_binding),
                                comment: None,
                            })
                            .collect(),
                        comment: None,
                    });
                }
            }
        }

        for (_handle, function) in naga_module.functions.iter() {
            if !contains_pre(function.name.as_deref()) {
                module.functions.push(Function {
                    name: Ident::from(function.name.clone()),
                    arguments: function
                        .arguments
                        .iter()
                        .map(|arg| FunctionArgument {
                            name: Ident::from(arg.name.clone()),
                            ty: build_ty(&naga_module.types[arg.ty], gctx, &def_paths),
                            binding: arg.binding.as_ref().map(build_binding),
                        })
                        .collect(),
                    ret: function
                        .result
                        .as_ref()
                        .map(|res| build_ty(&naga_module.types[res.ty], gctx, &def_paths)),
                    comment: None,
                });
            }
        }
    }

    Ok(doc)
}

fn contains_pre(name: Option<&str>) -> bool {
    name.map(|name| name.contains(NAGA_OIL_DECORATION_PRE))
        .unwrap_or(false)
}

struct Shader {
    source: ShaderSource,
    imports: Vec<ImportDefinition>,
    defines: HashMap<String, compose::ShaderDefValue>,
}

fn compile_shaders(
    shader_sources: Vec<ShaderSource>,
) -> Result<(HashMap<String, Shader>, Composer), Box<dyn std::error::Error>> {
    let mut composer = Composer::default();
    let mut shaders = HashMap::new();

    for shader_source in shader_sources {
        let (import_path, imports, defines) = compose::get_preprocessor_data(&shader_source.source);
        if let Some(import_path) = import_path {
            shaders.insert(
                import_path,
                Shader {
                    source: shader_source,
                    imports,
                    defines,
                },
            );
        }
    }

    fn add_to_composer(
        composer: &mut Composer,
        name: &str,
        shaders: &HashMap<String, Shader>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !composer.contains_module(name) {
            let this = match shaders.get(name) {
                Some(this) => this,
                None => return Err(format!("shader not found: {}", name).into()),
            };

            for import in &this.imports {
                add_to_composer(composer, &import.import, shaders)?;
            }

            composer.add_composable_module(ComposableModuleDescriptor {
                source: &this.source.source,
                file_path: &this.source.path.to_string_lossy(),
                language: ShaderLanguage::Wgsl,
                additional_imports: Default::default(),
                shader_defs: this.defines.clone(),
                as_name: None,
            })?;
        }

        Ok(())
    }
    for name in shaders.keys() {
        add_to_composer(&mut composer, name, &shaders)?;
    }

    Ok((shaders, composer))
}

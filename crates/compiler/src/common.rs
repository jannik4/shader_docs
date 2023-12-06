use docs::{Doc, IndexMap, IndexSet, Module, ShaderDefValue, Version};

pub fn doc_new(
    root_crate_name: String,
    root_crate_version: Version,
    shader_def_values: IndexMap<String, ShaderDefValue>,
) -> Doc {
    Doc {
        version: root_crate_version,
        root: Module {
            name: root_crate_name,
            source_url: None,
            modules: Vec::new(),
            constants: Vec::new(),
            global_variables: Vec::new(),
            structs: Vec::new(),
            functions: Vec::new(),
            shader_defs: IndexSet::new(),
        },
        compiled_with: shader_def_values,
    }
}

pub fn find_or_create_module<'a>(
    doc: &'a mut Doc,
    import_path: &str,
) -> (Vec<String>, &'a mut Module) {
    let mut module = &mut doc.root;
    let path = import_path
        .split("::")
        .map(str::to_owned)
        .collect::<Vec<_>>();
    for segment in &path {
        let idx = module
            .modules
            .iter()
            .position(|module| &module.name == segment);
        module = match idx {
            Some(idx) => &mut module.modules[idx],
            None => {
                module.modules.push(Module {
                    name: segment.to_string(),
                    source_url: None,
                    modules: Vec::new(),
                    constants: Vec::new(),
                    global_variables: Vec::new(),
                    structs: Vec::new(),
                    functions: Vec::new(),
                    shader_defs: IndexSet::new(),
                });
                module.modules.last_mut().unwrap()
            }
        };
    }

    (path, module)
}

use docs::{Doc, IndexSet, Module};

pub fn post_process(doc: &mut Doc) {
    // Sort compiled_with shader defs
    doc.compiled_with.sort_keys();

    // Post process modules
    post_process_module(&mut doc.root);
}

fn post_process_module(module: &mut Module) -> IndexSet<String> {
    // Inner modules
    for inner in &mut module.modules {
        let res_inner = post_process_module(inner);
        module.shader_defs.extend(res_inner);
    }

    // Sort modules
    module.modules.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort constants
    module.constants.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort global variables
    module.global_variables.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort structs
    module.structs.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort functions
    module.functions.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort shader defs
    module.shader_defs.sort();

    module.shader_defs.clone()
}

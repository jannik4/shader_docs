use docs::{Doc, Ident, Module};
use serde::Serialize;

pub fn all_items(doc: &Doc) -> Vec<Item> {
    let mut items = Vec::new();
    all_items_module(&doc.root, &[], &mut items);
    items.sort();
    items
}

fn all_items_module(module: &Module, parent: &[String], items: &mut Vec<Item>) {
    let path = parent
        .iter()
        .cloned()
        .chain([module.name.clone()])
        .collect::<Vec<_>>();
    for inner in &module.modules {
        all_items_module(inner, &path, items);

        items.push(Item::new(
            path.clone(),
            inner.name.clone(),
            ItemKind::Module,
        ));
    }

    for item in &module.constants {
        if let Ident::Named(name) = &item.name {
            items.push(Item::new(path.clone(), name.clone(), ItemKind::Constant));
        }
    }

    for item in &module.global_variables {
        if let Ident::Named(name) = &item.name {
            items.push(Item::new(
                path.clone(),
                name.clone(),
                ItemKind::GlobalVariable,
            ));
        }
    }

    for item in &module.structs {
        if let Ident::Named(name) = &item.name {
            items.push(Item::new(path.clone(), name.clone(), ItemKind::Struct));
        }
    }

    for item in &module.functions {
        if let Ident::Named(name) = &item.name {
            items.push(Item::new(path.clone(), name.clone(), ItemKind::Function));
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    path: Vec<String>,
    name: String,
    kind: ItemKind,
    url: String,
}

impl Item {
    fn new(path: Vec<String>, name: String, kind: ItemKind) -> Self {
        let mut url = path.join("/");
        match kind {
            ItemKind::Module => url.push_str(&format!("/{}/index.html", name)),
            ItemKind::Constant => url.push_str(&format!("/const.{}.html", name)),
            ItemKind::GlobalVariable => url.push_str(&format!("/var.{}.html", name)),
            ItemKind::Struct => url.push_str(&format!("/struct.{}.html", name)),
            ItemKind::Function => url.push_str(&format!("/fn.{}.html", name)),
        }

        Self {
            path,
            name,
            kind,
            url,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
enum ItemKind {
    Module,
    Constant,
    GlobalVariable,
    Struct,
    Function,
}

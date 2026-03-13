#![allow(dead_code)]

use std::path::PathBuf;

struct Node<'a> {
    parent: PathBuf,
    kind: NodeKind<'a>,
}

enum NodeKind<'a> {
    File {
        name: &'a str,
    },
    Folder {
        name: &'a str,
        children: Vec<Node<'a>>,
    },
}

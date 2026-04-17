use std::{collections::HashMap, hash::Hash};

use crate::FileBuffer;

pub enum HashDirectory<K: Eq + Hash> {
    Folder(HashMap<K, HashDirectory<K>>),
    File {
        name: String,
        contents: Option<FileBuffer>
    },
    Unimplemented
}

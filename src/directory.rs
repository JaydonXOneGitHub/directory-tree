use std::{collections::HashMap, error::Error, fs::ReadDir};

use crate::{DirError, FileBuffer, HashDirectory};

pub enum Directory {
    File {
        name: String,
        contents: Option<FileBuffer>
    },
    Folder {
        name: String,
        children: Vec<Directory>
    },
    /// Used as a placeholder.
    Unimplemented
}

impl Directory {
    pub fn from_read_dir(read_dir: ReadDir, read_files: bool) -> Result<Self, Box<dyn Error>> {
        let mut directory = Self::Folder {
            name: String::from("/"),
            children: Vec::new()
        };

        for entry in read_dir {
            if let Result::Err(err) = entry {
                return Result::Err(Box::from(err));
            }

            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                let res = entry.file_name().into_string();

                if let Result::Err(_) = res {
                    return Result::Err(Box::from(DirError::Message(
                        "Unable to convert OsString to String!".into()
                    )));
                }

                let file_name = res.unwrap();

                if read_files {
                    let res = std::fs::read(path);

                    if let Result::Err(err) = res {
                        return Result::Err(Box::from(err));
                    }

                    let contents = res.unwrap();

                    if let Self::Folder { name: _, children } = &mut directory {
                        children.push(Directory::File {
                            name: file_name,
                            contents: Option::Some(FileBuffer::make(contents))
                        });
                    }
                } else {
                    if let Self::Folder { name: _, children } = &mut directory {
                        children.push(Directory::File {
                            name: file_name,
                            contents: Option::None
                        });
                    }
                }
            }
            else if path.is_dir() {
                let res = std::fs::read_dir(path);

                if let Result::Err(err) = res {
                    return Result::Err(Box::from(err));
                }

                let entry = res.unwrap();

                let res = Self::from_read_dir(entry, read_files);

                if let Result::Err(err) = res {
                    return Result::Err(err);
                }

                if let Self::Folder { name: _, children } = &mut directory {
                    children.push(res.unwrap());
                }
            }
        }

        return Result::Ok(directory);
    }
}

impl Directory {
    /// Recursively filters a Directory enum - and if the value is Folder { .. }, its children.
    pub fn filter<F: Fn(&Self) -> bool>(self, callback: F) -> Option<Self> {
        return self.internal_filter(&callback);
    }

    pub fn for_each<F, R, P>(&self, path: &str, params: P, callback: F) -> Result<R, Box<dyn Error>>
    where 
        F: Fn(&Self, String, P) -> Result<R, Box<dyn Error>>,
        R: Default {
        return self.internal_for_each(path, &params, &callback);
    }

    pub fn as_file(&self) -> Option<(&String, Option<&FileBuffer>)> {
        return match self {
            Self::File { name, contents } => Option::Some((name, contents.as_ref())),
            _ => Option::None
        };
    }
}


impl Directory {
    fn internal_filter<F: Fn(&Self) -> bool>(self, callback: &F) -> Option<Self> {
        if !callback(&self) {
            return Option::None;
        }

        match self {
            Self::File { .. } => {
                return Option::Some(self);
            },
            Self::Folder { name, children } => {
                let children = children
                    .into_iter()
                    .filter_map(|child| child.internal_filter(callback))
                    .collect();

                return Option::Some(Self::Folder { name, children });
            },
            Self::Unimplemented => {
                return Option::None;
            }
        }
    }

    fn internal_for_each<F, R, P>(&self, path: &str, params: &P, callback: &F) -> Result<R, Box<dyn Error>>
    where 
        F: Fn(&Self, String, P) -> Result<R, Box<dyn Error>>,
        R: Default {
        match self {
            Self::Folder { name, children } => {
                for child in children {
                    let new_path = if path.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", path, name)
                    };

                    let res = child.internal_for_each(&new_path, params, callback);

                    if let Result::Err(err) = res {
                        return Result::Err(err);
                    }

                    let res = child.internal_for_each(&new_path, params, callback);

                    if let Result::Err(err) = res {
                        return Result::Err(err);
                    }
                }

                return Result::Ok(R::default());
            },
            Self::File { name, contents } => {
                let file_path = if path.is_empty() {
                    name.clone()
                } else {
                    format!("{}/{}", path, name)
                };

                return Result::Ok(R::default());
            }
            _ => {
                return Result::Ok(R::default());
            }
        }
    }
}

impl TryFrom<ReadDir> for Directory {
    type Error = Box<dyn Error>;
    fn try_from(value: ReadDir) -> Result<Self, Self::Error> {
        return Self::from_read_dir(value, false);
    }
}

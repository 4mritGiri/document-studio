// src/engine/world.rs

use super::fonts::font_assets;
use std::collections::HashMap;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, Source};
use typst::text::Font;
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

pub struct SimpleWorld {
    library: LazyHash<Library>,
    source: Source,
    assets: HashMap<String, Bytes>,
}

impl SimpleWorld {
    pub fn new(source: Source, assets: HashMap<String, Bytes>) -> Self {
        font_assets();
        Self {
            library: LazyHash::new(Library::default()),
            source,
            assets,
        }
    }
}

impl World for SimpleWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(std::path::PathBuf::from(
                id.vpath().get_without_slash(),
            )))
        }
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path_str = id.vpath().get_without_slash();
        self.assets
            .get(path_str)
            .cloned()
            .ok_or(FileError::NotFound(std::path::PathBuf::from(path_str)))
    }
    fn font(&self, index: usize) -> Option<Font> {
        font_assets().fonts.get(index).map(|f| f.as_ref().clone())
    }
    fn book(&self) -> &LazyHash<typst::text::FontBook> {
        &font_assets().book
    }
    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}

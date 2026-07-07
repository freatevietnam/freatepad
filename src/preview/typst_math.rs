use std::path::PathBuf;

use typst::{Library, LibraryExt};
use typst::World;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::FileId;
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst_layout::PagedDocument;
use typst_svg::{SvgOptions, svg};

const MATH_FONT_DATA: &[u8] = include_bytes!("../../fonts/NewCMMath-Regular.otf");

pub struct TypstWorld {
    source: typst::syntax::Source,
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

impl TypstWorld {
    pub fn new(source: impl Into<String>) -> Self {
        let (book, fonts) = Self::load_fonts();
        Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            fonts,
            source: typst::syntax::Source::detached(source),
        }
    }

    fn load_fonts() -> (FontBook, Vec<Font>) {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        let buffer = Bytes::new(MATH_FONT_DATA);
        for font in Font::iter(buffer) {
            book.push(font.info().clone());
            fonts.push(font);
        }

        (book, fonts)
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<typst::syntax::Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(PathBuf::new()))
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::new()))
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}

pub fn render_typst_math(math_expr: &str, display_mode: bool) -> Option<String> {
    let wrapped = if display_mode {
        format!("$\n{}\n$", math_expr)
    } else {
        format!("${}$", math_expr)
    };

    let world = TypstWorld::new(wrapped);
    let result = typst::compile::<PagedDocument>(&world);

    match result.output {
        Ok(document) => {
            if let Some(page) = document.pages().first() {
                let opts = SvgOptions {
                    render_bleed: false,
                    pretty: false,
                };
                let svg_str = svg(page, &opts);
                Some(svg_str)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

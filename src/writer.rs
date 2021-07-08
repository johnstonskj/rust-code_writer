/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{ErrorKind, Result};
use crate::model::identity::Namespace;
use crate::model::{
    Comment, Enumeration, FunctionDecl, HasName, Import, Module, ModuleContent, NamedValue,
    StructuredType, TypeAlias,
};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum NewLine {
    LineFeed,
    CarriageReturnLineFeed,
}

#[derive(Clone, Debug)]
pub struct WhitespaceHandling {
    indent: String,
    new_line: NewLine,
    trim_trailing: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockPlacement {
    Trailing,
    TrailingNewLine,
    OwnLine,
    OwnLineIndented,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    open: String,
    open_placement: BlockPlacement,
    close: String,
    close_placement: BlockPlacement,
}

pub struct CodeWriter<W>
where
    W: Write,
{
    writer: RefCell<W>,
    line_count: usize,
    indent: usize,
    whitespace: WhitespaceHandling,
    current_line: String,
}

pub trait ModuleWriter<W>
where
    W: Write,
{
    #[allow(unused_variables)]
    fn write_module(&self, writer: &mut CodeWriter<W>, module: &Module) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("module".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_sub_module(&self, writer: &mut CodeWriter<W>, module: &Module) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("module".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_import(&self, writer: &mut CodeWriter<W>, import: &Import) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("import".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_comment(&self, writer: &mut CodeWriter<W>, comment: &Comment) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("comment".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_structured_type(
        &self,
        writer: &mut CodeWriter<W>,
        structure: &StructuredType,
    ) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("record".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_enumeration(
        &self,
        writer: &mut CodeWriter<W>,
        enumeration: &Enumeration,
    ) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("enumeration".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_constant(&self, writer: &mut CodeWriter<W>, constant: &NamedValue) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("constant".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_variable(&self, writer: &mut CodeWriter<W>, variable: &NamedValue) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("variable".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_function_decl(
        &self,
        writer: &mut CodeWriter<W>,
        function_decl: &FunctionDecl,
    ) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("function_decl".to_string()).into())
    }

    #[allow(unused_variables)]
    fn write_type_alias(&self, writer: &mut CodeWriter<W>, type_alias: &TypeAlias) -> Result<()> {
        Err(ErrorKind::UnsupportedElementKind("type_alias".to_string()).into())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub type NamespaceFileFn = Box<dyn Fn(&Namespace, &Path) -> PathBuf>;

pub type ModuleFileWriterFn = Box<dyn Fn() -> Box<dyn ModuleWriter<File>>>;

pub fn write_modules(
    top_module: Module,
    fs_root: &Path,
    namespace_to_file: &impl Fn(&Namespace, &Path) -> PathBuf,
    module_writer: &impl Fn() -> Box<dyn ModuleWriter<File>>,
) -> Result<()> {
    write_a_module(
        &Namespace::new(vec![top_module.name().clone()]),
        fs_root,
        &top_module,
        &namespace_to_file,
        &module_writer,
    )
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NewLine {
    #[cfg(not(windows))]
    fn default() -> Self {
        Self::LineFeed
    }
    #[cfg(windows)]
    fn default() -> Self {
        Self::CarriageReturnLineFeed
    }
}

impl Display for NewLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NewLine::LineFeed => "\n",
                NewLine::CarriageReturnLineFeed => "\r'n",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for WhitespaceHandling {
    fn default() -> Self {
        Self {
            indent: "    ".to_string(),
            new_line: Default::default(),
            trim_trailing: true,
        }
    }
}

impl WhitespaceHandling {
    pub fn indent(&mut self, indent: &str) -> &mut Self {
        self.indent = indent.to_string();
        self
    }

    pub fn new_line(&mut self, new_line: NewLine) -> &mut Self {
        self.new_line = new_line;
        self
    }

    pub fn trim_trailing(&mut self) -> &mut Self {
        self.trim_trailing = true;
        self
    }

    pub fn no_trim_trailing(&mut self) -> &mut Self {
        self.trim_trailing = false;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for BlockPlacement {
    fn default() -> Self {
        Self::OwnLine
    }
}

// ------------------------------------------------------------------------------------------------

impl Block {
    pub fn new(
        open: &str,
        open_placement: BlockPlacement,
        close: &str,
        close_placement: BlockPlacement,
    ) -> Self {
        Self {
            open: open.to_string(),
            open_placement,
            close: close.to_string(),
            close_placement,
        }
    }

    pub fn open_with(&self) -> (&String, &BlockPlacement) {
        (&self.open, &self.open_placement)
    }

    pub fn close_with(&self) -> (&String, &BlockPlacement) {
        (&self.close, &self.close_placement)
    }
}

// ------------------------------------------------------------------------------------------------

impl<W> CodeWriter<W>
where
    W: Write,
{
    pub fn new(w: W) -> Self {
        Self {
            writer: RefCell::new(w),
            line_count: 0,
            indent: 0,
            whitespace: Default::default(),
            current_line: String::default(),
        }
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn outdent(&mut self) {
        self.indent -= 1;
    }

    pub fn open_block(&mut self, style: &Block) -> Result<()> {
        let (block_marker, placement) = style.open_with();
        match placement {
            BlockPlacement::Trailing => {
                self.open_inline_block(style)?;
            }
            BlockPlacement::TrailingNewLine => {
                self.write_str(" ")?;
                self.write_str(block_marker)?;
                self.new_line()?;
                self.indent();
            }
            BlockPlacement::OwnLine => {
                self.new_line()?;
                self.write_str(block_marker)?;
                self.new_line()?;
                self.indent();
            }
            BlockPlacement::OwnLineIndented => {
                self.new_line()?;
                self.indent();
                self.write_str(block_marker)?;
                self.new_line()?;
                self.indent();
            }
        }
        Ok(())
    }

    pub fn open_inline_block(&mut self, style: &Block) -> Result<()> {
        let (block_marker, _) = style.open_with();
        self.write_str(" ")?;
        self.write_str(block_marker)?;
        self.write_str(" ")
    }

    pub fn close_block(&mut self, style: &Block) -> Result<()> {
        let (block_marker, placement) = style.close_with();
        match placement {
            BlockPlacement::Trailing => {
                self.close_inline_block(style)?;
            }
            BlockPlacement::TrailingNewLine => {
                self.new_line()?;
                self.outdent();
                self.write_str(" ")?;
                self.write_str(block_marker)?;
            }
            BlockPlacement::OwnLine => {
                self.new_line()?;
                self.outdent();
                self.write_str(block_marker)?;
                self.new_line()?;
            }
            BlockPlacement::OwnLineIndented => {
                self.new_line()?;
                self.outdent();
                self.write_str(block_marker)?;
                self.new_line()?;
                self.outdent();
            }
        }
        Ok(())
    }

    pub fn close_inline_block(&mut self, style: &Block) -> Result<()> {
        let (block_marker, _) = style.close_with();
        self.write_str(" ")?;
        self.write_str(block_marker)
    }

    pub fn current_line_len(&mut self) -> usize {
        self.current_line.len()
    }

    pub fn write_str(&mut self, text: &str) -> Result<()> {
        if !text.is_empty() {
            let lines: Vec<&str> = text.split(&self.whitespace.new_line.to_string()).collect();
            let end = lines.len() - 1;
            for (i, s) in lines.iter().enumerate() {
                self.write_no_newline(s);
                if i < end || (i == end && text.ends_with(&self.whitespace.new_line.to_string())) {
                    self.new_line()?;
                }
            }
        }
        Ok(())
    }

    pub fn space(&mut self) {
        self.current_line.push(' ');
    }

    pub fn blank_line(&mut self) -> Result<()> {
        if !self.current_line.trim().is_empty() {
            self.write_current_line()?;
        }
        write!(self.writer.borrow_mut(), "{}", self.whitespace.new_line)?;
        Ok(())
    }

    pub fn new_line(&mut self) -> Result<()> {
        self.write_current_line()?;
        write!(self.writer.borrow_mut(), "{}", self.whitespace.new_line)?;
        self.line_count += 1;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.write_current_line()
    }

    pub fn current_position(&self) -> (usize, usize) {
        (
            self.line_count,
            (self.indent * self.whitespace.indent.len()) + self.current_line.len(),
        )
    }

    // --------------------------------------------------------------------------------------------

    fn write_current_line(&mut self) -> Result<()> {
        self.write_current_indentation()?;
        self.write_current_line_no_indent()
    }

    fn write_no_newline(&mut self, text: &str) {
        self.current_line.push_str(text);
    }

    fn write_current_indentation(&mut self) -> Result<()> {
        for _ in 0..self.indent {
            write!(self.writer.borrow_mut(), "{}", self.whitespace.indent)?;
        }
        Ok(())
    }

    fn write_current_line_no_indent(&mut self) -> Result<()> {
        if self.whitespace.trim_trailing {
            let _ = self.current_line.trim_end();
        }
        write!(self.writer.borrow_mut(), "{}", self.current_line)?;
        self.current_line.clear();
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_a_module(
    current_namespace: &Namespace,
    current_directory: &Path,
    module: &Module,
    namespace_to_file: &impl Fn(&Namespace, &Path) -> PathBuf,
    module_writer: &impl Fn() -> Box<dyn ModuleWriter<File>>,
) -> Result<()> {
    let file_path = namespace_to_file(current_namespace, current_directory);
    let file = File::open(&file_path)?;
    let mut writer = CodeWriter::new(file);
    let visitor = module_writer();
    visitor.write_module(&mut writer, &module)?;
    writer.blank_line()?;
    for element in module.content() {
        match element {
            ModuleContent::Import(v) => {
                visitor.write_import(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Comment(v) => {
                visitor.write_comment(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Structure(v) => {
                visitor.write_structured_type(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Constant(v) => {
                visitor.write_constant(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Variable(v) => {
                visitor.write_variable(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Function(v) => {
                visitor.write_function_decl(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Alias(v) => {
                visitor.write_type_alias(&mut writer, v)?;
                writer.blank_line()
            }
            ModuleContent::Module(v) => {
                visitor.write_sub_module(&mut writer, v)?;
                writer.blank_line()?;
                // TODO: this won't work
                write_a_module(
                    &current_namespace.with(v.name().clone()),
                    &file_path,
                    v,
                    namespace_to_file,
                    module_writer,
                )
            }
        }?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

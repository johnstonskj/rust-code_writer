/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::model::identity::Namespace;
use crate::model::{
    Comment, Enumeration, FunctionDecl, HasDocumentation, HasName, HasOptionalType,
    HasOptionalValue, HasProperties, HasType, HasValue, HasVisibility, Import, ImportItem,
    IsOptional, KnownType, Module, NamedValue, StructuredType, StructuredTypeKind, TypeAlias,
    Value, ValueType, Visibility,
};
use crate::writer::{CodeWriter, ModuleWriter};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct RustWriter {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// `Fn(&Namespace, &Path) -> PathBuf`
pub fn namespace_to_file_path(namespace: &Namespace, current_path: &Path) -> PathBuf {
    PathBuf::from(current_path)
}

/// `Fn() -> Box<dyn ModuleWriter<File>>`
pub fn rust_module_writer() -> Box<dyn ModuleWriter<File>> {
    Box::new(RustWriter::default())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for RustWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<W> ModuleWriter<W> for RustWriter
where
    W: Write,
{
    fn write_module(&self, writer: &mut CodeWriter<W>, module: &Module) -> Result<()> {
        if let Some(documentation) = module.documentation() {
            self.write_block_comment(writer, "/*!", "*/", documentation)?;
        }
        Ok(())
    }

    fn write_sub_module(&self, writer: &mut CodeWriter<W>, module: &Module) -> Result<()> {
        self.write_visibility(writer, module)?;
        writer.write_str(&format!("mod {};", module.name()))?;
        writer.new_line()
    }

    fn write_import(&self, writer: &mut CodeWriter<W>, import: &Import) -> Result<()> {
        assert!(!import.items().is_empty());
        self.write_visibility(writer, import)?;
        writer.write_str(&format!("use {}::", import.namespace().join("::")))?;
        if import.items().len() == 1 {
            writer.write_str(&format!(
                "{};",
                import_item(import.items().iter().next().unwrap())
            ))?;
        } else {
            writer.write_str(&format!(
                "{{{}}};",
                import
                    .items()
                    .iter()
                    .map(import_item)
                    .collect::<Vec<String>>()
                    .join(", ")
            ))?;
        }
        writer.new_line()
    }

    fn write_comment(&self, writer: &mut CodeWriter<W>, comment: &Comment) -> Result<()> {
        if comment.is_line() {
            self.write_line_comment(writer, "//", comment.text())
        } else {
            self.write_block_comment(writer, "/*", "*/", comment.text())
        }
    }

    fn write_structured_type(
        &self,
        writer: &mut CodeWriter<W>,
        record: &StructuredType,
    ) -> Result<()> {
        match record.kind() {
            StructuredTypeKind::Structure => self.write_structure(writer, record, "struct"),
            StructuredTypeKind::Union => self.write_structure(writer, record, "union"),
            StructuredTypeKind::Exception => self.write_exception(writer, record),
            StructuredTypeKind::Class => self.write_structure(writer, record, "class"),
            StructuredTypeKind::Interface => self.write_structure(writer, record, "interface"),
            StructuredTypeKind::Service => self.write_structure(writer, record, "service"),
        }
    }

    fn write_enumeration(
        &self,
        writer: &mut CodeWriter<W>,
        enumeration: &Enumeration,
    ) -> Result<()> {
        self.write_documentation(writer, enumeration)?;
        self.write_attributes(writer, enumeration, false)?;
        self.write_visibility(writer, enumeration)?;
        writer.write_str(&format!("enum {} {{", enumeration.name()))?;
        if !enumeration.variants().is_empty() {
            writer.new_line()?;
            writer.indent();
            for member in enumeration.variants() {
                self.write_documentation(writer, member)?;
                self.write_attributes(writer, member, false)?;
                writer.write_str(&format!(
                    "{}{},",
                    member.name(),
                    match &member.value_type() {
                        None => String::new(),
                        Some(v) => format!("({})", self.value_type(v, false)),
                    }
                ))?;
                writer.new_line()?;
            }
            writer.outdent();
        }

        writer.write_str("}")?;
        writer.new_line()
    }

    fn write_constant(&self, writer: &mut CodeWriter<W>, constant: &NamedValue) -> Result<()> {
        self.write_documentation(writer, constant)?;
        self.write_attributes(writer, constant, false)?;
        self.write_visibility(writer, constant)?;
        writer.write_str(&format!(
            "const {}: {} = {};",
            constant.name(),
            self.value_type(&constant.value_type(), false),
            self.value(&constant.value(), false),
        ))?;
        writer.new_line()
    }

    fn write_variable(&self, writer: &mut CodeWriter<W>, variable: &NamedValue) -> Result<()> {
        self.write_documentation(writer, variable)?;
        self.write_attributes(writer, variable, false)?;
        writer.write_str(&format!(
            "let {}: {} = {};",
            variable.name(),
            self.value_type(variable.value_type(), false),
            self.value(&variable.value(), false),
        ))?;
        writer.new_line()
    }

    fn write_function_decl(
        &self,
        writer: &mut CodeWriter<W>,
        function_decl: &FunctionDecl,
    ) -> Result<()> {
        self.write_documentation(writer, function_decl)?;
        self.write_attributes(writer, function_decl, false)?;
        self.write_visibility(writer, function_decl)?;
        writer.write_str(&format!(
            "fn {}({}){}",
            function_decl.name(),
            &function_decl
                .parameters()
                .iter()
                .map(|p| format!(
                    "{}: {}",
                    p.name(),
                    self.value_type(&p.value_type(), p.is_optional())
                ))
                .collect::<Vec<String>>()
                .join(", "),
            match &function_decl.value_type() {
                None => String::new(),
                Some(vt) => format!(": {}", self.value_type(vt, false)),
            },
        ))?;
        writer.write_str(";")?;
        writer.new_line()
    }

    fn write_type_alias(&self, writer: &mut CodeWriter<W>, type_alias: &TypeAlias) -> Result<()> {
        self.write_visibility(writer, type_alias)?;
        writer.write_str(&format!(
            "type {} = {};",
            type_alias.name(),
            self.value_type(&type_alias.value_type(), false)
        ))?;
        writer.new_line()
    }
}

impl RustWriter {
    fn write_structure<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        record: &StructuredType,
        kind: &'static str,
    ) -> Result<()> {
        self.write_documentation(writer, record)?;
        self.write_attributes(writer, record, false)?;
        self.write_visibility(writer, record)?;
        writer.write_str(&format!("{} {} {{", kind, record.name()))?;
        if !record.fields().is_empty() {
            writer.new_line()?;
            writer.indent();
            for member in record.fields() {
                self.write_documentation(writer, member)?;
                self.write_attributes(writer, member, false)?;
                self.write_visibility(writer, record)?;
                writer.write_str(&format!(
                    "{}: {},",
                    member.name(),
                    self.value_type(&member.value_type(), member.is_optional()),
                ))?;
                writer.new_line()?;
            }
            writer.outdent();
        }

        writer.write_str("}")?;
        writer.new_line()
    }

    fn write_exception<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        record: &StructuredType,
    ) -> Result<()> {
        self.write_structure(writer, record, "struct")
    }

    fn write_line_comment<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        prefix: &str,
        text: &str,
    ) -> Result<()> {
        let line_length = writer.current_line_len();
        for line in text.split("\n") {
            for _ in 0..line_length {
                writer.space();
            }
            writer.write_str(prefix)?;
            writer.space();
            writer.write_str(line)?;
            writer.new_line()?;
        }
        Ok(())
    }

    fn write_block_comment<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        start: &str,
        end: &str,
        text: &str,
    ) -> Result<()> {
        writer.write_str(start)?;
        writer.new_line()?;
        for line in text.split("\n") {
            writer.write_str(line)?;
            writer.new_line()?;
        }
        writer.write_str(end)?;
        writer.new_line()
    }

    fn write_visibility<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        item: &dyn HasVisibility,
    ) -> Result<()> {
        writer.write_str(match item.visibility() {
            Some(Visibility::Private) => "",
            Some(Visibility::Local) => "pub(super) ",
            Some(Visibility::Package) => "pub(crate) ",
            Some(Visibility::Public) => "pub ",
            _ => "",
        })
    }

    fn write_documentation<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        item: &dyn HasDocumentation,
    ) -> Result<()> {
        if let Some(documentation) = item.documentation() {
            self.write_block_comment(writer, "/**", "*/", documentation)?;
        }
        Ok(())
    }

    fn write_attributes<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        item: &dyn HasProperties,
        is_module: bool,
    ) -> Result<()> {
        for property in item.properties() {
            if is_module {
                writer.write_str("#![")?;
            } else {
                writer.write_str("#[")?;
            }
            writer.write_str(property.name().as_ref())?;
            if let Some(value) = property.value() {
                writer.write_str(&self.value(value, true))?;
            }
            writer.write_str("]")?;
            writer.new_line()?;
        }
        Ok(())
    }

    fn value(&self, value: &Value, is_attribute: bool) -> String {
        match value {
            Value::I8(v) => v.to_string(),
            Value::U8(v) => v.to_string(),
            Value::I16(v) => v.to_string(),
            Value::U16(v) => v.to_string(),
            Value::I32(v) => v.to_string(),
            Value::U32(v) => v.to_string(),
            Value::I64(v) => v.to_string(),
            Value::U64(v) => v.to_string(),
            Value::F32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Char(v) => v.to_string(),
            Value::String(v) => v.to_string(),
            Value::Values(vs) => {
                if is_attribute {
                    format!(
                        "({})",
                        vs.iter()
                            .map(|v| self.value(v, is_attribute))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    format!(
                        "[{}]",
                        vs.iter()
                            .map(|v| self.value(v, is_attribute))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Value::NamedValues(vs) => {
                if is_attribute {
                    format!(
                        "({})",
                        vs.iter()
                            .map(|(k, v)| format!(
                                "{}: {}",
                                self.value(k, is_attribute),
                                self.value(v, is_attribute)
                            ))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    format!(
                        "{{{}}}",
                        vs.iter()
                            .map(|(k, v)| format!(
                                "{} = {}",
                                self.value(k, is_attribute),
                                self.value(v, is_attribute)
                            ))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Value::Identifier(v) => v.to_string(),
        }
    }

    fn value_type(&self, value_type: &ValueType, optional: bool) -> String {
        let initial = match value_type {
            ValueType::Known(kt) => match kt {
                KnownType::I8 => "i8",
                KnownType::U8 => "u8",
                KnownType::I16 => "i16",
                KnownType::U16 => "u16",
                KnownType::I32 => "i32",
                KnownType::U32 => "u32",
                KnownType::I64 => "i64",
                KnownType::U64 => "u64",
                KnownType::F32 => "f32",
                KnownType::F64 => "f64",
                KnownType::Boolean => "bool",
                KnownType::Char => "char",
                KnownType::String => "String",
            }
            .to_string(),
            ValueType::Reference(t) => t.to_string(),
            ValueType::Array(t) => {
                format!("Vec<{}>", self.value_type(&t, false))
            }
            ValueType::Set(t) => format!("HashSet<{}>", self.value_type(&t, false)),
            ValueType::Map(kt, vt) => format!(
                "HashMap<{}, {}>",
                self.value_type(&kt, false),
                self.value_type(&vt, false)
            ),
            ValueType::Constrained(t, tc) => {
                assert!(!tc.is_empty());
                format!(
                    "{}: {}",
                    t,
                    tc.iter()
                        .map(|t| self.value_type(t, false))
                        .collect::<Vec<String>>()
                        .join(" + ")
                )
            }
            ValueType::Generic(t, gt) => {
                assert!(!gt.is_empty());
                format!(
                    "{}<{}>",
                    t,
                    gt.iter()
                        .map(|t| self.value_type(t, false))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ValueType::Function(pt, rt) => {
                format!(
                    "fn({}){}",
                    pt.iter()
                        .map(|vt| self.value_type(vt, false))
                        .collect::<Vec<String>>()
                        .join(", "),
                    match rt {
                        None => String::new(),
                        Some(rt) => format!(" -> {}", self.value_type(&rt, false)),
                    }
                )
            }
        };
        if optional {
            format!("Option<{}>", initial)
        } else {
            initial
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn import_item(ii: &ImportItem) -> String {
    format!(
        "{}{}",
        ii.name(),
        match &ii.alias() {
            None => String::new(),
            Some(a) => format!(" as {}", a),
        }
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

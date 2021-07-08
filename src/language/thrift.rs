/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::model::{
    Comment, Enumeration, FunctionDecl, HasDocumentation, HasName, HasOptionalType, HasType,
    HasValue, Import, IsOptional, KnownType, Module, NamedValue, StructuredType,
    StructuredTypeKind, TypeAlias, Value, ValueType,
};
use crate::writer::{CodeWriter, ModuleWriter};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct ThriftWriter {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ThriftWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<W> ModuleWriter<W> for ThriftWriter
where
    W: Write,
{
    fn write_module(&self, writer: &mut CodeWriter<W>, module: &Module) -> Result<()> {
        if let Some(documentation) = module.documentation() {
            self.write_block_comment(writer, "/*", "*/", documentation)?;
        }
        writer.blank_line()
    }

    fn write_import(&self, writer: &mut CodeWriter<W>, import: &Import) -> Result<()> {
        assert!(!import.items().is_empty());
        writer.write_str(&format!("include \"{}\"", import.namespace().join("/")))?;
        writer.new_line()
    }

    fn write_comment(&self, writer: &mut CodeWriter<W>, comment: &Comment) -> Result<()> {
        if comment.is_line() {
            self.write_line_comment(writer, "#", comment.text())
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
            StructuredTypeKind::Exception => self.write_structure(writer, record, "exception"),
            StructuredTypeKind::Class => self.write_structure(writer, record, "struct"),
            StructuredTypeKind::Interface => self.write_structure(writer, record, "struct"),
            StructuredTypeKind::Service => self.write_structure(writer, record, "service"),
        }
    }

    fn write_enumeration(
        &self,
        writer: &mut CodeWriter<W>,
        enumeration: &Enumeration,
    ) -> Result<()> {
        writer.write_str(&format!("enum {} {{", enumeration.name()))?;
        if !enumeration.variants().is_empty() {
            writer.new_line()?;
            writer.indent();
            for variant in enumeration.variants() {
                writer.write_str(&format!("{},", variant.name()))?;
                writer.new_line()?;
            }
            writer.outdent();
        }
        writer.write_str("}")?;
        writer.new_line()
    }

    fn write_constant(&self, writer: &mut CodeWriter<W>, named_value: &NamedValue) -> Result<()> {
        writer.write_str(&format!(
            "const {} {} = {}",
            value_type_string(named_value.value_type()),
            named_value.name(),
            value_str(named_value.value()),
        ))?;
        writer.new_line()
    }

    fn write_function_decl(
        &self,
        writer: &mut CodeWriter<W>,
        function_decl: &FunctionDecl,
    ) -> Result<()> {
        self.write_function_head(writer, function_decl)?;
        writer.write_str(";")?;
        writer.new_line()
    }

    fn write_type_alias(&self, writer: &mut CodeWriter<W>, type_alias: &TypeAlias) -> Result<()> {
        writer.write_str(&format!(
            "typedef {} {}",
            value_type_string(&type_alias.value_type()),
            type_alias.name(),
        ))?;
        writer.new_line()
    }
}

impl ThriftWriter {
    fn write_structure<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        record: &StructuredType,
        kind: &'static str,
    ) -> Result<()> {
        writer.write_str(&format!("{} {} {{", kind, record.name()))?;
        if !record.fields().is_empty() {
            writer.new_line()?;
            writer.indent();
            for (i, member) in record.fields().iter().enumerate() {
                writer.write_str(&format!(
                    "{}: {} {} {},",
                    i + 1,
                    if member.is_optional() {
                        "optional"
                    } else {
                        "required"
                    },
                    value_type_string(member.value_type()),
                    member.name(),
                ))?;
                writer.new_line()?;
            }
            writer.outdent();
        }

        writer.write_str("}")?;
        writer.new_line()
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

    fn write_function_head<W: Write>(
        &self,
        writer: &mut CodeWriter<W>,
        function_decl: &FunctionDecl,
    ) -> Result<()> {
        writer.write_str(&format!(
            "{} {}({})",
            match &function_decl.value_type() {
                None => String::from("void"),
                Some(vt) => value_type_string(vt),
            },
            function_decl.name(),
            &function_decl
                .parameters()
                .iter()
                .map(|p| format!("{} {}", value_type_string(p.value_type()), p.name(),))
                .collect::<Vec<String>>()
                .join(", "),
        ))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn value_type_string(vt: &ValueType) -> String {
    match vt {
        ValueType::Known(kt) => match kt {
            KnownType::I8 => "i8",
            KnownType::U8 => "byte",
            KnownType::I16 | KnownType::U16 => "i16",
            KnownType::I32 | KnownType::U32 => "i32",
            KnownType::I64 | KnownType::U64 => "i64",
            KnownType::F32 | KnownType::F64 => "double",
            KnownType::Boolean => "boolean",
            KnownType::Char => "i8",
            KnownType::String => "string",
        }
        .to_string(),
        ValueType::Reference(t) => t.to_string(),
        ValueType::Array(t) => {
            format!("list<{}>", value_type_string(&t))
        }
        ValueType::Set(t) => format!("set<{}>", value_type_string(&t)),
        ValueType::Map(kt, vt) => format!(
            "map<{}, {}>",
            value_type_string(&kt),
            value_type_string(&vt)
        ),
        ValueType::Constrained(t, tc) => {
            assert!(!tc.is_empty());
            format!(
                "{}: {}",
                t,
                tc.iter()
                    .map(value_type_string)
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
                    .map(value_type_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        ValueType::Function(pt, rt) => {
            format!(
                "fn({}){}",
                pt.iter()
                    .map(value_type_string)
                    .collect::<Vec<String>>()
                    .join(", "),
                match rt {
                    None => String::new(),
                    Some(rt) => format!(" -> {}", value_type_string(rt)),
                }
            )
        }
    }
}

fn value_str(value: &Value) -> String {
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
        Value::Values(vs) => format!(
            "[{}]",
            vs.iter()
                .map(|v| value_str(v))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Value::NamedValues(vs) => {
            format!(
                "{{{}}}",
                vs.iter()
                    .map(|(k, v)| format!("{}: {}", value_str(k), value_str(v)))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Value::Identifier(v) => v.to_string(),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

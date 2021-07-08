use code_writer::error::Result;
use code_writer::language::rust::RustWriter;
use code_writer::model::{
    Builder, Comment, Enumeration, Field, HasDocumentation, HasVisibility, Identifier, Import,
    IsOptional, KnownType, Module, ModuleContent, NamedValue, StructuredType, TypeAlias, ValueType,
    Visibility,
};
use code_writer::writer::{CodeWriter, ModuleWriter};

pub fn print_module(module: &Module) -> Result<()> {
    let mut out = std::io::stdout();
    let mut writer = CodeWriter::new(&mut out);

    let visitor = RustWriter::default();

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
                writer.write_str("------------------------------------------------------------------------------------------------")?;
                print_module(v)?;
                writer.write_str("------------------------------------------------------------------------------------------------")?;
                writer.blank_line()
            }
        }?;
    }
    Ok(())
}

pub fn write_code_model<W>(code: &mut CodeWriter<W>, writer: Box<dyn ModuleWriter<W>>)
where
    W: std::io::Write,
{
    writer
        .write_import(
            code,
            Import::new(vec![Identifier::new("std"), Identifier::new("io")].into())
                .item(Identifier::new("Write"))
                .item_with_alias(Identifier::new("Error"), Identifier::new("IoError")),
        )
        .unwrap();
    code.blank_line().unwrap();

    writer
        .write_comment(code, &Comment::line("hello world!"))
        .unwrap();
    code.blank_line().unwrap();

    writer
        .write_comment(code, &Comment::block("some more text"))
        .unwrap();

    writer
        .write_constant(
            code,
            &NamedValue::f64(Identifier::new("pi"), std::f64::consts::PI),
        )
        .unwrap();
    code.blank_line().unwrap();

    writer
        .write_structured_type(
            code,
            StructuredType::structure(Identifier::new("Address"))
                .set_visibility(Visibility::Package)
                .add_field(
                    Field::new(
                        Identifier::new("line_one"),
                        ValueType::Known(KnownType::String),
                    )
                    .required()
                    .build(),
                )
                .add_field(
                    Field::new(
                        Identifier::new("line_two"),
                        ValueType::Known(KnownType::String),
                    )
                    .optional()
                    .build(),
                )
                .add_field(
                    Field::new(Identifier::new("city"), ValueType::Known(KnownType::String))
                        .required()
                        .build(),
                )
                .add_field(
                    Field::new(
                        Identifier::new("state"),
                        ValueType::Known(KnownType::String),
                    )
                    .required()
                    .build(),
                )
                .add_field(
                    Field::new(Identifier::new("zip"), ValueType::Known(KnownType::String))
                        .required()
                        .build(),
                ),
        )
        .unwrap();
    code.blank_line().unwrap();

    writer
        .write_enumeration(
            code,
            Enumeration::new(Identifier::new("AddressType"))
                .set_documentation("the type, required by postal service.")
                .set_visibility(Visibility::Public)
                .add_named_variant(Identifier::new("Commercial"))
                .add_named_variant(Identifier::new("POBox"))
                .add_named_variant(Identifier::new("Residential")),
        )
        .unwrap();
    code.blank_line().unwrap();

    writer
        .write_type_alias(
            code,
            &TypeAlias::with_visibility(
                Visibility::Public,
                Identifier::new("AddrType"),
                ValueType::Reference(Identifier::new("AddressType")),
            ),
        )
        .unwrap();

    code.flush().unwrap();
}

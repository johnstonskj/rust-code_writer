use code_writer::language::rust::RustWriter;
use code_writer::writer::CodeWriter;

pub mod common;

#[test]
fn test_generate() {
    let mut out = std::io::stdout();
    let mut writer = CodeWriter::new(&mut out);
    let rust_writer = RustWriter::default();
    common::write_code_model(&mut writer, Box::new(rust_writer));
}

use code_writer::language::thrift::ThriftWriter;
use code_writer::writer::CodeWriter;

pub mod common;

#[test]
fn test_generate() {
    let mut out = std::io::stdout();
    let mut writer = CodeWriter::new(&mut out);
    let thrift_writer = ThriftWriter::default();
    common::write_code_model(&mut writer, Box::new(thrift_writer));
}

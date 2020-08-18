use crate::types::result::Report;
use crate::writer::ReportWriter;

pub struct ConsoleWriter;

impl ReportWriter for ConsoleWriter {
    fn write(r: &Report) {
        let as_json = serde_json::to_string(&r).unwrap();
        println!("{}", as_json);
    }
}

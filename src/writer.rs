use crate::types::result::Report;

pub trait ReportWriter {
    fn write(r: &Report);
}

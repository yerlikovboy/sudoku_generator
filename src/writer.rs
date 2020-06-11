use crate::job::result::Report;

pub trait ReportWriter {
    fn write(r: &Report);
}

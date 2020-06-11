use crate::job::result::Report;

pub struct FileWriter {
    path: String,
}

impl FileWriter for ReportWriter {
    fn write(&self, r: &Report) {}
}

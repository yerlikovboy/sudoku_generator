use crate::cmd;
use crate::types::result;

pub fn generate(cfg: &cmd::Config) -> result::Report {
    super::base::generate(cfg, &[0; 81])
}

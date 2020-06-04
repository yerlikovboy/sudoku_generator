use crate::job::result;

pub fn generate(n_iter: u32) -> result::Report {
    super::base::generate(n_iter, &[0; 81])
}

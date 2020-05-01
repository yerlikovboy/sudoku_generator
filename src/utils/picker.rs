use rand::Rng;

/// randomly pick a value from the given slice of candidates. Return None
/// if slice has length of zero.
pub fn pick<T: Copy>(c: &[T]) -> Option<T> {
    match c.len() {
        0 => None,
        1 => Some(c[0]),
        _ => {
            let v = rand::thread_rng().gen_range(0, c.len());
            Some(c[v])
        }
    }
}
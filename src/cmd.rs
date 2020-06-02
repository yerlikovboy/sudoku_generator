#[derive(Copy, Clone)]
pub enum Algorithm {
    Brute,
    Diagonal,
}

pub struct Config {
    n_iterations: u32,
    alg: Algorithm,
}

impl Config {
    pub fn new(n_iterations: u32, alg: Algorithm) -> Config {
        Config { n_iterations, alg }
    }

    pub fn n_iterations(&self) -> u32 {
        self.n_iterations
    }

    pub fn algorithm(&self) -> Algorithm {
        self.alg
    }
}

pub fn parse_args(args: Vec<String>) -> Result<Config, String> {
    if args.len() != 3 {
        return Err(String::from("not enough arguments"));
    }

    let alg_type = if args[1].as_str() == "diag" {
        Algorithm::Diagonal
    } else {
        Algorithm::Brute
    };
    let n_iterations = *&args[2].parse::<u32>().unwrap();

    Ok(Config::new(n_iterations, alg_type))
}

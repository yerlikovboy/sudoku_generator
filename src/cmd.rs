use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Algorithm {
    Brute,
    Diagonal,
}

pub struct Config {
    n_iterations: u32,
    alg: Algorithm,
    is_daemon: bool,
    frequency_secs: Option<u64>,
}

impl Config {
    fn default() -> Config {
        Config {
            alg: Algorithm::Diagonal,
            n_iterations: 50000,
            is_daemon: false,
            frequency_secs: None,
        }
    }

    fn with_alg(&self, alg: Algorithm) -> Config {
        Config { alg: alg, ..*self }
    }

    fn as_daemon(&self) -> Config {
        Config {
            is_daemon: true,
            ..*self
        }
    }

    fn with_freq(&self, freq: u64) -> Config {
        Config {
            frequency_secs: Some(freq),
            ..*self
        }
    }

    fn with_niter(&self, niter: u32) -> Config {
        Config {
            n_iterations: niter,
            ..*self
        }
    }

    pub fn n_iterations(&self) -> u32 {
        self.n_iterations
    }

    pub fn algorithm(&self) -> Algorithm {
        self.alg
    }

    pub fn is_daemon(&self) -> bool {
        self.is_daemon
    }

    pub fn frequency_secs(&self) -> u64 {
        self.frequency_secs.unwrap_or(10)
    }
}

pub fn parse_args(args: &Vec<String>) -> Result<Config, String> {
    let mut iter = args.iter();
    let mut cfg = Config::default();

    //move iterator one to skip the command (gmd or whatever) itself
    iter.next();

    while let Some(opt) = iter.next() {
        match opt.as_str() {
            "-d" => cfg = cfg.as_daemon(),
            "-f" => {
                if let Some(freq_val) = iter.next() {
                    cfg = cfg.with_freq(freq_val.parse().unwrap());
                } else {
                    return Err(String::from("missing frequency value"));
                }
            }
            "-i" => {
                if let Some(niter_val) = iter.next() {
                    cfg = cfg.with_niter(niter_val.parse().unwrap());
                } else {
                    return Err(String::from("missing frequency value"));
                }
            }
            "-a" => {
                if let Some(alg_val) = iter.next() {
                    match alg_val.as_str() {
                        "diag" => cfg = cfg.with_alg(Algorithm::Diagonal),
                        "brute" => cfg = cfg.with_alg(Algorithm::Brute),
                        _ => return Err(String::from("invalid value for algorithm")),
                    }
                } else {
                    return Err(String::from("missing algorithm"));
                }
            }

            _ => return Err(String::from("unknown option")),
        }
    }
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let tc = vec![
            (Config::new(20, Algorithm::Brute), (20, Algorithm::Brute)),
            (
                Config::new(10, Algorithm::Diagonal),
                (10, Algorithm::Diagonal),
            ),
        ];

        for t in tc {
            let c = t.0;
            let expected = t.1;

            if c.n_iterations() == expected.0 {
                let _expected_alg = expected.1;
                match c.algorithm() {
                    _expected_alg => (),
                    _ => return Err(String::from("algorithm different than expected")),
                }
            } else {
                return Err(String::from("n_iterations different than expected"));
            }
        }

        Ok(())
    }
}

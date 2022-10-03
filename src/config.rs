use std::error::Error;

/// Options for configuring the evolutionary algorithm.
/// Currently you can only configure population size and number of evaluations.
pub struct Config {
    pub file_name: String,
    pub pop_size: usize,
    pub num_evals: usize,
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) 
    -> Result<Self, &'static str> {
        let _ = args.next();
        let file_name = args.next()
            .ok_or_else(|| "file name not provided")?;
        let pop_size = args.next()
            .ok_or_else(|| "pop size not provided")?
            .parse::<usize>();
        if let Err(_) = pop_size {
            return Err("could not parse pop size");
        }

        let num_evals = args.next()
            .ok_or_else(|| "num evals not provided")?
            .parse::<usize>();
        if let Err(_) = num_evals {
            return Err("could not parse num evals");
        }

        let pop_size = pop_size.unwrap();
        let num_evals = num_evals.unwrap();
        assert!(pop_size > 1);
        assert!(num_evals > 0);
        Ok(Self {
            file_name,
            pop_size,
            num_evals,
        })
    }
}

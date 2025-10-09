use crate::{
    panic::{rust_backtrace, PanicError},
    Seed,
};
use core::fmt::{Debug, Display};

/// Contains information about a test failure
#[derive(Debug)]
pub struct Failure<Input> {
    pub error: PanicError,
    pub input: Input,
    pub seed: Option<Seed>,
    pub hide_error: bool,
}

impl<Input: Debug> Display for Failure<Input> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "\n======================== Test Failure ========================\n"
        )?;
        if let Some(seed) = &self.seed {
            writeln!(f, "BOLERO_RANDOM_SEED={seed}\n")?;
        }

        writeln!(f, "Input: \n{:#?}\n", self.input)?;

        if !self.hide_error {
            writeln!(f, "Error: \n{}", self.error)?;
            if f.alternate() {
                if let Some(backtrace) = self.error.backtrace.as_ref().filter(|_| rust_backtrace()) {
                    writeln!(f, "{backtrace}\n")?;
                } else {
                    writeln!(f, "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n")?;
                }
            }
        }

        write!(
            f,
            "=============================================================="
        )?;
        Ok(())
    }
}

impl<Input: Debug> std::error::Error for Failure<Input> {}

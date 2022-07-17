mod uniform_crossover;
pub use self::uniform_crossover::*;
use crate::{Chromosome, RngCore};

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        chromosome_a: &Chromosome,
        chromosome_b: &Chromosome,
    ) -> Chromosome;
}

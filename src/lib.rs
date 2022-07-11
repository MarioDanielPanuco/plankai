
mod selection;
mod chromosome;
mod mutation;
mod crossover;

use rand::prelude::SliceRandom;
use rand::RngCore;
use crate::chromosome::Chromosome;
use crate::crossover::CrossoverMethod;
use crate::mutation::MutationMethod;
use crate::selection::SelectionMethod;

#[derive(Debug, Clone)]
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

impl<S> GeneticAlgorithm<S>
    where 
        S: SelectionMethod {
    pub fn new(selection_method: S,
               crossover_method: impl CrossoverMethod + 'static,
               mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self,
                     rng: &mut dyn RngCore,
                     population: &[I],
    ) -> Vec<I>
        where
            I: Individual {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                let parent_b = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                let mut child = self
                    .crossover_method
                    .crossover(rng, parent_a, parent_b);
                // TODO mutation
                todo!()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::iter::FromIterator;

    mod ranked_selection {
        use super::*;
    }
}

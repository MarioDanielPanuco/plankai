use crate::selection::SelectionMethod;
use crate::*;

#[derive(Clone, Debug, Default)]
pub struct RouletteWheel;

impl SelectionMethod for RouletteWheel {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

use crate::selection::SelectionMethod;
use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Rank;

impl SelectionMethod for Rank {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
        where
            I: Individual,
    {
        todo!()
    }
}

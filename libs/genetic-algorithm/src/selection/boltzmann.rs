use crate::selection::SelectionMethod;
use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Boltzmann;

/*impl SelectionMethod for Boltzmann {
    fn select<'a, I>(&self, rng: &mut dyn RngCore,
                     population: &'a [I],
                     temperature: f32) -> &'a I
        where
            I: Individual,
    {
        todo!();
    }
}*/

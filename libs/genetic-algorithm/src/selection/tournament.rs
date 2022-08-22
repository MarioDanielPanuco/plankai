use crate::selection::SelectionMethod;
use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Tournament;

impl SelectionMethod for Tournament {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        let mut tournament_size = population.len() / 2;

        let mut best: Option<&I> = None;

        loop {
            let indiv = population.choose(rng);

            match best {
                Some(x) => {
                    if indiv.unwrap().fitness() > x.fitness() {
                        best = indiv;
                    }
                }
                None => best = indiv,
            }

            if tournament_size == 0 {
                return best.unwrap();
            }

            tournament_size -= 1;
        }
    }
}

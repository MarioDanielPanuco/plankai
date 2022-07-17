pub use self::roulette_wheel::*;

use crate::*;

mod boltzmann;
mod roulette_wheel;

/*
enum S_Param{
    Normal {rng: &mut dyn RngCore, population: &'a [I]},
    boltzmann {rng: &mut dyn RngCore, population: &'a [I], temperature: f32}
}
struct Selection_Parameters<'a, I> {
    rng: &mut dyn RngCore,
    population: &'a [I],
    temperature: Option<f32>,
}*/

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

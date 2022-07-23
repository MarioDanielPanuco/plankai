use crate::*;

pub struct AnimalIndividual {
    pub fitness: f32,
    pub chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn from_chromosome(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        let fitness_factors: f32 = animal.satiation as f32;

        Self {
            fitness: fitness_factors,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, config: &Config, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(config, rng, self.chromosome)
    }
}

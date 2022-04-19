use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::fitness::Fitness;
use crate::gene::Gene;
use crate::population::Population;
use std::fmt;

pub struct Permutate<T: Gene, F: Fitness<T>> {
    pub context: Context<T>,
    pub best_chromosome: Option<Chromosome<T>>,
    pub fitness: Option<F>,
    pub population: Population<T>,
}

impl<T: Gene, F: Fitness<T>> Permutate<T, F> {
    pub fn new(context: Context<T>) -> Self {
        Self {
            context: context,
            fitness: None,
            best_chromosome: None,
            population: Population::new_empty(),
        }
    }

    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }

    pub fn is_valid(&self) -> bool {
        self.fitness.is_some()
    }

    pub fn call(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        self.execute()
    }

    fn execute(mut self) -> Self {
        let fitness = self.fitness.as_ref().cloned().unwrap();

        self.population = self.context.permutation_population_factory();
        self.population = fitness.call_for_population(self.population);
        self.update_best_chromosome();
        self
    }

    fn update_best_chromosome(&mut self) {
        if self.best_chromosome.as_ref() < self.population.best_chromosome() {
            self.best_chromosome = self.population.best_chromosome().cloned();
        }
    }

    fn best_fitness_score(&self) -> Option<usize> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<T: Gene, F: Fitness<T>> fmt::Display for Permutate<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "permutate:\n")?;
        write!(f, "  fitness: {:?}\n", self.fitness.as_ref())?;
        write!(f, "  population size: {:?}\n", self.population.size())?;
        write!(f, "  best fitness score: {:?}\n", self.best_fitness_score())?;
        write!(
            f,
            "  best_chromosome: {:?}\n",
            self.best_chromosome.as_ref()
        )
    }
}

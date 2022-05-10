use genetic_algorithm::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::{thread, time};

pub type MicroSeconds = u64;

#[derive(Clone, Debug)]
pub struct ExpensiveCount {
    pub micro_seconds: MicroSeconds,
}
impl ExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds) -> Self {
        Self { micro_seconds }
    }
}
impl Fitness for ExpensiveCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        thread::sleep(time::Duration::from_micros(self.micro_seconds));
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

#[derive(Clone, Debug)]
pub struct CachedExpensiveCount {
    pub micro_seconds: MicroSeconds,
    pub cache: HashMap<GenesKey, FitnessValue>,
}
impl CachedExpensiveCount {
    pub fn new(micro_seconds: MicroSeconds, initial_cache_cap: usize) -> Self {
        Self {
            micro_seconds,
            cache: HashMap::with_capacity(initial_cache_cap),
        }
    }
}
impl Fitness for CachedExpensiveCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        //print!("cache try ({}), ", self.cache.len());
        Some(
            *self.cache.entry(chromosome.genes_key()).or_insert_with(|| {
                //println!("miss");
                thread::sleep(time::Duration::from_micros(self.micro_seconds));
                chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue
            }),
        )
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_gene_size(100)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(100)
        .with_mutate(MutateOnce(0.05))
        //.with_fitness(ExpensiveCount::new(1000))
        .with_fitness(CachedExpensiveCount::new(1000, 5000))
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}

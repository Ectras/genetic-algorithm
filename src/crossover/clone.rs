use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Children are clones of the parents, effectively doubling the population if you keep the parents.
/// Acts as no-op if the parents are not kept.
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone {
    pub keep_parent: bool,
}
impl Crossover for Clone {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let population_size = state.population.size();
        if self.keep_parent {
            state
                .population
                .chromosomes
                .extend_from_within(..population_size);
        };
        state
            .population
            .chromosomes
            .iter_mut()
            .take(population_size)
            .for_each(|c| c.age = 0);

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl Clone {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}

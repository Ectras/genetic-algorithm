use super::{Crossover, KeepParent};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

/// Children are clones of the parents, effectively doubling the population if you keep the parents.
/// Acts as no-op if the parents are not kept.
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone(pub KeepParent);
impl Crossover for Clone {
    fn call<T: Genotype, R: Rng>(
        &self,
        _genotype: &T,
        mut population: Population<T>,
        _rng: &mut R,
    ) -> Population<T> {
        if self.0 {
            let mut clones = population.clone();
            population.merge(&mut clones);
            population
        } else {
            population
        }
    }

    fn allow_unique_genotype(&self) -> bool {
        true
    }
}

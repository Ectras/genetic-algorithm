//! The crossover phase where every two parent chromosomes create two children chromosomes. The
//! [competition](crate::compete) phase determines the order and the amount of the parent pairing
//! (overall with fitter first). If you choose to a keep a percentage of the top parents, the
//! parents will compete with their own children and the population is temporarily overbooked and
//! part of it will be discarded in the [competition](crate::compete) phase of the next generation.
mod clone;
mod multi_gene;
mod multi_point;
mod par_multi_point;
mod single_gene;
mod single_point;
mod uniform;
mod wrapper;

pub use self::clone::Clone as CrossoverClone;
pub use self::multi_gene::MultiGene as CrossoverMultiGene;
pub use self::multi_point::MultiPoint as CrossoverMultiPoint;
pub use self::par_multi_point::ParMultiPoint as CrossoverParMultiPoint;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;
pub use self::wrapper::Wrapper as CrossoverWrapper;

use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool {
        false
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        false
    }
}

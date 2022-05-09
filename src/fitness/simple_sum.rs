use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{
    ContinuousGenotype, DiscreteGenotype, Genotype, MultiDiscreteGenotype, UniqueDiscreteGenotype,
};

#[derive(Clone, Debug)]
pub struct SimpleSumContinuousGenotype;
impl Fitness for SimpleSumContinuousGenotype {
    type Genotype = ContinuousGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumDiscreteGenotype;
impl Fitness for SimpleSumDiscreteGenotype {
    type Genotype = DiscreteGenotype<usize>;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumUniqueDiscreteGenotype;
impl Fitness for SimpleSumUniqueDiscreteGenotype {
    type Genotype = UniqueDiscreteGenotype<usize>;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumMultiDiscreteGenotype;
impl Fitness for SimpleSumMultiDiscreteGenotype {
    type Genotype = MultiDiscreteGenotype<usize>;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

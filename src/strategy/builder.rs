use crate::crossover::Crossover;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::{EvolveGenotype, IncrementalGenotype, PermutableGenotype};
use crate::mutate::Mutate;
use crate::select::Select;
use crate::strategy::evolve::EvolveBuilder;
use crate::strategy::hill_climb::HillClimbBuilder;
use crate::strategy::permutate::PermutateBuilder;
use crate::strategy::{Strategy, StrategyReporter, StrategyReporterNoop, StrategyVariant};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for a Strategy struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: EvolveGenotype + IncrementalGenotype + PermutableGenotype,
    M: Mutate,
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Select,
    E: Extension,
    SR: StrategyReporter<Genotype = G>,
> {
    pub crossover: Option<S>,
    pub extension: E,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub genotype: Option<G>,
    pub max_chromosome_age: Option<usize>,
    pub max_stale_generations: Option<usize>,
    pub mutate: Option<M>,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub reporter: SR,
    pub rng_seed: Option<u64>,
    pub select: Option<C>,
    pub target_fitness_score: Option<FitnessValue>,
    pub target_population_size: usize,
    pub valid_fitness_score: Option<FitnessValue>,
}

impl<
        G: EvolveGenotype + IncrementalGenotype + PermutableGenotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Select,
    > Default for Builder<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            target_population_size: 0,
            max_stale_generations: None,
            max_chromosome_age: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: false,
            mutate: None,
            fitness: None,
            crossover: None,
            select: None,
            extension: ExtensionNoop::new(),
            reporter: StrategyReporterNoop::new(),
            rng_seed: None,
        }
    }
}
impl<
        G: EvolveGenotype + IncrementalGenotype + PermutableGenotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Select,
    > Builder<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>>
{
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: EvolveGenotype + IncrementalGenotype + PermutableGenotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_target_population_size(mut self, target_population_size: usize) -> Self {
        self.target_population_size = target_population_size;
        self
    }
    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = Some(max_stale_generations);
        self
    }
    pub fn with_max_stale_generations_option(
        mut self,
        max_stale_generations_option: Option<usize>,
    ) -> Self {
        self.max_stale_generations = max_stale_generations_option;
        self
    }
    pub fn with_max_chromosome_age(mut self, max_chromosome_age: usize) -> Self {
        self.max_chromosome_age = Some(max_chromosome_age);
        self
    }
    pub fn with_max_chromosome_age_option(
        mut self,
        max_chromosome_age_option: Option<usize>,
    ) -> Self {
        self.max_chromosome_age = max_chromosome_age_option;
        self
    }
    pub fn with_target_fitness_score(mut self, target_fitness_score: FitnessValue) -> Self {
        self.target_fitness_score = Some(target_fitness_score);
        self
    }
    pub fn with_target_fitness_score_option(
        mut self,
        target_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.target_fitness_score = target_fitness_score_option;
        self
    }
    pub fn with_valid_fitness_score(mut self, valid_fitness_score: FitnessValue) -> Self {
        self.valid_fitness_score = Some(valid_fitness_score);
        self
    }
    pub fn with_valid_fitness_score_option(
        mut self,
        valid_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.valid_fitness_score = valid_fitness_score_option;
        self
    }
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
    }
    pub fn with_par_fitness(mut self, par_fitness: bool) -> Self {
        self.par_fitness = par_fitness;
        self
    }
    pub fn with_replace_on_equal_fitness(mut self, replace_on_equal_fitness: bool) -> Self {
        self.replace_on_equal_fitness = replace_on_equal_fitness;
        self
    }
    pub fn with_mutate(mut self, mutate: M) -> Self {
        self.mutate = Some(mutate);
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_crossover(mut self, crossover: S) -> Self {
        self.crossover = Some(crossover);
        self
    }
    pub fn with_select(mut self, select: C) -> Self {
        self.select = Some(select);
        self
    }
    pub fn with_extension<E2: Extension>(self, extension: E2) -> Builder<G, M, F, S, C, E2, SR> {
        Builder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_chromosome_age: self.max_chromosome_age,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            select: self.select,
            extension,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn with_reporter<SR2: StrategyReporter<Genotype = G>>(
        self,
        reporter: SR2,
    ) -> Builder<G, M, F, S, C, E, SR2> {
        Builder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_chromosome_age: self.max_chromosome_age,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            select: self.select,
            extension: self.extension,
            reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn with_rng_seed_from_u64(mut self, rng_seed: u64) -> Self {
        self.rng_seed = Some(rng_seed);
        self
    }
    pub fn with_rng_seed_from_u64_option(mut self, rng_seed_option: Option<u64>) -> Self {
        self.rng_seed = rng_seed_option;
        self
    }
}

#[allow(clippy::type_complexity)]
impl<
        'a,
        G: EvolveGenotype + IncrementalGenotype + PermutableGenotype + 'a,
        M: Mutate + 'a,
        F: Fitness<Genotype = G> + 'a,
        S: Crossover + 'a,
        C: Select + 'a,
        E: Extension + 'a,
        SR: StrategyReporter<Genotype = G> + 'a,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn build(
        self,
        variant: StrategyVariant,
    ) -> Result<Box<dyn Strategy<G> + 'a>, TryFromBuilderError> {
        match variant {
            StrategyVariant::Permutate(_) => match self.to_permutate_builder().build() {
                Ok(permutate) => Ok(Box::new(permutate)),
                Err(error) => Err(TryFromBuilderError(error.0)),
            },
            StrategyVariant::Evolve(_) => match self.to_evolve_builder().build() {
                Ok(evolve) => Ok(Box::new(evolve)),
                Err(error) => Err(TryFromBuilderError(error.0)),
            },
            StrategyVariant::HillClimb(hill_climb_variant) => {
                match self
                    .to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .build()
                {
                    Ok(hill_climb) => Ok(Box::new(hill_climb)),
                    Err(error) => Err(TryFromBuilderError(error.0)),
                }
            }
        }
    }
    pub fn to_permutate_builder(self) -> PermutateBuilder<G, F, SR> {
        PermutateBuilder {
            genotype: self.genotype,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness,
            reporter: self.reporter,
        }
    }
    pub fn to_evolve_builder(self) -> EvolveBuilder<G, M, F, S, C, E, SR> {
        EvolveBuilder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_chromosome_age: self.max_chromosome_age,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            select: self.select,
            extension: self.extension,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn to_hill_climb_builder(self) -> HillClimbBuilder<G, F, SR> {
        HillClimbBuilder {
            genotype: self.genotype,
            variant: None,
            max_stale_generations: self.max_stale_generations,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
}

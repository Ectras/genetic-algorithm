use genetic_algorithm::compete::{CompeteDispatch, Competes};
use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
use genetic_algorithm::fitness::FitnessSimpleCount;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::meta::{MetaConfig, MetaPermutate};
use genetic_algorithm::mutate::{MutateDispatch, Mutates};

fn main() {
    let rounds = 10;
    let population_sizes = vec![10, 20, 50, 100];
    let max_stale_generations_options = vec![Some(1000)];
    let target_fitness_score_options = vec![Some(100)];
    let degeneration_range_options = vec![None, Some(0.001..0.995)];
    let mutates = vec![
        MutateDispatch(Mutates::Once, 0.05),
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
        MutateDispatch(Mutates::Once, 0.4),
        MutateDispatch(Mutates::Once, 0.5),
    ];
    let crossovers = vec![
        CrossoverDispatch(Crossovers::Single, true),
        CrossoverDispatch(Crossovers::Single, false),
        CrossoverDispatch(Crossovers::All, true),
        CrossoverDispatch(Crossovers::All, false),
        CrossoverDispatch(Crossovers::Range, true),
        CrossoverDispatch(Crossovers::Range, false),
    ];
    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        CompeteDispatch(Competes::Tournament, 8),
    ];
    let evolve_genotype = BinaryGenotype::new().with_gene_size(100).build();
    let evolve_fitness = FitnessSimpleCount;

    let config = MetaConfig::new(
        rounds,
        evolve_genotype,
        evolve_fitness,
        population_sizes,
        max_stale_generations_options,
        target_fitness_score_options,
        degeneration_range_options,
        mutates,
        crossovers,
        competes,
    );

    println!("{:?}", config);

    let permutate = MetaPermutate::new(config).call();

    println!("{}", permutate);
}

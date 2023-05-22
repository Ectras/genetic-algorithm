use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::meta::prelude::*;

fn main() {
    env_logger::init();

    let rounds = 10;
    let target_population_sizes = vec![1, 2, 3, 4, 5, 10];
    let max_stale_generations_options = vec![Some(100)];
    let target_fitness_score_options = vec![Some(0)];
    let mutates = vec![
        MutateOnce::new_dispatch(0.05),
        MutateOnce::new_dispatch(0.1),
        MutateOnce::new_dispatch(0.2),
        MutateOnce::new_dispatch(0.3),
        MutateOnce::new_dispatch(0.4),
        MutateOnce::new_dispatch(0.5),
    ];
    let crossovers = vec![
        CrossoverDispatch(Crossovers::Clone, false),
        CrossoverDispatch(Crossovers::Clone, true),
        CrossoverDispatch(Crossovers::SingleGene, false),
        CrossoverDispatch(Crossovers::SingleGene, true),
        CrossoverDispatch(Crossovers::SinglePoint, false),
        CrossoverDispatch(Crossovers::SinglePoint, true),
        CrossoverDispatch(Crossovers::Uniform, false),
        CrossoverDispatch(Crossovers::Uniform, true),
    ];
    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        CompeteDispatch(Competes::Tournament, 8),
    ];
    let extensions = vec![
        ExtensionNoop::new_dispatch(),
        ExtensionMassDegeneration::new_dispatch(0.9, 10),
        ExtensionMassExtinction::new_dispatch(0.9, 0.1),
        ExtensionMassGenesis::new_dispatch(0.9),
        ExtensionMassInvasion::new_dispatch(0.9, 0.1),
    ];
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let fitness = CountTrue;
    let evolve_builder = EvolveBuilder::new()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize);
    let evolve_fitness_to_micro_second_factor = 1_000_000;

    let config = MetaConfig::builder()
        .with_evolve_builder(evolve_builder)
        .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
        .with_rounds(rounds)
        .with_target_population_sizes(target_population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .with_extensions(extensions)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();
    println!();
    println!("{}", permutate);
}

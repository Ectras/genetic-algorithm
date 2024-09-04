use genetic_algorithm::fitness::placeholders::SumGenes;
use genetic_algorithm::strategy::evolve::prelude::*;

fn main() {
    env_logger::init();

    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0..=10)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_target_fitness_score(99 * 100_000)
        // .with_par_fitness(true) // 2x slower in this case
        .with_mutate(MutateSingleGene::new(0.2))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectTournament::new(4, 0.9))
        .call()
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
}

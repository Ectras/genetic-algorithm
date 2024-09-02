//! A genetic algorithm implementation for Rust.
//! Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)
//!
//! There are three main elements to this approach:
//! * The [Genotype](crate::genotype) (the search space)
//! * The [Fitness](crate::fitness) function (the search goal)
//! * The [Strategy](crate::strategy::Strategy) (the search strategy)
//!     * [Evolve](crate::strategy::evolve::Evolve) (evolution strategy)
//!     * [Permutate](crate::strategy::permutate::Permutate) (for small search spaces, with a 100% guarantee)
//!     * [HillClimb](crate::strategy::hill_climb::HillClimb) (when search space is convex with little local optima or when crossover is impossible/inefficient)
//!
//! Terminology:
//! * [Population](crate::population): a population has `population_size` number of individuals (called chromosomes).
//! * [Chromosome](crate::chromosome): a chromosome has `genes_size` number of genes
//! * Gene: a gene is a combination of position in the chromosome and value of the gene (allele)
//! * [Allele](crate::genotype::Allele): alleles are the possible values of the genes
//! * [Genotype](crate::genotype): holds the `genes_size` and alleles and knows how to generate and mutate chromosomes efficiently
//! * [Fitness](crate::fitness): knows how to determine the fitness of a chromosome
//!
//! All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
//!
//! ## Quick Usage
//!
//! ```rust
//! use genetic_algorithm::strategy::evolve::prelude::*;
//!
//! // the search space
//! let genotype = BinaryGenotype::builder() // boolean alleles
//!     .with_genes_size(100)                // 100 genes per chromosome
//!     .build()
//!     .unwrap();
//!
//! println!("{}", genotype);
//!
//! // the search goal to optimize towards (maximize or minimize)
//! #[derive(Clone, Debug)]
//! pub struct CountTrue;
//! impl Fitness for CountTrue {
//!     type Allele = BinaryAllele; // bool
//!     fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Allele>) -> Option<FitnessValue> {
//!         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
//!     }
//! }
//!
//! // the search strategy
//! let evolve = Evolve::builder()
//!     .with_genotype(genotype)
//!     .with_compete(CompeteElite::new())             // sort the chromosomes by fitness to determine crossover order
//!     .with_crossover(CrossoverUniform::new(0.5))    // crossover all individual genes between 2 chromosomes for offspring, keep 50% parents around for next generation
//!     .with_mutate(MutateSingleGene::new(0.2))       // mutate a single gene with a 20% probability per chromosome
//!     .with_fitness(CountTrue)                       // count the number of true values in the chromosomes
//!     .with_target_population_size(100)              // evolve with 100 chromosomes
//!     .with_target_fitness_score(100)                // goal is 100 times true in the best chromosome
//!     .with_reporter(EvolveReporterSimple::new(100)) // optional builder step, report every 100 generations
//!     .call()
//!     .unwrap();
//!
//! println!("{}", evolve);
//! ```
//!
//! ## Tests
//!
//! Use `.with_rng_seed_from_u64(0)` builder step to create deterministic tests results.
//!
//! ## Examples
//!
//! * N-Queens puzzle <https://en.wikipedia.org/wiki/Eight_queens_puzzle>
//!     * See [examples/evolve_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_nqueens.rs)
//!     * See [examples/hill_climb_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_nqueens.rs)
//!     * `UniqueGenotype<u8>` with a 64x64 chess board setup
//!     * custom `NQueensFitness` fitness
//! * Knapsack problem: <https://en.wikipedia.org/wiki/Knapsack_problem>
//!     * See [examples/evolve_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_knapsack.rs)
//!     * See [examples/permutate_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_knapsack.rs)
//!     * `BinaryGenotype<Item(weight, value)>` each gene encodes presence in the knapsack
//!     * custom `KnapsackFitness(&items, weight_limit)` fitness
//! * Infinite Monkey theorem: <https://en.wikipedia.org/wiki/Infinite_monkey_theorem>
//!     * See [examples/evolve_monkeys](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_monkeys.rs)
//!     * `ListGenotype<char>` 100 monkeys randomly typing characters in a loop
//!     * custom fitness using hamming distance
//! * Permutation strategy instead of Evolve strategy for small search spaces, with a 100% guarantee
//!     * See [examples/permutate_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_knapsack.rs)
//!     * See [examples/permutate_scrabble](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_scrabble.rs)
//! * HillClimb strategy instead of Evolve strategy, when crossover is impossible or inefficient
//!     * See [examples/hill_climb_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_nqueens.rs)
//!     * See [examples/hill_climb_table_seating](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_table_seating.rs)
//! * Explore internal and external multithreading options
//!     * See [examples/explore_multithreading](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/explore_multithreading.rs)
//! * Custom Fitness function with LRU cache
//!     * See [examples/evolve_binary_lru_cache_fitness](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_binary_lru_cache_fitness.rs)
//!     * _Note: doesn't help performance much in this case..._
//! * Custom Reporting implementation
//!     * See [examples/permutate_scrabble](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_scrabble.rs)
//!
//! ## Performance considerations
//!
//! For the [Evolve](strategy::evolve::Evolve) strategy:
//!
//! * [Compete](compete): no considerations. All competes are basically some form of in-place
//!   sorting of some kind. This is relatively fast compared to the rest of the
//!   operations.
//! * [Crossover](crossover): the workhorse of internal parts. Crossover touches most genes each
//!   generation and clones the whole population if you keep the parents around. See
//!   performance tips below.
//! * [Mutate](mutate): no considerations. It touches genes like crossover does, but should
//!   be used sparingly anyway; with low gene counts (<10%) and low probability (5-20%)
//! * [Fitness](fitness): can be anything. This fully depends on the user domain. Parallelize
//!   it using `with_par_fitness()` in the Builder. But beware that parallelization
//!   has it's own overhead and is not always faster.
//!
//! **Performance Tips**
//! * Small genes sizes
//!   * It seems that [CrossoverMultiGene](crossover::CrossoverMultiGene) with
//!     `number_of_crossovers = genes_size / 2` and `allow_duplicates = true`
//!     is the best tradeoff between performance and effect.
//!     [CrossoverUniform](crossover::CrossoverUniform) is an alias for the same approach,
//!     taking the genes_size from the genotype at runtime.
//!   * Keeping the parents around doesn't matter that much as the cloning is relatively less
//!     pronounced (but becomes more prominent for larger population sizes)
//! * Large genes sizes
//!   * It seems that [CrossoverMultiPoint](crossover::CrossoverMultiPoint) with
//!     `number_of_crossovers = genes_size / 9` and `allow_duplicates = false` is
//!     the best tradeoff between performance and effect.
//!   * Keeping the parents around has major performance effects and should be avoided

pub mod chromosome;
pub mod compete;
pub mod crossover;
pub mod extension;
pub mod fitness;
pub mod genotype;
pub mod mutate;
pub mod population;
pub mod strategy;

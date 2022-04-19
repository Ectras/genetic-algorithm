mod support;

#[cfg(test)]
mod compete_tests {
    use crate::support::*;
    use genetic_algorithm::compete;
    use genetic_algorithm::compete::Compete;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::fitness;
    use genetic_algorithm::fitness::Fitness;

    #[test]
    fn test_elite() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]);

        let population = fitness::SimpleSum.call_for_population(population);
        let population = compete::Elite.call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
                vec![true, true, true],
            ]
        );
    }

    #[test]
    fn test_tournament() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population = build::population(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]);

        let population = fitness::SimpleSum.call_for_population(population);
        let population = compete::Tournament(4).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![false, true, true],
                vec![true, false, false],
            ]
        );
    }

    #[test]
    fn test_tournament_shortage() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_rng(rng);

        let population =
            build::population(vec![vec![false, false, false], vec![false, false, true]]);

        let population = fitness::SimpleSum.call_for_population(population);
        let population = compete::Tournament(4).call(&mut context, population);

        assert_eq!(
            inspect::population(&population),
            vec![vec![false, false, true], vec![false, false, false],]
        );
    }
}

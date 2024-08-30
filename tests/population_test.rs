mod support;

#[cfg(test)]
mod population_tests {
    use crate::support::*;
    use approx::assert_relative_eq;
    use genetic_algorithm::fitness::placeholders::CountTrue;
    use genetic_algorithm::fitness::{Fitness, FitnessOrdering};

    #[test]
    fn fitness_score_stddev() {
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountTrue.call_for_population(population, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.866, epsilon = 0.001);

        let population = &mut build::population(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
        ]);

        assert_eq!(population.fitness_score_stddev(), 0.0);
        CountTrue.call_for_population(population, None);
        assert_relative_eq!(population.fitness_score_stddev(), 0.331, epsilon = 0.001);
    }

    #[test]
    fn best_chromosome() {
        let population = &mut build::population_with_fitness_scores(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        let best_chromosome = population.best_chromosome(FitnessOrdering::Maximize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score),
            Some(3)
        );
        let best_chromosome = population.best_chromosome(FitnessOrdering::Minimize);
        assert_eq!(
            best_chromosome.map_or(Some(99), |c| c.fitness_score),
            Some(0)
        );
    }

    #[test]
    fn fitness_score_cardinality() {
        let population = &mut build::population_with_fitness_scores(vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]);

        assert_eq!(population.fitness_score_cardinality(), 3 + 1);
    }

    #[test]
    fn trim_from_start_to() {
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);
        assert_eq!(population.chromosomes.capacity(), 8);

        population.drain_from_start_to(8);
        assert_eq!(population.chromosomes.len(), 8);
        population.drain_from_start_to(10);
        assert_eq!(population.chromosomes.len(), 8);

        population.drain_from_start_to(6);
        assert_eq!(
            inspect::population(population),
            vec![
                vec![false, false, true],
                vec![false, false, false],
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
                vec![true, false, false],
            ]
        );
        assert_eq!(population.chromosomes.capacity(), 8);
    }
    #[test]
    fn trim_from_end_to() {
        let population = &mut build::population(vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
        ]);
        assert_eq!(population.chromosomes.capacity(), 8);

        population.truncate(8);
        assert_eq!(population.chromosomes.len(), 8);
        population.truncate(10);
        assert_eq!(population.chromosomes.len(), 8);

        population.truncate(6);
        assert_eq!(
            inspect::population(population),
            vec![
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false, true],
                vec![false, false, false],
                vec![true, true, true],
                vec![true, true, false],
            ]
        );
        assert_eq!(population.chromosomes.capacity(), 8);
    }
}

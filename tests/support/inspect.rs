use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genotype::Genotype;
//use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Genotype>(chromosome: &Chromosome<T>) -> Vec<<T as Genotype>::Gene> {
    chromosome.genes.clone()
}

//#[allow(dead_code)]
//pub fn population<T: Gene>(population: &Population<T>) -> Vec<Vec<T>> {
//population
//.chromosomes
//.iter()
//.map(|c| chromosome(&c))
//.collect()
//}

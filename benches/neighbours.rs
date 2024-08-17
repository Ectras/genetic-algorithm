use criterion::*;
use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("neighbours");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let mut rng1 = SmallRng::from_entropy();
    let mut rng2 = SmallRng::from_entropy();

    group.bench_function("continues-neighbouring_population-scaled", |b| {
        let genotype = ContinuousGenotype::builder()
            .with_genes_size(10)
            .with_allele_range(-1.0..=1.0)
            .with_allele_neighbour_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
            .build()
            .unwrap();

        b.iter_batched(
            || genotype.chromosome_factory(&mut rng1),
            |c| genotype.neighbouring_population(&c, Some(1), &mut rng2),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("continues-neighbouring_population-unscaled", |b| {
        let genotype = ContinuousGenotype::builder()
            .with_genes_size(10)
            .with_allele_range(-1.0..=1.0)
            .with_allele_neighbour_range(-0.1..=0.1)
            .build()
            .unwrap();

        b.iter_batched(
            || genotype.chromosome_factory(&mut rng1),
            |c| genotype.neighbouring_population(&c, None, &mut rng2),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

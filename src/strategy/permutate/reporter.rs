//! Reporters directed at Permutate process specific data
use crate::genotype::PermutableGenotype;
use crate::strategy::{StrategyConfig, StrategyReporter, StrategyState, STRATEGY_ACTIONS};
use num::{BigUint, ToPrimitive};
use std::marker::PhantomData;

/// A Simple Permutate reporter generic over Genotype.
/// A report is triggered every period generations
#[derive(Clone)]
pub struct Simple<G: PermutableGenotype> {
    pub period: usize,
    pub show_genes: bool,
    _phantom: PhantomData<G>,
}
impl<G: PermutableGenotype> Default for Simple<G> {
    fn default() -> Self {
        Self {
            period: 1,
            show_genes: false,
            _phantom: PhantomData,
        }
    }
}
impl<G: PermutableGenotype> Simple<G> {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    }
    pub fn new_with_flags(period: usize, show_genes: bool) -> Self {
        Self {
            period,
            show_genes,
            ..Default::default()
        }
    }
}
impl<G: PermutableGenotype> StrategyReporter for Simple<G> {
    type Genotype = G;

    fn on_new_generation<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        if state.current_generation() % self.period == 0 {
            let progress = (BigUint::from(state.current_generation() * 100)
                / &genotype.chromosome_permutations_size())
                .to_u8();
            println!(
                "progress: {}, current_generation: {}, best_generation: {}",
                progress.map_or("-".to_string(), |v| format!("{:3.3}%", v)),
                state.current_generation(),
                state.best_generation(),
            );
        }
    }

    fn on_new_best_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!(
            "new best - generation: {}, fitness_score: {:?}, genes: {:?}",
            state.current_generation(),
            state.best_fitness_score(),
            if self.show_genes {
                Some(genotype.best_genes())
            } else {
                None
            },
        );
    }

    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("finish - generation: {}", state.current_generation());
        STRATEGY_ACTIONS.iter().for_each(|action| {
            if let Some(duration) = state.durations().get(action) {
                println!("  {:?}: {:?}", action, duration,);
            }
        });
        println!("  Total: {:?}", &state.total_duration());
    }
}

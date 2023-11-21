use prisoners_dilemna::{
    strategies::{ALWAYS_COOPERATE, ALWAYS_DEFECT},
    Duel,
};

fn main() {
    let mut duel = Duel::new(ALWAYS_COOPERATE, ALWAYS_DEFECT, 10);
    let outcome = duel.run();

    dbg!(outcome);
}

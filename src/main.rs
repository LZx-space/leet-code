use crate::solver::Solver;

mod m_add_two_num;
mod m_remove_duplicates_from_sorted_array_ii;
mod s_two_sum;
mod solver;

fn main() {
    let solutions: Vec<Box<dyn Solver>> = vec![
        Box::new(m_add_two_num::Solution {}),
        Box::new(m_remove_duplicates_from_sorted_array_ii::Solution {}),
        Box::new(s_two_sum::Solution {}),
    ];
    solutions.iter().for_each(|solver| {
        println!("***********************************");
        println!(">>> solve -------------------------");
        solver.solve();
        println!(">>> related lang concepts ---------");
        solver.related_lang_concepts();
    });
}

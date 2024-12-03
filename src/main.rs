use crate::solver::Solver;

mod m_add_two_numbers;
mod m_longest_substring_without_repeating_characters;
mod m_remove_duplicates_from_sorted_array_ii;
mod s_two_sum;
mod solver;

fn main() {
    let solutions: Vec<Box<dyn Solver>> = vec![
        Box::new(m_add_two_numbers::Solution {}),
        Box::new(m_longest_substring_without_repeating_characters::Solution {}),
        Box::new(m_remove_duplicates_from_sorted_array_ii::Solution {}),
        Box::new(s_two_sum::Solution {}),
    ];
    solutions.iter().enumerate().for_each(|(i, solver)| {
        println!("{:0>3} *******************************", i + 1);
        println!(">>> solve -------------------------");
        solver.solve();
        println!(">>> related lang concepts ---------");
        solver.related_lang_concepts();
    });
}

use crate::solver::Solver;

mod m_80;
mod m_add_two_num;
mod s_1436;
mod solver;

fn main() {
    let solutions: Vec<Box<dyn Solver>> = vec![
        Box::new(m_80::Solution{}),
        Box::new(s_1436::Solution {})
    ];
    solutions.iter().for_each(|solver| {
        println!("***********************************");
        println!(">>> solve -------------------------");
        solver.solve();
        println!(">>> related lang concepts ---------");
        solver.related_lang_concepts();
    });
}

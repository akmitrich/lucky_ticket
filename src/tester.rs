use std::{
    time::Instant,
};

pub fn run_test<Solver>(path: &str, solver: Solver)
where Solver: Fn(Vec<&str>) -> String {
    let mut i = 0;
    while let Ok(str_n) = std::fs::read_to_string(dbg!(format!("{path}/test.{i}.in"))) {
        let start = Instant::now();
        let input_data: Vec<&str> = str_n.lines().collect();
        let solved = solver(input_data);
        let expected_result = std::fs::read_to_string(dbg!(format!("{path}/test.{i}.out"))).unwrap();
        let _ = dbg!(Instant::now().duration_since(start));
        assert_eq!(expected_result.trim(), solved);
        i += 1;
    }
}
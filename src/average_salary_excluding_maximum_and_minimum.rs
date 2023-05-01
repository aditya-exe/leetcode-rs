pub fn average(mut salary: Vec<i32>) -> f64 {
    salary.sort_unstable();

    let newSalary = salary
        .iter()
        .enumerate()
        .filter(|(i, x)| *i != 0 as usize && *i != salary.len() - 1)
        .map(|(i, x)| *x)
        .collect::<Vec<_>>();

    let ans: f64 = newSalary.iter().sum::<i32>() as f64 / newSalary.len() as f64;

    return ans;
}
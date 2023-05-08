pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    obstacles.into_iter().scan(Vec::new(), |monotonic_stack, height| {
        let point = monotonic_stack.partition_point(|&prev| prev <= height);
        if point == monotonic_stack.len() {
            monotonic_stack.push(height);
        } else {
            monotonic_stack[point] = height;
        }
        Some(point as i32 + 1)
    }).collect()
}
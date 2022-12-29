pub fn part1(inst: &Vec<(i32, i32, i32)>, stacks: &mut Vec<Vec<char>>) {
    // For each tup (representing an instruction) iterate over the first element
    // which represent the "move n" part of the instruction
    for tup in inst.iter() {
        for _ in 0..tup.0 {
            // tup.1 represents the "from n" part of the instruction, so by
            // using pop where remove the element from n, and
            match stacks[tup.1 as usize - 1].pop() {
                Some(c) => {
                    // move it to tup.2. tup.2 represents "to n" part of the instruction
                    stacks[tup.2 as usize - 1].push(c);
                }
                None => {}
            };
        }
    }
    print!("Part 1: ");
    for v in stacks {
        let c = v.last().unwrap();
        print!("{}", c);
    }
    println!("");
}

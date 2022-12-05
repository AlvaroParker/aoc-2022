pub fn part2(inst: &Vec<(i32, i32, i32)>, stacks: &mut [Vec<char>; 9]) {
    // Temp vector to move the elements in groups

    // For each instruction
    for tup in inst.iter() {
        // select n elements starting from "from"
        let from = stacks[tup.1 as usize - 1].len() - tup.0 as usize;
        let mut u = stacks[tup.1 as usize - 1]
            .drain(from..)
            .collect::<Vec<char>>();
        // Append those elements to the column tup.2
        stacks[tup.2 as usize - 1].append(&mut u);
    }

    print!("Part 2: ");
    for v in stacks {
        let c = v.last().unwrap();
        print!("{}", c);
    }
    println!("");
}

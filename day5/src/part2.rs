pub fn part2(inst: &Vec<(i32, i32, i32)>, stacks: &mut Vec<Vec<char>>) {
    // Temp vector to move the elements in groups

    // For each instruction
    for tup in inst.iter() {
        // select n elements starting from "from"
        let from = stacks.get(tup.1 as usize - 1).unwrap().len() - tup.0 as usize;
        let mut u = stacks
            .get_mut(tup.1 as usize - 1)
            .unwrap()
            .drain(from..)
            .collect::<Vec<char>>();
        // Append those elements to the column tup.2
        stacks.get_mut(tup.2 as usize - 1).unwrap().append(&mut u);
    }

    print!("Part 2: ");
    for v in stacks {
        let c = v.last().unwrap();
        print!("{}", c);
    }
    println!("");
}

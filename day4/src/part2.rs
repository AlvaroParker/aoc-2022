pub fn part2(v: &Vec<Vec<(i32, i32)>>) {
    let mut counter = 0;
    for element in v {
        assert!(element.len() == 2);
        let tup1 = element.get(0).unwrap();
        let tup2 = element.get(1).unwrap();

        // (10-200) (30-40)
        if (tup1.0 >= tup2.0 && (tup1.1 <= tup2.1 || tup1.0 <= tup2.1))
            || (tup2.0 >= tup1.0 && (tup2.0 <= tup1.1 || tup2.1 <= tup1.1))
        {
            counter += 1;
        }
    }
    println!("P2 Final count: {}", counter);
}

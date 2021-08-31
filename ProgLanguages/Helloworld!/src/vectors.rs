// Vectors -- resizeable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // add to vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    //pop off last value
    numbers.pop();
    numbers.pop();
    numbers.pop();
    println!("After pop: {:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vec: {:?}", numbers);

}
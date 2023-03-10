fn main() {
    // ================================================================
    // TUPLES
    // ================================================================
    // Tuples are immutable collections of data.
    // Tuples can be heterogenous.
    let shape_data = ("Square", 4, true);
    // Data is accessed from a tuple using <tuple>.<index> notation
    println!(
        "{} has {} sides. Is it a quadrilateral? : {}",
        shape_data.0, shape_data.1, shape_data.2
    );
    // Tuples can be destructured
    let (shape, sides_count, is_quadrilateral) = shape_data;
    println!(
        "{} has {} sides. Is it a quadrilateral? : {}",
        shape, sides_count, is_quadrilateral
    );
    // Tuples structure itself is immutable.
    // But the inner values can be made mutable
    let mut shape_data = ("Square", 4, true);
    println!("Old value : {}", shape_data.0);
    // Here element at index 0 will remain a string. Datatype can't be changed.
    shape_data.0 = "Rectangle";
    println!("New value : {}", shape_data.0);

    // ================================================================
    // ARRAYS
    // ================================================================
    // Arrays are collection of homogenous data elements
    // All elements are supposed to be of same datatype
    let data_arr = [1, 2, 3, 4];
    println!(
        "Array contains : {} {} {} {}",
        data_arr[0], data_arr[1], data_arr[2], data_arr[3],
    );

    // ================================================================
    // VECTORS
    // ================================================================
    // Vectors are extendible lists that behave like stacks
    let mut stack = vec![1, 2, 3];

    println!("Stack value {:?}", stack);

    println!("Operation - Push: 4");
    stack.push(4);
    println!("Stack value {:?}", stack);

    println!("Operation - Pop");
    stack.pop();
    println!("Stack value {:?}", stack);

    // Stacks also allow inserting and removing values at certain indices
    println!("Operation - Insert: -1 at 0");
    stack.insert(0, -1);
    println!("Stack value {:?}", stack);

    println!("Operation - Remove value at 0");
    stack.remove(0);
    println!("Stack value {:?}", stack);

    // To generate a series of numbers
    let numbers_till_5: Vec<i32> = (0..5).collect();
    println!("Numbers till 5 {:?}", numbers_till_5);

    // ================================================================
    // SLICES
    // ================================================================
    // Slices are portions of a bigger collection that can be used
    // instead of refrencing the complete collection

    let numbers_till_5: Vec<i32> = (0..5).collect();
    println!("Numbers till 5 {:?}", numbers_till_5);

    let slice_from_1_to_3 = &numbers_till_5[1..4];
    println!("Numbers from 1 to 3 {:?}", slice_from_1_to_3);

    // ================================================================
    // STRINGS
    // ================================================================
    // String allocates memory on a heap and store valid utf-8 sequences.
    let name = String::from("Luffy");
    println!("{}", name);

    let name = "Luffy".to_string();
    println!("{}", name);

    // String slice does not allocate memory on heap.
    // It references a portion of slice via address.
    // String slices can be used for arguments for a function.
    let shortname = &name[0..2];
    println!("{}", shortname);

    // String literals are special strings
    // which can contain characters besides valid utf-8 set
    let rust_name = "\x52\x75\x73\x74";
    println!("{}", rust_name);
}

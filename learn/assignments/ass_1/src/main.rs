// Task 1
// Create three variables with the names: val1, val2, and ans.
// We want to perform a simple operation of generating the modulo of val1 and val2.
// Set val1 to 5 and val2 to 2. Assign the answer to the ans variable.
// Before executing your code, what do you think the answer will be?

// Task 2
// Create a vector and put in the values "2, 4, 6, 8, 10".
// Once you have created the vector perform the following:
// print out the current values,
// remove the value 10,
// add the value 12, and then
// print the vector back out to confirm your results.

// Task 3
// Create a function called "concat_string".
// Create a string variable and assign the value "Hello" to it.
// The function is going to take one argument that is of type string and
// is going to return a String.
// Inside this function, concatenate the string " World".
// Print out the results in main() to confirm your results.

// Task 4
// Create a function called control_flow.
// This is going to take one argument that is an integer.
// Based on this integer, print out the following:
// "The value is one",
// "The value is greater than 50",
// "The value is less than 25",
// or "The value is greater than 25 but less than 50".

fn main() {
    // Task 1
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;

    println!("Modulo of {} and {} is {}", val1, val2, ans);

    // Task 2
    let mut vector = vec![2, 4, 6, 8, 10];
    println!("Current values: {:?}", vector);
    vector.pop();
    vector.push(12);
    println!("Final values: {:?}", vector);

    // Task 3
    let str_var = String::from("Hello");
    println!("{:?}", concat_string_slice(&str_var));
    println!("{:?}", concat_string(str_var));

    // Task 4
    let task_4_val_1: i32 = 16;
    let task_4_val_2: i32 = 26;
    let task_4_val_3: i32 = 56;
    let task_4_val_4: i32 = 1;
    let task_4_val_5: i32 = -1;

    println!("{:?}", control_flow(task_4_val_1));
    println!("{:?}", control_flow(task_4_val_2));
    println!("{:?}", control_flow(task_4_val_3));
    println!("{:?}", control_flow(task_4_val_4));
    println!("{:?}", control_flow(task_4_val_5));
}

fn concat_string_slice(str_var: &str) -> String {
    str_var.to_owned() + " World"
}

fn concat_string(str_var: String) -> String {
    str_var + " World"
}

fn control_flow(val: i32) -> String {
    if val == 1 {
        format!("The value {} is one", val)
    } else if val > 50 {
        format!("The value {} is greater than 50", val)
    } else if val < 25 {
        format!("The value {} is less than 25", val)
    } else {
        format!("The value {} is greater than 25 but less than 50", val)
    }
}

fn control_flow(val: i32) -> String {
    if val == 1 {
        String::from("The value is one")
    } else if val > 50 {
        String::from("The value is greater than 50")
    } else if val < 25 {
        String::from("The value is less than 25")
    } else {
        String::from("The value is greater than 25 but less than 50")
    }
}

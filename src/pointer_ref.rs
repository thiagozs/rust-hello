//

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Arr Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assing another variable to a pice of data, 
    //the first variable will no longer hold that value. You have to do more work to copy the data, which incurs a performance cost.

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vec Values: {:?}", (&vec1, vec2));
}
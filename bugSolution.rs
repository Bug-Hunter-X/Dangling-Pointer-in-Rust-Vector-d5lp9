fn main() {
    let mut v = vec![1, 2, 3];
    // Correct approach: Use indexing instead of raw pointers.
    v[0] = 10;
    println!("v: {:?}", v);
    //Alternative Approach: Avoid reallocation if possible
    let mut v2 = vec![1,2,3];
    v2.push(4); //This will likely not cause reallocation
    let ptr2 = v2.as_mut_ptr();
    unsafe{
        *ptr2 = 10;
    }
    println!("v2: {:?}", v2); 
} 
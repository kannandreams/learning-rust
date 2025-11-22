fn main() {
    /*  Vectors -  The data lives on the heap.
        generic, growable collection type defined in the Rust standard library.
        std::vec::Vec<T>
        how a vector is stored:
        Vec struct (on stack)
         ├── ptr → heap memory for elements
         ├── len
         └── cap

        eg:
            Stack                                 Heap
            -----------------------------------   -----------------
            vec3: Vec {                                 1
                ptr → -------------------------->       2
                len: 3                                  3
                cap: 3
            }
    */

    /* Create Vector
    Syntax : let vec_1: Vec<T> = Vec::new();
    */
    let vec1: Vec<String> = Vec::new();

    let vec2 = Vec::from(["a", "b", "c"]);
    // vec2 is Vec<&str>
    // element type  T = &str
    /* What happens ?
       The array literal ["a", "b", "c"] is a temporary fixed-size array created on the stack.
       Vec::from(...) copies those values into a new Vec.
       A Vec always stores its elements on the heap.
    */

    // or we could create using vec macro
    let mut vec3 = vec!["a", "b", "c"];
    /* What is the vec![] macro?
        - It is a macro built into Rust’s standard library.
        - Rust expands it roughly into:
            {
                let tmp = [1, 2, 3];    // stack array
                Vec::from(tmp)          // heap allocation + copy
            }
        - It’s equivalent to: Vec::from([1, 2, 3]);
    */

    println!("{:?}", vec1);
    println!("{:?}", vec2);
    println!("{:?}", vec3);

    // iterate
    for item in vec3.iter() {
        println!("{:?}", item);
    }

    // check if element exists or not
    // Since a has type &'static str, it has to converted to &String using .to_string()
    println!("{}", vec2.contains(&"a"));
    println!("{}", vec3.contains(&"a"));
    /* contains() expects a reference
        fn contains(&self, x: &T) -> bool
        so T = String (i.e) &String
    */

    /* NOTE: Why does it expect &T?
    The vector already owns its elements.
    contains() only needs to look at the element, not take ownership.
    Taking ownership would require cloning or moving elements — inefficient or impossible.
    */

    // push elements
    //you must use mut to push elements into a Vec.
    vec3.push("d");
    println!("{:?}", vec3);
}

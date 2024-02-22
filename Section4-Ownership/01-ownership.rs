// ------------------------------------
//          Ownershup basics
// ------------------------------------

/*
1. Each value has a variable that's its owner
2. A value can only have one owner at at time
3. If the value goes out of scope the value is cleaned up.
 */

 fn main() {
    let s1: String = String::from("World");
    // println!("s1 is: {s1}");    // Get the error of borrow of moved value: s1

    {
        let s2: String = s1;

    }
    // println!("s2 is {s2}");   // s2 is inner scope

    let x: i32 = 15;
    let y: i32 = x;

    println!("x is {x}");
 }

/*
Two Types of Memory
1. Non-Volatile
    - Hard Drives / SSD
    - Slow but abundant
    - Persist Data
2. Volatile memory
    - RAM/Cache
    - Fast and scarce
    - During Program Execution

Program Memory Layout
Stack -- Grows Downward   -- Stack deal with data which has fixed known size data at compile time, size is known so values are stored in lIFO way, management
                            is easy as everything is predictable and donot required any computation
Heat  -- Grows Upward     -- Heap deals with data that is size is unknown at compile time, Heap is slow Why ? require alot of memory management
Static --- Used to store the static variables, binary instructions, clean up is automatic and fill is also when program starts and ends


// What these parts do with the ownership?

Stack                                                                           Heap
s1 ptr                                                                              --> World           
   len       5    // occupying the length in heap
   capacity  5    // assigned by the allocator or manager of heap 
pointer, length and capacity has fixed sizes so they recide on the stack
value is moved in to s2 allso s1 data is copied on stack, to ensure rust the single owner to rust will immediately 
delete s1
that's why accessing s1 gives us the error
to copy we use the s1.clone method

What it does behind the scene
1. First it copy the stack data of s1
2. Second it will also copy the heap data
3. So now ownership rule remain intact and s1 is not terminated, deep copy is similar to clone
when stack data is only copied rust use the word copy 

When owner goes out of scope the value goes of

Implication of this in term of memory prespective 
it prevents memory leak how becuase when scope ends variable also goes of
all this is possible at compile time when we are writing code
zero run time cost

On stack they are copied but on heap they are copied and moved so for heap allocated data we get the error of this kind for 
not the int

 */
# Vector - Data Structures: Rust Programming Series

Rust provides several data structures in its standard library, and in this article, we will focus on working with vectors, as it is the most commonly used collection type in Rust.

**Vector (Vec<T>)**

A vector is similar to an array, except it can grow in size or shrink as needed.

**Key Features:**

- Ownership and Borrowing: **Vec<T>** strictly adheres to Rust's ownership principles, ensuring memory safety without a garbage collector. This means Rust guarantees no dangling pointers or data races at compile time.

- Type Safety: **Vec<T>** is generic (**<T>**), meaning it stores elements of a single, specified type **T**. The compiler enforces this type safety.

- Dynamic Resizing

Let's create a vector, add elements, iterate through the it using the for loop, remove the elements, and finally, print out the length and size of the vector.

``` rust 
fn main() {

    // Initialize a mutable vector using macro 
    let mut vec = vec![1, 2, 3];
    
    // Add elements
    // Using push to add elements
    // This will automatically resize the vector if needed
    vec.push(4);
    vec.push(5);
    
    // Access elements
    // Using indexing and get method
    // Indexing will panic if the index is out of bounds
    // Get method returns an Option
    // and will return None if the index is out of bounds
    println!("First element: {}", vec[0]); // 1
    println!("Second element: {}", vec.get(1).unwrap()); // 2
    
    // Iterate over elements
    // Using a for loop to iterate over a reference to vec
    for i in &vec {
        println!("{}", i);
    }
    
    // Remove element
    // Using pop to remove the last element
    // This will return an Option, and will return None if the vector is empty
    vec.pop(); 
    
    // Length and capacity
    // Length is the number of elements in the vector
    // Capacity is the number of elements the vector can hold without reallocating
    // The capacity is automatically increased when needed
    println!("Length: {}, Capacity: {}", vec.len(), vec.capacity());
}

```

**Output**:

```
First element: 1
Second element: 2
1
2
3
4
5
Length: 4, Capacity: 6

```
**Specific code explanation:**

```rust
let mut vec = vec![1, 2, 3];
```

Creates a new mutable vector with initial values using the macro vec!. Creates a new mutable vector with initial values using the macro vec!. In Rust, macros are denoted by the bang symbol **!** and are a form of metaprogramming that expands into regular code at compile time. Also, a new blank vector can be created using **= Vec::new()** as well.  

```rust 
vec.push()
```

Inserts a new element at the end of the vector object, similar to the Stack data structure, where **LIFO** (Last In First Out) is the principle that determines the data flow. Also, **.insert(<index>, <value>)** can be used to insert an element at a specific position. 

```rust 
vec.pop()
```

Removes the last element from the vector. Another way to remove an element is to use the built-in method **.remove(<index>)** to remove a specific element, and also .clear() to remove all the elements in the vector. 

```rust
vec.capacity()
```

This method returns the total number of elements the vector can hold without reallocating memory. It's the size of the overall allocated memory block. 

Now that we have a basic understanding of how the vector works, we will work on solving data structure-related questions using vectors.

## Question 1:

Given an empty vector of integers, perform the following operations:

1. Add the numbers 8, 9, and 10 to the vector.

2. Print the vector.

3. Print the last element in the vector.

4. Remove the last element.

5. Insert the number 5 at the beginning of the vector.

6. Print the final vector and its length.

**Expected output:** 

```
Vector after adding elements: [8, 9, 10]
The last element: 10
Final vector: [5, 8, 9]
Final length: 3
```

**Solution:** 

Based on the question, it looks like we need to initialize an empty mutable vector and then add elements to it. In this case, we will create a new vector using the **new()** method, then add new elements to it. 

```rust 
let mut vec = Vec::new();
vec.push(8);
vec.push(9);
vec.push(10); 

```

Part 2 says we need to print the vector that looks similar to the expected output. To print all the elements in the vector, we will use the **{:?}** formatter. 

```rust
println!("Vector after adding elements: {:?}", vec);
```

Part 3, print the last element in the vector:

```rust
println!("The last element: {}", vec.last().unwrap());
```

Part 4, remove the last element: 

```rust 
vec.pop();
```

Part 5, insert number 5 at the beginning of the vector:

```rust
vec.insert(0, 5);
```

Part 6, print final vector and its length: 

```rust
println!("The final vector {:?}", vec);
println!("The length {}", vec.len());
```

**Complete code:**

```rust 
fn main(){

    // Initialize a new empty vector
    let mut vec = Vec::new();

    // Add elements to the vector
    vec.push(8);
    vec.push(9);
    vec.push(10);

    println!("Vector after adding elements: {:?}", vec);

    // Remove the last element
    println!("The last element: {}", vec.last().unwrap());
    vec.pop();

    // insert an element 5 at the beginning
    vec.insert(0, 5);

    println!("Final vector: {:?}", vec);
    println!("Final length: {}", vec.len());
}
```

## Question 2 

Write a function sum_of_vector that takes a reference to a vector of i32 and returns the sum of all its elements.

**Function Signature**: fn sum_of_vector(numbers: &Vec<i32>) -> i32

**Example**: sum_of_vector(&vec![1, 2, 3, 4, 5]) should return 15

**Solution 1**:

```rust 
fn sum_of_vector(numbers: &Vec<i32>) -> i32 {
   let mut sum = 0;

   for &num in numbers {
    // Using a reference to avoid unnecessary copying
    // and to ensure we are working with the original data
    sum += num; //add value as we iterate
   }

   sum
}
```

**Solution 2**: Use the **.iter()** method to access the each element. 

```rust 
fn sum_of_vector(numbers: &Vec<i32>) -> i32 {
   let mut sum = 0;

    for &num in numbers.iter() {
    // Using the `&` operator to dereference the value
    // and get the actual integer value
    // This is necessary because .iter() method returns an 
    // iterator over references to the elements of the vector
    
         sum += num;
    }
   sum
}
```

**Solution 3**: Use the built-in methods **.iter()** and **.sum()** to iterate through the elements and add up the values. 

```rust 
fn sum_of_vector(numbers: &Vec<i32>) -> i32 {
    // uses the .iter() method to create an iterator 
    // and then sums the elements using .sum() method
   numbers.iter().sum()
}
```

To call our function from main(): 

```rust 
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let total = sum_of_vector(&numbers);
    println!("The sum of the vector is: {}", total);
}
```

**Output**: 

```
The sum of the vector is: 15
```

## Question 3

Write a function filter_even_numbers that takes a reference to a vector of i32 and returns a new vector containing only the even numbers from the input vector.

**Function Signature**: fn filter_even_numbers(numbers: &Vec<i32>) -> Vec<i32>

**Example**: filter_even_numbers(&vec![1, 2, 3, 4, 5, 6]) should return vec![2, 4, 6]

**Solution 1**: Using .filter() and .clone()

```rust 
fn filter_even_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    // Using an iterator to filter even numbers
    // filtering is done using a closure that checks if the number is even
    // &&x is used to dereference the reference to the value in the iterator
    // The cloned() method is used to convert the references back to values
    // The collect() method is used to gather the results into a new vector
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();

    // Returning the vector of even numbers
    even_numbers
}
```

**Solution 2**: 

```rust
fn filter_even_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut even_numbers = Vec::new();

    for &num in numbers{
        if num % 2 == 0 {
            even_numbers.push(num);
        }   
    }

    even_numbers
}
```

Calling the function: 

```rust 
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter_even_numbers(&numbers);
    println!("Even numbers: {:?}", even_numbers);
}
```

**output**: 

```
Even numbers: [2, 4, 6]
```

## Question 4

Write a function remove_duplicates that takes a mutable reference to a vector of i32 and modifies it in place to remove any duplicate elements. The order of the remaining unique elements does not need to be preserved.

**Function Signature**: fn remove_duplicates(numbers: &mut Vec<i32>)

**Example**: If numbers is vec![1, 1, 2, 3, 3, 4, 4, 5], after calling the function, it could be vec![1, 2, 3, 4, 5] (order may vary).

**Solution 1**: Using **sort_unstable()** and **dedup()**

```Rust
fn remove_duplicates(numbers: &mut Vec<i32>) {

    // Sort the vector first
    // This is necessary for deduplication to work correctly
    // The sort_unstable() method is used for better performance
    // It does not guarantee the order of equal elements
    // but is faster than sort
    // The dedup method will then remove adjacent duplicate elements
    numbers.sort_unstable();
    numbers.dedup();
    
}
```

**Solution 2**: Using a **HashSet** and another **vector** object.

```Rust 
fn remove_duplicates(numbers: &mut Vec<i32>) {

    // initializing a HashSet to keep track of unique numbers
    // This will help us efficiently check for duplicates
    // We use a HashSet because it provides O(1) average time complexity for insertions and lookups
    // The &mut Vec<i32> allows us to modify the original vector in place
    // We will iterate through the vector and only keep unique numbers
    // The unique_numbers HashSet will store numbers that have already been encountered
    // The vec will store the unique numbers
    let mut unique_numbers = std::collections::HashSet::new();
    let mut vec = Vec::new();

    for &num in numbers.iter() {
        // Insert method returns true if the value was not present in the set
        // This ensures that we only add unique numbers to the new vector
        // If the number is already in the set, it won't be added again
        // This effectively removes duplicates
        // We use &num to avoid moving the value out of the original vector
        if unique_numbers.insert(num) {
            vec.push(num);
        }
    }
    // Replace the original vector with the new one without duplicates
    *numbers = vec;
}
```

**Calling this function**: 

```Rust
fn main() {
    let mut numbers = vec![1, 1, 2, 3, 3, 4, 4, 5];
    println!("Before removing duplicates: {:?}", numbers);
    
    remove_duplicates(&mut numbers);
    
    println!("After removing duplicates: {:?}", numbers);
}
```

**Output**: 

```
Before removing duplicates: [1, 1, 2, 3, 3, 4, 4, 5]
After removing duplicates: [1, 2, 3, 4, 5]
```

Till next time!
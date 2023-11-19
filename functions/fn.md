# Functions
Functions are declared in rust using the fn keyword as shown below:
`fn test(){
println!("Hello world")
}`

# Function Parameters
The function above could have received parameters. Parameters are additional information that is passed to a function;
For example, we dont wanna say hello world, we wanna take some number and log it as well. Here's how we'd do it:
`
fn test(a:i32){
println!("Hello world: {}", a)
}
`
When the above function is called, Rust will expect that the argument
will be passed to the test function. Also note that the argument has a `i32` type annotation. This is because
Rust is a statically typed language therefore all parameters MUST be annotated.

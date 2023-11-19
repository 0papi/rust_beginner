# Arrays
In rust, unlike other languages, arrays are fixed in length. They cannot grow or shrink, therefore you would usually use arrays for something that doesn't change over the lifetime of your app. For example if you needed to store all the months of the year, an array would be a good use-case given that it wouldn't change. Note that arrays cannot have more than one data type; thus, you cannot mix integers with characters or booleans as you may be able to do in some languages(JavaScript for instance).

## Declaring Arrays
Arrays are declared with [] as is the case in many other programming languages. An example below:
`let months = ["January", "February", "March"]
`
You can declare arrays like the above without annotating the types or you may annotate the types as shown below:
`let random_numbers: [i32; 5]=[1,2,3,4,5]
`
The code above shows a simple example of how to annotate an array type in rust (this is important as Rust is statically typed like TypeScript). The first part of the type annotation states the data type that makes up the array. `i32` is an integer type that shows that the values that will be bound to this array are signed 32 bit integer types. The final part of the type annotation `5` shows the total length of the array. Like I said arrays are fixed in length! In summary, `i32;5` means that `random_numbers` is an array of 5 elements where each element is a 32-bit signed integer. 
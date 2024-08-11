use std::io;
fn main() {
println!("Guess the number!");
println!("Please input your guess.");
let mut guess = String::new();
io::stdin().read_line(&mut guess) .expect("Failed to read line");
println!("You guessed: {}", guess); }

/*
Let’s return to the guessing game program. You now know that let mut guess will introduce a mutable variable named guess. On the other side of the equal sign (=) is the value that guess is bound to, which is the result of calling String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.
To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
Recall that we included the input/output functionality from the stan- dard library with use std::io; on the first line of the program. Now we’ll call an associated function, stdin, on io:
io::stdin().read_line(&mut guess) .expect("Failed to read line");
If we hadn’t listed the use std::io line at the beginning of the program, we could have written this function call as std::io::stdin. The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user. We’re also passing one argument to read_line: &mut guess.
  16 Chapter 2
www.EBooksWorld.ir
The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.
The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex fea- ture, and one of Rust’s major advantages is how safe and easy it is to use ref- erences. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immu- table by default. Hence, you need to write &mut guess rather than &guess to make it mutable. (Chapter 4 will explain references more thoroughly.)
*/
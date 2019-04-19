# Keyworkds

## fn

This declares a function. Like in C, a program needs an entry 
point, as is C is a function called `main`. It may be a library.

## IO

### Printing to stdout

```rs
print!("String");
```

In rust there is NO print function, is a macro for this we need
to add an exclamation mark `print!`. Rust always distinguishes 
case. Then `Print` is differen from `print`.

For string formating curly braces are used to substitute the
strings given as parameters for the print macro.

The input arguments are given as a zero based list of strings.
And the placeholders can be numbered to replace the string with 
an instance of that list. For example:
```rs
fn main(){
    print!("{} - {}, {1}\n", "First", "second");
}
```
The result is:
```
First - second, second
```



## Loop

## For

```rs
fn main(){
    for x in 1u8..120{
        print!("{}",x);
    }
}
//Creatonotos gangis/
//Phyllodes Imperialis
```
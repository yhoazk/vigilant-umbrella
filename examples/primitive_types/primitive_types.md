# Primitive types

* `bool`: either `true` or `false`
* `char`:
* Integer types:
  * Signed `i` unsigned `u`
  * `u|i8`:
  * `u|i16`:
  * `u|i32`:
  * `u|i64`:
  * `u|i128`:
* `u|isize`: Similar to `size_t` and `ssize_t`
* `f32, f64`: Float or double
* `[T:N]`: Fixed size array of type `T` a non-negative constant size `N`
* `[T]`: A dynamically-sized array in a contiguous sequence for any type `T`
* `str`: A string slice tipically used as a reference `&str`
* `(T,U,..)`: A finite sequence where `T` and `U` can be different types
* `fn(T) -> U`: Function that takes a type `T` and returns a type `U`. In rust functions also have a type


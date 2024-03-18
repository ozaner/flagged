# flagged
`flagged` is a Rust library for creating `Flagged` values, that is, values with an associated set of bitflags.

## Purpose
The main use case of the `Flagged` struct is to be able to return values from a function along with associated warnings. There already exist crates that accomplish this like [`warned`](https://crates.io/crates/warned) and [`wresult`](https://crates.io/crates/w_result), but these crates instead represent their warnings as a `Vec`. This means that every call of a function that returns one of these warning types, must also potentially allocate a `Vec`. For many use cases, this seems like overkill. This was the inciting reason for the creation of this crate.

## Installation
The `flagged` crate depends on the [`enumflag2`](https://crates.io/crates/enumflags2) crate, and so both must be included in your `Cargo.toml`:

```toml
[dependencies]
flagged = "0.1.0"
enumflags2 = "0.7.9"
```
*Note that as long as [this issue](https://github.com/meithecatte/enumflags2/issues/48) is still open, `flagged` cannot simply reexport the `enumflags2` crate.*

*Also note that, since `enumflags2` still has a major version of 0, every version change is potentially breaking. As such, using the specific version above is recommended.*

## Usage
To create a flagged value, we first need to define a `BitFlag` [c-like enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html) to use it with:
```rust
#[bitflags]
#[repr(u8)] //bitflags must have an explicit repr
#[derive(Clone, Copy)] //bitflags must impl `Copy`
pub enum ParsingWarning {
    TooLong,
    TooShort,
    InvalidToken,
}
```

With this defined, we can now define a function that returns a `Flagged` value:
```rust
pub fn parse(str: &str) -> Flagged<MyStruct, ParsingWarning> {
    //... rest of the fn
    Flagged {
        value: myStruct,
        flags: myFlags,
    }
}
```

We can then access the value, warnings, as well as perform other operations on the returned `Flagged` value:
```rust
let f = parse(&"Hello, World");

//access wrapped value
println!("{}", f.value);

//access flags
if f.flags.contains(ParsingWarning::InvalidToken) {
    println!("Invalid character found, ignoring!");
}

//turn into a result and error when any flags are set
assert!(f.flags.to_result().is_err());

//turn into a result and error only when certain flags are set
assert!(f.flags.to_result_against(ParsingWarning::TooShort | ParsingWarning::TooLong).is_err());
```

*Note that the `flags` field is of type [`BitFlags`](https://docs.rs/enumflags2/0.7.9/enumflags2/struct.BitFlags.html) from the `enumflag2` crate, you can read more about it in the docs.*

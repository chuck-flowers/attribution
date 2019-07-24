# Attribution

Attribution is a crate that makes parsing attribute-style procedural macros as 
easy as declaring a struct.

## Example Usage

Let's see it in action. Imagine that we want to create a procedural macro 
attribute called `ez_trace`. That adds a custom message to the beginning and 
end of a method call. See the example below for how the usage of this attribute 
would appear.

```rust
#[ez_trace(start = "Starting...", end = "Ending...")]
fn my_func() {
    println!("Do some work");
}
```

In order to parse such an attribute we would need to add the following code to 
our hypothetical procedural macro crate.

**lib.rs**
```rust
#[attr_args]
struct EzTraceArgs {
    start: String,
    end: String
}

fn ez_trace(attr_ts: TokenStream, func_ts: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr_ts as EzTraceArgs);
    /// Remaining macro implementation...
}
```

With the above code, `args` will contain the string values `start` and `end` 
("Starting..." and "Ending..." respectively). To parse additional parameters 
simply add more fields to the `EzTraceArgs` struct. Currently the supported 
types are: `String`, `u64`, and `bool`. Support for more standard library types 
will arrive in the future and the ability to use custom types will also arrive 
later.

## Contributing

If you'd like to support further development of attribution there are a few 
things you can do. Feel free to open an issue [here]
(https://github.com/chuck-flowers/attribution/issues/new) for any bugs you come 
across or any feature requests you have for future releases of attribution. For 
bugs, please include the minimal code required to produce the bug in the issue 
description along with what you expect to happen and what actually happens.

I will be adding a link soon to accept financial donations if you so wish. I 
will continue to work on the project regardless of financial donations but if 
you find attribution useful and would like to buy me a cup of coffee I would 
be very grateful.

I don't intend to accept pull requests at this time because I feel it would be 
unethical to solicit donations and accept pull requests without financially 
compensating the submitter. I don't want to get bogged down in the process of 
determining what a fair share of the donations would be. I also feel the scope 
and codebase of the project is small enough that I should be able to handle any 
issues that appear in a timely manner.
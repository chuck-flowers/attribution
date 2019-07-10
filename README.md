# Attribution

Attribution is a crate containing macros that are designed to help in the 
development of attribute style procedural macros. Specifically it is designed to 
help in the parsing of attribute arguments. Attribution is able to do this 
without the user having to know how to parse the attribute arguments. 
Attribution's goal is to make parsing the attribute as easy as declaring its 
parameters

## Attribution in Action

Here's a quick example how Attribution helps make implementing attribute style 
proc macros easier. For this example, a simple attribute called `ez_log` will be 
used. The purpose of the attribute is to log a message at the beginning of the 
function call. The actual implementation of the proc macro won't be written but 
the logic to parse it attribute arguments using Attribution will be shown.

**DISCLAIMER:** At the time of this writing `ez_log` is not an existing rust 
crate. If that should change in the future and you (as the owner) would like me 
to change this example, please feel free to open an issue.

Let the following be an example usage of this example macro.

```rust
use ez_log::ez_log;

#[ez_log(msg = "inside my_function")]
fn my_function(a: i32, b: i32) -> i32 {
    a + b
}
```

The attribute takes a single parameter, `msg` which is a string. To parse the 
argument within the proc macro write the following.

```rust
use attribution::attribute_args;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[attribute_args]
struct EzLogArgs {
    msg: &'static str
}

fn ez_log(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EzLogArgs);
    // ...
}
```

The `attribute_args` attribute generates the code necessary to parse an 
`EzLogArgs` struct from the `TokenStream` using the 
[parse_macro_input!](https://docs.rs/syn/0.15.39/syn/macro.parse_macro_input.html) 
macro from the [syn](https://crates.io/crates/syn) crate. adding new parameters 
to the macro is as easy as adding new fields to the `EzLogArgs` struct.
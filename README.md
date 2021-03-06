[![Docs](https://docs.rs/enumerate/badge.svg)](https://docs.rs/enumerate/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)]()

# enumerate

This crate provides a procedural macro that, given a trait and its implementers, generates an enum that can then be used in place of a trait object, avoiding dynamic dispatch. The concept is based on that of the [enum_dispatch](https://docs.rs/enum_dispatch) crate, but this implementation focuses on giving the user a more concise API with even less boilerplate necessary.

# Usage

Any trait can be annotated with `#[enumerate(<implementers>)]` to generate a corresponding enum.

# Examples

The following example shows how the macro can be used to generate an enum from a trait and two implementers.

```rust
#[enumerate(Talker, Shouter)]
pub trait SayMessage {
    fn say_message(&self);
}

pub struct Talker {
    message: String
}

impl SayMessage for Talker {
    fn say_message(&self) {
        println!("{}", self.message);
    }
}

pub struct Shouter {
    message: String,
    repetitions: i32
}

impl SayMessage for Shouter {
    fn say_message(&self) {
        for _ in 0..self.repetitions {
            println!("{}", self.message.to_uppercase());
        }
    }
}
```
This will generate an enum with the same name as the trait (`SayMessage` in this case). It will have the variants `Talker` and `Shouter`. Because you obviously can't have two types with the same identifier within the same module, `enumerate` will place the enum in a submodule. The names of these submodules are generated by converting the annotated trait's name to snake_case and appending `_enm`. So the above example will create a module named `say_message_enm`.

## Custom names

It is also possible to specify custom names for both the enum and its variants. Specifying the name of the enum is done by adding `<name>:` as the first argument to the macro. Custom names for the variants can be specified by using the syntax `<implementer> as <name>`. All of this is demonstrated in the example below.

```rust
#[enumerate(InterestingName: ImplementerOne as One, ImplementerTwo as Two)]
pub trait BoringName { /* ... */ }

pub struct ImplementerOne;

impl BoringName for ImplementerOne { /* ... */ }

pub struct ImplementerTwo;

impl BoringName for ImplementerTwo { /* ... */ }
```
This will generate an enum named `InterestingName` with the variants `One` and `Two`. Since there is now no naming conflict between the enum and the trait anymore, no extra module is created and the enum is placed in the same module as the trait.

## Instantiation

`enumerate` automatically generates implementations for `From<T>` for each variant. This means that you can use `<enum>::from(<instance>)` as well as `<instance>.into()` to aquire an instance of the enum from an instance of an implementer of the annotated trait. This is demonstrated below.

```rust
let instance = Implementer::new();
let enum_instance = Enum::from(instance);

let alternative = Implementer::new();
let alt_enum_instance: Enum = alternative.into();

enum_instance.trait_method();
alt_enum_instance.trait_method();
```

# Associated functions

You cannot call associated functions on enums generated by this macro. Generation of the enum will work fine, but calling any associated function on the enum will panic.

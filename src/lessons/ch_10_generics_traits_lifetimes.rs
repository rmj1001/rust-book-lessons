use std::fmt::Display;

pub fn fmt() {
    learning_rust::tui_formatting::module_tui_formatter(
        "Title",
        "Description",
        main,
        Some(learning_rust::tui_formatting::ModuleFlags::Lesson),
    );
}

fn main() {
    generic_types();
    traits();
    lifetimes();
}

fn generic_types() {
    //? Below is an example of a function that takes any type in the parameter:
    //? Note though this will not compile sinc > can't be applied to generic type
    //? references.

    // fn largest<T>(list: &[T]) -> &T {
    //     let mut largest = &list[0];

    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    //? Here's an example of a struct using generic types

    struct Point<T> {
        x: T,
        y: T,
    }

    let will_work = Point { x: 5, y: 6 };
    // let wont_work = Point { x: 6, y: 7.0};

    //? When using generic types, the first value that is cast to
    //? the generic determines the type. If a second value is of a different
    //? type, the code will panic. In this instance, you need to use multiple
    //? generic types:

    struct PointMultipleGenerics<T, U> {
        x: T,
        y: U,
    }

    let will_work_now = PointMultipleGenerics { x: 5.0, y: 6 };

    //? The same rules apply to enum definitions.

    enum Stoplight<T> {
        Go(T),
        SlowDown,
        Stop,
    }

    enum StoplightMultipleGenerics<T, U> {
        Go(T),
        SlowDown(U),
        Stop,
    }

    enum CustomResult<T, E> {
        Ok(T),
        Err(E),
    }

    //? You can also implement generics on struct/enum methods.
    impl<T> Stoplight<T> {
        fn go(output: T) -> T {
            output
        }
    }

    //? You can also define constraints on generics.
    //? The below only allows the impl'd function to be used
    //? on <f32, f32> instances of the struct. Instances of this
    //? type not using <f32, f32> will not have this method implemented.
    impl PointMultipleGenerics<f32, f32> {
        fn print_f32_points(&self) {
            println!("{}, {}", self.x, self.y);
        }
    }

    //? Generics are a zero-cost abstraction.
}

fn traits() {
    //? Traits are functionality shared between multiple generic types.

    //? Types implementing this trait MUST WRITE FUNCTIONALITY FOR
    //? ALL PATHS IN THE TRAIT. Not using {} in the trait means the types
    //? implementing the trait will define their own functionality.
    pub trait Summary {
        fn content(&self) -> String;
        fn summarize(&self) -> String;
    }

    //? Example Structs
    struct Tweet {
        pub username: String,
        pub content: String,
        pub likes: u32,
        pub retweets: u32,
    }

    struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub location: String,
        pub content: String,
        pub url: String,
        pub likes: u32,
        pub shares: u32,
    }

    //? Implementing traits for structs:
    impl Summary for Tweet {
        fn content(&self) -> String {
            self.content.clone()
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Summary for NewsArticle {
        fn content(&self) -> String {
            self.content.clone()
        }

        fn summarize(&self) -> String {
            format!(
                "{}, by {} at {}: {}",
                self.headline, self.author, self.location, self.content
            )
        }
    }

    //? You can also implement default implementations.
    //? DEFAULT TRAIS ARE ALSO NOT REQUIRED TO BE OVERRIDDEN IN
    //? TYPES!
    trait NewSummary {
        fn generalize(&self) -> String {
            "(Read More...)".to_string()
        }
    }

    //? To use default implements, use empty curly brackets.
    impl NewSummary for NewsArticle {}

    //? Default implementations can call other methods in a trait.
    //? NOTE: You cannot call the default implementation from an overriding
    //? implementation of a method.
    trait ThirdNewSummary {
        fn new_summarize(&self);
        fn new_generalize(&self) {
            self.new_summarize();
        }
    }

    //? You can define function parameters that only take generics that
    //? implement a trait. The below will accept any type that implements the
    //? NewSummary trait.
    fn takes_a_traited_generic(the_type: &impl NewSummary) {
        println!("{}", the_type.generalize());
    }

    //? Here is an example where the function takes any type that implements Copy
    fn copier(the_type: &impl Copy) -> impl Copy {
        #[allow(clippy::clone_on_copy)]
        the_type.clone()
    }

    //? The above is just syntactic sugar for the following trait bound syntax:
    fn copier_too<T: Copy>(the_type: &T) -> T {
        #[allow(clippy::clone_on_copy)]
        the_type.clone()
    }

    //? Trait bound syntax makes multiple impl parameters look cleaner:
    fn copier_thwee_long(the_type: &impl Copy, the_type_too: &impl Copy) -> impl Copy {}
    fn copier_thwee<T: Copy>(the_type: &T, the_type_too: &T) -> Vec<T> {
        vec![
            #[allow(clippy::clone_on_copy)]
            the_type.clone(),
            //
            #[allow(clippy::clone_on_copy)]
            the_type_too.clone(),
        ]
    }

    //? You can have parameters require multiple traits:
    fn require_multiple_traits(item: &(impl Summary + NewSummary)) {}
    fn shorter_require<T: Summary + NewSummary>(item: &T) {}

    //? Using multiple traits can look messy so it also provides `where` syntax.
    //? ex: fn function<T: Copy + Display, U: Summary + NewSummary>(item1: &T, item2: &U)
    #[allow(clippy::unused_unit)]
    fn function<T, U>(item1: &T, item2: &U) -> ()
    where
        T: Display + Copy,
        U: Summary + NewSummary,
    {
    }

    //? You can also return values that implement a trait.
    //? HOWEVER: You can only return a single type.
    // ex: fn function() -> impl Copy {}

    //? You can use multiple impl blocks with different trait requirements
    //? to conditionally implement methods based on traits.

    // impl block with no trait requirement (blanket implementation)
    // ex. impl<T> StructName<T>

    // impl block with trait requirement
    // ex: impl<T: std::fmt::Display> StructName<T>
}

fn lifetimes() {
    //? Lifetimes ensure references are valid as long as we want. They also
    //? prevent dangling references (referencing data other than data intended
    //? to be referenced).

    //? Lifetimes are inferred most of the time, but required to be
    //? stated explicitly when lifetimes are related in mulitple ways.

    //? Examples of lifetime annotations:
    // &i32         - reference
    // &'a i32      - reference with lifetime annotation
    // &'a mut i32  - mutable reference with lifetime annotation

    //? Use lifetimes in function parameters:
    fn lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // for the above, the function signature tells Rust that for the lifetime
    // 'a, the function takes two parameters, both of which are string slices
    // that live as long as lifetime 'a. It also tells Rust that the returned
    // string slice will live as long as lifetime 'a.

    // In practice, that means the lifetime of the reference returned by the
    // function is the same as the smallest lifetime in the function's params.

    //? LIFETIMES ARE NOT CHANGED BY ANNOTATION. THE ANNOTATIONS ONLY TELL RUST
    //? THE RELATIONSHIPS BETWEEN THE LIFETIMES.

    // The following is also valid since only the x variable relates to the
    // lifetime of the function
    fn lifetime2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    //? You can use lifetime annotations in structs for references
    struct ImportantException<'a> {
        part: &'a i32,
    }

    let i: i32 = 10;

    let exception: ImportantException<'_> = ImportantException { part: &i };

    println!("{}", exception.part);

    //? ------------------------------------------------------------------------
    //? LIFETIME ELLISION
    //?
    //? Sometimes Rust infers lifetimes automatically, such as in the case of
    //? using string slices. This is based on 3 rules, the first being based
    //? on input lifetimes, the last two on output lifetimes. If the compiler
    //? gets to the end of the three rules and there are still references it
    //? cannot figure out lifetimes for, it will throw an error. This applies to
    //? fn and impl.
    //?
    //? The Rules:
    //? 1. The compiler assigns lifetime parameters to each parameter that's a
    //?    reference. For example:
    fn elided_lifetime1<'a>(n1: &'a i32) {}
    fn elided_lifetime2<'a, 'b>(n1: &'a i32, n2: &'b i32) {} // and more

    //? 2. If there is exactly one input lifetime parameter, that lifetime is
    //?    assigned to all output lifetime parameters.
    // ex. fn elided_lifetime3<'a>(n2: &'a i32) -> &'a i32 { }

    //? 3. If there are multiple input lifetime parameters, but one of them is
    //?    &self or &mut self because this is a method, the lifetime of self is
    //?    assigned to all output lifetimes parameters.
    //? ------------------------------------------------------------------------

    //? STATIC LIFETIMES
    //?
    //? Sometimes references live for the duration of a program. String literals
    //? for example live the full duration of a program's runtime.
    // ex. let string_literal: &'static str = "text";
}

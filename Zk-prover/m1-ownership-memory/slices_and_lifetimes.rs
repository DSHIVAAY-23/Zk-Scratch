// //### Task 4 — `slices_and_lifetimes.rs`
// **Write a function returning `&[u64]` and annotate every lifetime out loud**

// - Write `fn first_half<'a>(data: &'a [u64]) -> &'a [u64]` — explain why `'a` appears on both sides.
// - Write a struct `Coefficients<'a> { data: &'a [u64] }` and a method on it that returns another `&'a [u64]`.
// - Write a second function with two input slices of different lifetimes — fail to compile it, then fix it.
// - Leave a `// EXPLAIN:` comment next to every lifetime annotation describing what it constrains.

// **What to understand**: Lifetime elision rules (when can you omit `'a`?). The NLL (Non-Lexical Lifetimes) mental model. Why the compiler needs lifetime annotations at all — it is not the compiler being pedantic, it is you communicating intent.


//a appears both side coz it implies lifetime elission rule that f there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
fn first_half<'a>(data:&'a [u64]) -> &'a [u64]{
    let mid = data.len() / 2;
    &data[..mid]
}

struct Coefficients<'a> {
    data: &'a [u64],
}

impl<'a> Coefficients<'a>{
    fn first_half(&self) -> &'a [u64]{
        first_half(self.data)
        
    }
}

fn first_half_fail<'a,'b>(a:&'a [u64], b:&'b [u64]) -> &'a [u64]{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}

fn first_half_fix<'a,'b>(a:&'a [u64], b:&'a [u64]) -> &'a [u64]{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}

//we see lifetime rules applied on all 3 functions and methods in methode we use self that is assgned to all as 3rd rule of lifetime ellision  if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters
//in first_half_fail we see that we are returning a or b but we dont know which one so we cant assign a lifetime to it


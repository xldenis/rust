// manually desugared version of an `async fn` (but with a closure instead of a generator)
pub fn a() -> impl Fn() -> u32 {
    || content::doesnt::matter()
    //~^ ERROR failed to resolve
}

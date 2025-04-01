#[allow(dead_code)]
mod static_tests {
    use gc::Gc;
    use gc_derive::{Finalize, Trace};

    #[derive(Trace, Finalize)]
    enum List<A> {
        Cons(A, Box<List<A>>),
        Nil,
    }

    fn test_list() {
        // This fails with during build with the message
        // > overflow evaluating the requirement `static_tests::List<()>: gc::Trace` [E0275]
        Gc::new(List::Nil::<()>);
    }
}

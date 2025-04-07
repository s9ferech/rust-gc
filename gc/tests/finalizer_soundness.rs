use gc::{custom_trace, force_collect, Finalize, Gc, GcCell, Trace};
use std::cell::Cell;

thread_local! {
    static BACKDOOR: Cell<Option<Gc<Dangerous>>> = const { Cell::new(None) };
    static VALUE_GOT_DROPPED: Cell<bool> = const { Cell::new(false) };
}

struct Dangerous {
    self_ref: GcCell<Option<Gc<Dangerous>>>,
}

impl Finalize for Dangerous {
    fn finalize(&self) {
        BACKDOOR.set(Option::<Gc<_>>::clone(&self.self_ref.borrow()));
    }
}

unsafe impl Trace for Dangerous {
    custom_trace!(this, {
        mark(&this.self_ref);
    });
}

impl Drop for Dangerous {
    fn drop(&mut self) {
        VALUE_GOT_DROPPED.set(true);
    }
}

#[test]
fn finalizer_soundness() {
    let dangerous = Gc::new(Dangerous {
        self_ref: GcCell::new(None),
    });
    *dangerous.self_ref.borrow_mut() = Some(Gc::clone(&dangerous));

    drop(dangerous);
    force_collect();

    // This produces an error in Miri:
    let _ = BACKDOOR.take().unwrap().as_ref();

    assert!(!VALUE_GOT_DROPPED.get());
}

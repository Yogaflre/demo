/*
 * Static dispatch
 *
 * strlen function will generate all AsRef type functions when compiled.
 */
fn strlen(s: impl AsRef<str>) -> usize {
    // Or fn strlen<T>(s: T) where T: AsRef<str>
    s.as_ref().len()
}
/*
fn strlen_str(s: &str) -> usize {
    s.len()
}

fn strlen_string(s: String) -> usize {
    s.len()
}

...
*/

/*
 * Dynamic dispatch
 *
 * Use trait object(fat pointer) to save vtable.
 */
trait Hello {
    fn hello(&self);

    fn new()
    where
        Self: Sized, // Make Hello be a object safe trait
    {
    }
}

impl Hello for String {
    fn hello(&self) {
        println!("hello: {}", self);
    }
}

impl Hello for &str {
    fn hello(&self) {
        println!("hello: {}", self);
    }
}

trait HelloAsRef: Hello + AsRef<str> {} // Create new trait to allow multiple traits in vtable
fn hello_asref(t: &dyn HelloAsRef) {}

/*
 * DST object:
 *  1. dyn Trait: (*mut data, *mut vtable)
 *  2. [u8]:      (*mut data, usize)
 *  3. str:       (*mut data, usize)
 */

#[cfg(test)]
mod tests {
    use super::{strlen, Hello};
    use std::mem;

    #[test]
    fn static_dispatch() {
        let s = "length";
        let len = strlen(s); // Actually call strlen_str().
        println!("len: {}", len);
    }

    #[test]
    fn dynamic_dispatch() {
        let s = &"tom";
        let h: &dyn Hello = s as &dyn Hello;
        // Two DST pointer is same.
        assert_eq!(unsafe { mem::transmute::<&dyn Hello, u128>(h) }, unsafe {
            mem::transmute(s as &dyn Hello)
        });
        /*
         * h is a fat pointer. contains:
         *  1. a pointer to a &tom.
         *  2. a pointer to a vtable.
         */
        let (p, vtable): (usize, usize) = unsafe { mem::transmute(h) };
        assert_eq!(p, unsafe { mem::transmute(s) }); // p address == s address

        // TODO
        let v: *const usize = vtable as *const usize;
        unsafe {
            println!(
                "vtable address: [{}, {}, {}, {}]",
                *v,
                *v.offset(1),
                *v.offset(2),
                *v.offset(3),
            );
        }
        h.hello(); // Actually call h.vtable.hello(h.p);
        drop(h); // When h is droped, h must need impl Drop. So h actually is "dyn Hello + Drop", and drop() function in vtable
    }
}

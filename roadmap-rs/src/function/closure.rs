fn foo<T>(_: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::foo;
    use std::mem;

    #[test]
    fn func_size() {
        let f = foo::<i32>; // function
        assert_eq!(mem::size_of_val(&f), 0); // fuction do not have size.
        let fp = f as fn(usize) -> usize; // funcion pointer
        assert_eq!(mem::size_of_val(&fp), 8); // function pointer have size.
        let fpt = &f as &dyn Fn(usize) -> usize; // function trait pointer / Closure
        assert_eq!(mem::size_of_val(&fpt), 16); // function trait pointer is a fat pointer.
    }
}

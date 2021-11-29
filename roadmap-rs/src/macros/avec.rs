#[macro_export] // set macro in crate
macro_rules! avec {
    // expr means anything. Inner brackets "{}" to define a block
    ($($element:expr),*) => {{
        let c: usize = $crate::avec![@COUNT; $($element: expr),*];
        #[allow(unused_mut)]
        let mut v = Vec::with_capacity(c);
        $(v.push($element);)*
        v
    }};
    ($element:expr; $count:expr) => {{
        let mut v = Vec::new();
        v.resize($count, $element); // v.extend(std::iter::repeat($element).take(count));
        v
    }};
    (@COUNT; $($element:expr),*)  => {
        <[()]>::len(&[$($crate::avec![@SUBSTITUE; $element]),*])
    };
    (@SUBSTITUE; $_element:expr) => {
        ()
    };
}

#[macro_export]
macro_rules! max_value {
    ($t: ty) => {{
        <$t>::MAX
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        let v: Vec<i32> = avec![];
        assert!(v.is_empty());
    }

    #[test]
    fn more_vec() {
        let v = avec![7, 8];
        assert!(!v.is_empty());
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 7);
        assert_eq!(v[1], 8);
    }

    #[test]
    fn init_vec() {
        let mut element = Some(0);
        let count = 10;
        let v = avec![element.take().unwrap(); count];
        assert_eq!(v.len(), count);
        for i in 0..10 {
            assert_eq!(v[i], 0);
        }
    }

    #[test]
    fn max_value() {
        assert_eq!(max_value!(i32), i32::MAX);
        assert_eq!(max_value!(u32), u32::MAX);
    }
}

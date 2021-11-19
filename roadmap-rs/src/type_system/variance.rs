use std::marker::PhantomData;

/*
 * In "text: &'a mut &'b str"
 * &'a mut is covariance
 * &'b str is invariance
 * So we need two lifetime.
 */
fn strtok<'a, 'b>(text: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some((f, l)) = text.split_once(delimiter) {
        *text = l;
        return f;
    } else {
        let prefix = *text;
        *text = "";
        return prefix;
    }
}

/*
 * make T covariance
 */
struct Deserializer<T> {
    _t: PhantomData<fn() -> T>,
}

#[cfg(test)]
mod tests {
    use super::strtok;

    #[test]
    fn it_works() {
        let mut text = "hello world";
        let hello: &'static str = strtok(&mut text /* &'text mut &'static text */, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(text, "world");
    }
}

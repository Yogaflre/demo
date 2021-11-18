fn strtok<'a>(text: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some((f, l)) = text.split_once(delimiter) {
        *text = l;
        return f;
    } else {
        let prefix = *text;
        *text = "";
        return prefix;
    }
}

#[cfg(test)]
mod tests {
    use super::strtok;

    // #[test]
    // fn it_works() {
    //     let mut text = "hello world";
    //     let hello = strtok(&mut text, ' ');
    //     assert_eq!(hello, "hello");
    //     assert_eq!(text, "world");
    // }

    #[test]
    fn tmp() {
        let mut s = "h";
        let ms = &mut s;
        // life(&mut s);
        assert_eq!(s, "h");
        assert_eq!(ms, "h");
    }

    fn life<'a>(_: &'a mut &'a str) {}
}

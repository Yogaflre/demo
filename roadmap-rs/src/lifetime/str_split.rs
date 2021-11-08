#[derive(Debug)]
pub struct StrSplit<'text, D> {
    remainder: Option<&'text str>,
    delimiter: D,
}
impl<'text, D> StrSplit<'text, D> {
    pub fn new(text: &'text str, delimiter: D) -> Self {
        return Self {
            remainder: Some(text),
            delimiter,
        };
    }
}

impl<'text, D> Iterator for StrSplit<'text, D>
where
    D: Delimiter,
{
    type Item = &'text str;

    fn next(&mut self) -> Option<Self::Item> {
        // why "let remainder = &mut self.remainder?;" does not work?
        // beacuse self.remainder? will get a copy reference. Not reference of self.remainder
        let remainder/* &mut &'text self.remainder */ = self.remainder.as_mut()?;
        if let Some((start, next)) = self.delimiter.find_next(remainder) {
            let until_delim = &remainder[..start];
            *remainder = &remainder[next..];
            return Some(until_delim);
        } else {
            return self.remainder.take();
        }
    }
}

trait Delimiter {
    fn find_next(&self, text: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, text: &str) -> Option<(usize, usize)> {
        return text.find(self).map(|start| (start, start + self.len()));
    }
}

impl Delimiter for char {
    fn find_next(&self, text: &str) -> Option<(usize, usize)> {
        return text
            .char_indices() // cautions utf-8 char length
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()));
    }
}

#[test]
fn split() {
    let text = "a b c d e";
    let split = StrSplit::new(text, " ");
    assert_eq!(
        split.into_iter().collect::<Vec<_>>(),
        vec!["a", "b", "c", "d", "e"]
    );

    let text = "a 💣 c ";
    let split = StrSplit::new(text, ' ');
    assert_eq!(
        split.into_iter().collect::<Vec<_>>(),
        vec!["a", "💣", "c", ""]
    );
}

fn unitl_char(text: &str, c: char) -> &str {
    return StrSplit::new(text, c).next().expect("Not found.");
}

#[test]
fn until() {
    let text = "a b ";
    assert_eq!(unitl_char(text, ' '), "a");
}

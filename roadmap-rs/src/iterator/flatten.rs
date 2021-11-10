fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator, // beacause params is iter.into_iter(). Could be Iterator if params is iter.iter()
    I::Item: IntoIterator, // because Flatten type O::Item needs IntoIterator
{
    return Flatten::new(iter.into_iter());
}

trait IteratorExt: Iterator {
    fn my_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn my_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        return flatten(self);
    }
}

struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    // if you need both next() and next_back(), you must have two cursor.
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        return Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        };
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // equals  let Some(inner_iter) = self.front_iter.as_mut()
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator, // Flatten.outer needs next_back() function
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator, // Flatten.inner needs next_back() function
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            // equals  let Some(inner_iter) = &mut self.front_iter;
            if let Some(inner_iter) = self.back_iter.as_mut() {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            if let Some(back_inner) = self.outer.next_back() {
                self.back_iter = Some(back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[test]
fn flatten_front() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    assert_eq!(flatten(vec![Vec::<()>::new(), vec![]]).count(), 0);

    assert_eq!(flatten(vec![vec![1, 2]]).count(), 2);
    assert_eq!(flatten(vec![vec![1], vec![2]]).count(), 2);
}

#[test]
fn flatten_rev() {
    assert_eq!(
        flatten(vec![vec![1, 2]]).rev().collect::<Vec<_>>(),
        vec![2, 1]
    );
    assert_eq!(
        flatten(vec![vec![1], vec![2]]).rev().collect::<Vec<_>>(),
        vec![2, 1]
    );
}

#[test]
fn both_ends() {
    let mut iter = flatten(vec![vec![1], vec![2, 3, 4], vec![5]]);
    assert_eq!(iter.next().unwrap(), 1);
    assert_eq!(iter.next_back().unwrap(), 5);
    assert_eq!(iter.next().unwrap(), 2);
    assert_eq!(iter.next_back().unwrap(), 4);

    assert_eq!(iter.next().unwrap(), 3);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn my_flatten() {
    assert_eq!(vec![vec![1], vec![2]].into_iter().my_flatten().count(), 2);
}

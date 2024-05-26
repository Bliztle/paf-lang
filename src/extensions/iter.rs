pub trait IteratorExt: Iterator {
    fn limit_count<P>(self, pred: P, limit: usize) -> LimitCount<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn limit_count<P>(self, pred: P, limit: usize) -> LimitCount<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        LimitCount {
            iter: self,
            pred,
            limit,
            count: 0,
        }
    }
}

pub struct LimitCount<I, P> {
    iter: I,
    pred: P,
    limit: usize,
    count: usize,
}

impl<I, P> Iterator for LimitCount<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            if (self.pred)(&item) {
                self.count += 1;
                if self.count > self.limit {
                    return None;
                }
            }
            return Some(item);
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_limit_count {
        ($name:ident, $src:expr, $pred:expr, $limit:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src: Vec<i32> = $src;
                let pred = $pred;
                let limit = $limit;
                let should_be = $should_be;

                let got: Vec<i32> = src.into_iter().limit_count(pred, limit).collect();
                assert_eq!(got, should_be);
            }
        };
    }

    test_limit_count!(
        test_limit_count,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        |x: &i32| *x % 2 == 0,
        3 => vec![1, 2, 3, 4, 5, 6, 7]
    );
}

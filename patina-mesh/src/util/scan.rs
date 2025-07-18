use std::iter;

pub struct Scan<I, B, F> {
    iter: I,
    accum: Option<B>,
    step: F,
}

pub trait ScanIteratorExt: Iterator + Sized {
    fn scan_full<B, F: for<'a> FnMut(&B, Self::Item) -> B>(
        self,
        init: B,
        step: F,
    ) -> Scan<Self, B, F> {
        Scan {
            iter: self,
            accum: Some(init),
            step,
        }
    }
}

impl<I> ScanIteratorExt for I where I: Iterator {}

impl<I: Iterator, B, F: for<'a> FnMut(&'a B, I::Item) -> B> Iterator for Scan<I, B, F> {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.accum.take()?;
        if let Some(mixin) = self.iter.next() {
            self.accum = Some((self.step)(&old, mixin));
        }
        Some(old)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.accum.is_some() {
            let (min, max) = self.iter.size_hint();
            (min + 1, max.map(|x| x + 1))
        } else {
            (0, Some(0))
        }
    }
}

#[test]
fn test_scan() {
    assert_eq!(
        vec![1],
        [].into_iter()
            .scan_full(1i32, |a, b: i32| *a + b)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![1, 3],
        [2i32]
            .into_iter()
            .scan_full(1i32, |a, b| *a + b)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![1, 3, 13],
        [2i32, 10i32]
            .into_iter()
            .scan_full(1i32, |a, b| *a + b)
            .collect::<Vec<_>>()
    );
}

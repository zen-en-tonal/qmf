use num_traits::Num;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpSampling<I, T>
where
    T: Num,
{
    iter: I,
    scale: usize,
    with: T,
    count: usize,
}

impl<I, T> UpSampling<I, T>
where
    T: Num,
{
    pub fn new(iter: I, scale: usize, with: T) -> UpSampling<I, T> {
        UpSampling {
            iter,
            scale,
            with,
            count: 0,
        }
    }

    pub fn with_zero(iter: I, scale: usize) -> UpSampling<I, T> {
        UpSampling::new(iter, scale, T::zero())
    }
}

impl<I, T> Iterator for UpSampling<I, T>
where
    I: Iterator<Item = T>,
    T: Num + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.count % self.scale == 0 {
            self.iter.next()
        } else {
            Some(self.with.clone())
        };
        self.count = (self.count + 1) % self.scale;
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownSampling<I> {
    iter: I,
    scale: usize,
}

impl<I> DownSampling<I> {
    pub fn new(iter: I, scale: usize) -> Self {
        Self { iter, scale }
    }
}

impl<I> Iterator for DownSampling<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.iter.next();
        for _ in 0..(self.scale - 1) {
            let _ = self.iter.next();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::sampling::DownSampling;

    use super::UpSampling;

    #[test]
    fn test_upsampling() {
        let vec = vec![1, 2, 3];
        let mut iter = UpSampling::with_zero(vec.into_iter(), 2);
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_downsampling() {
        let vec = vec![1, 2, 3];
        let mut iter = DownSampling::new(vec.into_iter(), 2);
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next());
    }
}

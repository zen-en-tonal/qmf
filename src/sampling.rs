use num_traits::Num;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpSampler<T>
where
    T: Num,
{
    scale: usize,
    with: T,
    count: usize,
}

impl<T> UpSampler<T>
where
    T: Num,
{
    pub fn new(scale: usize, with: T) -> UpSampler<T> {
        UpSampler {
            scale,
            with,
            count: 0,
        }
    }

    pub fn with_zero(scale: usize) -> UpSampler<T> {
        UpSampler::new(scale, T::zero())
    }

    pub fn iter<I: Iterator<Item = T>>(&mut self, iter: I) -> UpSampling<'_, I, T> {
        UpSampling {
            iter,
            sampler: self,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UpSampling<'a, I, T>
where
    T: Num,
{
    iter: I,
    sampler: &'a mut UpSampler<T>,
}

impl<'a, I, T> Iterator for UpSampling<'a, I, T>
where
    I: Iterator<Item = T>,
    T: Num + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.sampler.count % self.sampler.scale == 0 {
            self.iter.next()
        } else {
            Some(self.sampler.with.clone())
        };
        if ret.is_some() {
            self.sampler.count = (self.sampler.count + 1) % self.sampler.scale;
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownSampler {
    scale: usize,
    count: usize,
}

impl DownSampler {
    pub fn new(scale: usize) -> Self {
        Self { scale, count: 0 }
    }

    pub fn iter<I: Iterator>(&mut self, iter: I) -> DownSampling<'_, I> {
        DownSampling {
            iter,
            sampler: self,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DownSampling<'a, I> {
    iter: I,
    sampler: &'a mut DownSampler,
}

impl<'a, I> Iterator for DownSampling<'a, I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret: Option<Self::Item> = None;
        for _ in 0..self.sampler.scale {
            let Some(item) = self.iter.next() else {
                break;
            };
            if self.sampler.count == 0 {
                ret = Some(item);
            }
            self.sampler.count = (self.sampler.count + 1) % self.sampler.scale;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::sampling::{DownSampler, UpSampler};

    #[test]
    fn test_upsampling() {
        let vec = vec![1, 2, 3];
        let mut sampler = UpSampler::with_zero(2);
        let mut iter = sampler.iter(vec.into_iter());
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(3), iter.next());

        let vec = vec![4, 5, 6];
        let mut iter = sampler.iter(vec.into_iter());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(5), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(0), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_downsampling() {
        let vec = vec![1, 2, 3];
        let mut sampler = DownSampler::new(2);
        let mut iter = sampler.iter(vec.into_iter());
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next());

        let vec = vec![4, 5, 6];
        let mut iter = sampler.iter(vec.into_iter());
        assert_eq!(Some(5), iter.next());
        assert_eq!(None, iter.next());
    }
}

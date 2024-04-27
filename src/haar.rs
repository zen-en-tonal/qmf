use num_traits::{Float, ToPrimitive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaarFilter<T>
where
    T: Float,
{
    prev: T,
    taps: [T; 2],
}

impl<T> HaarFilter<T>
where
    T: Float,
{
    pub fn new(h0: impl ToPrimitive, h1: impl ToPrimitive) -> Self {
        Self {
            prev: T::zero(),
            taps: [T::from(h0).unwrap(), T::from(h1).unwrap()],
        }
    }

    pub fn consume(&mut self, x: T) -> T {
        let ret = self.taps[0] * x + self.taps[1] * self.prev;
        self.prev = x;
        ret
    }
}

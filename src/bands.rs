use core::array;
use num_traits::Float;

use crate::{
    haar::HaarFilter,
    sampling::{DownSampling, UpSampling},
};

struct Band<T>
where
    T: Float,
{
    in_lowpass_filter: HaarFilter<T>,
    in_highpass_filter: HaarFilter<T>,
    out_lowpass_filter: HaarFilter<T>,
    out_highpass_filter: HaarFilter<T>,
}

impl<T> Band<T>
where
    T: Float,
{
    pub fn new() -> Self {
        // rational number coefficients are taken from
        // [奥村 博造. ハールウェーブレット変換と完全再構成QMフィルタ](https://nagano.repo.nii.ac.jp/record/457/files/nagano_20-04-01.pdf)
        Self {
            in_lowpass_filter: HaarFilter::new(0.5, 0.5),
            in_highpass_filter: HaarFilter::new(-0.5, 0.5),
            out_lowpass_filter: HaarFilter::new(1., 1.),
            out_highpass_filter: HaarFilter::new(1., -1.),
        }
    }

    pub fn analysis(&mut self, xs: &[T]) -> (impl Iterator<Item = T>, impl Iterator<Item = T>) {
        let mut low = alloc::vec::Vec::from(xs);
        let mut high = alloc::vec::Vec::from(xs);
        for (l, h) in core::iter::zip(low.iter_mut(), high.iter_mut()) {
            *l = self.in_lowpass_filter.consume(*l);
            *h = self.in_highpass_filter.consume(*h);
        }
        (
            DownSampling::new(low.into_iter(), 2),
            DownSampling::new(high.into_iter(), 2),
        )
    }

    pub fn synthesis(&mut self, low: &[T], high: &[T], out: &mut [T]) {
        for ((l, h), o) in core::iter::zip(
            UpSampling::with_zero(low.iter().copied(), 2),
            UpSampling::with_zero(high.iter().copied(), 2),
        )
        .zip(out.iter_mut())
        {
            *o = self.out_lowpass_filter.consume(l) + self.out_highpass_filter.consume(h)
        }
    }
}

impl<T> Default for Band<T>
where
    T: Float,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Bands<T, const N: usize>
where
    T: Float,
{
    bands: [Band<T>; N],
}

impl<T, const N: usize> Bands<T, N>
where
    T: Float,
{
    pub fn new() -> Self {
        Self {
            bands: array::from_fn(|_| Band::new()),
        }
    }

    pub fn process<F>(&mut self, buffer: &mut [T], mut closure: F)
    where
        F: FnMut(&mut [T], usize),
    {
        self.process_band(buffer, &mut closure, 0)
    }

    fn process_band<F>(&mut self, buffer: &mut [T], closure: &mut F, count: usize)
    where
        F: FnMut(&mut [T], usize),
    {
        let (lows, highs) = self.bands[count].analysis(buffer);
        let mut lows = lows.collect::<alloc::vec::Vec<T>>();
        let mut highs = highs.collect::<alloc::vec::Vec<T>>();

        if count + 1 >= N {
            closure(lows.as_mut_slice(), count + 1);
        } else {
            self.process_band(lows.as_mut_slice(), closure, count + 1);
        }
        closure(highs.as_mut_slice(), count);

        self.bands[count].synthesis(lows.as_slice(), highs.as_slice(), buffer);
    }

    pub const fn delay(&self) -> usize {
        2_i32.pow(N as u32) as usize
    }
}

impl<T, const N: usize> Default for Bands<T, N>
where
    T: Float,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Bands;

    #[test]
    fn test_bands_reconstruct() {
        let mut bands: Bands<f64, 6> = Bands::new();
        let mut in_data = vec![1.; 1024];
        let out_data = in_data.clone();

        bands.process(in_data.as_mut_slice(), |_d, _c| {});

        assert_eq!(in_data, out_data);
    }
}

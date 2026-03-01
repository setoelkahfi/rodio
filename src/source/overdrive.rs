use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::Source;

/// Internal function that builds an `Overdrive` object.
pub fn overdrive<I>(input: I, gain: f32, color: f32) -> Overdrive<I>
where
    I: Source,
{
    Overdrive { input, gain, color }
}

/// Filter that applies an overdrive effect to the source.
#[derive(Clone, Debug)]
pub struct Overdrive<I> {
    input: I,
    gain: f32,
    color: f32,
}

impl<I> Overdrive<I> {
    /// Modifies the overdrive gain.
    #[inline]
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    /// Modifies the overdrive color.
    #[inline]
    pub fn set_color(&mut self, color: f32) {
        self.color = color;
    }

    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }
}

impl<I> Iterator for Overdrive<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|value| {
            // Simple overdrive: tanh(gain * value) * color + value * (1 - color)
            let g = self.gain;
            let c = self.color.clamp(0.0, 1.0);
            let dry = value;
            let wet = (g * value).tanh();
            (wet * c) + (dry * (1.0 - c))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Overdrive<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Overdrive<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.input.try_seek(pos)
    }
}

//! Implementations of CPU-based (and GPU based in the future) pixel sorting algorithms.

pub mod cpu;

/// A span representing a range of pixels to be sorted.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    /// The line (y-coordinate) of the span.
    pub line: u32,
    /// The start x-coordinate of the span (inclusive).
    pub start: u32,
    /// The end x-coordinate of the span (exclusive).
    pub end: u32,
}

impl Span {
    /// Creates a new `Span`.
    pub fn new(line: u32, start: u32, end: u32) -> Self {
        Span { line, start, end }
    }
}
/// Configuration for pixel sorting algorithms.
#[derive(Debug, Clone, Copy)]
pub struct SortConfig {
    /// Minimum threshold for sorting.
    min_threshold: u8,
    /// Maximum threshold for sorting.
    max_threshold: u8,
}

impl SortConfig {
    /// Sets the minimum and maximum thresholds for sorting.
    ///
    /// Panics if `min` is greater than `max`.
    #[inline]
    pub fn with_thresholds(mut self, min: u8, max: u8) -> Self {
        assert!(
            min <= max,
            "min_threshold must be less than or equal to max_threshold"
        );
        self.min_threshold = min;
        self.max_threshold = max;
        self
    }

    /// Sets the minimum and maximum thresholds for sorting.
    ///
    /// This will swap the values if `min` is greater than `max`.
    #[inline]
    pub fn set_thresholds(&mut self, min: u8, max: u8) {
        self.min_threshold = u8::min(min, max);
        self.max_threshold = u8::max(min, max);
    }

    /// Returns the distance between the minimum and maximum thresholds.
    #[inline(always)]
    pub fn threshold_distance(&self) -> u8 {
        self.max_threshold.saturating_sub(self.min_threshold)
    }

    /// Returns the minimum threshold.
    #[inline(always)]
    pub fn min_threshold(&self) -> u8 {
        self.min_threshold
    }

    /// Returns the maximum threshold.
    #[inline(always)]
    pub fn max_threshold(&self) -> u8 {
        self.max_threshold
    }
}

impl Default for SortConfig {
    fn default() -> Self {
        SortConfig {
            min_threshold: 0,
            max_threshold: 255,
        }
    }
}

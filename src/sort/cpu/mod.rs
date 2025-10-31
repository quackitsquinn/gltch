//! CPU-based pixel sorting.
//!
//! This was more a less just a refresher on how to implement pixel sorting algorithms and will likely
//! be removed in favor of GPU-based sorting in the future.

use iced::widget::image::Handle;
use iced_aw::style::badge::info;
use image::ImageBuffer;
use log::info;

use crate::image::{GlitchImageBuffer, make_handle};

/// CPU-based pixel sorter.
pub struct CpuSorter<V: ValueGenerator> {
    /// Configuration for the sorting algorithm.
    pub config: super::SortConfig,
    current_image: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    phantom: std::marker::PhantomData<V>,
}

impl<V: ValueGenerator> std::fmt::Debug for CpuSorter<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuSorter")
            .field("config", &self.config)
            .finish()
    }
}

impl<V: ValueGenerator> CpuSorter<V> {
    /// Creates a new `CpuSorter` with the given configuration.
    pub fn new(config: super::SortConfig) -> Self {
        CpuSorter {
            config,
            current_image: None,
            phantom: std::marker::PhantomData,
        }
    }

    /// Sorts the given image using the CPU-based algorithm.
    pub fn sort(&mut self, image: &GlitchImageBuffer) -> (GlitchImageBuffer, Handle) {
        self.current_image = Some(
            ImageBuffer::from_raw(image.width(), image.height(), image.as_raw().to_vec())
                .expect("Incorrect size for image!"),
        );
        info!("Generating sort values and spans... this may take a while.");

        self.generate_values();
        let spans = self.generate_spans();
        info!("Sorting {} spans on CPU...", spans.len());
        let image = self.current_image.as_mut().expect("no image");
        for span in spans {
            let span_start = (span.line * image.width() + span.start) as usize;
            let span_end = (span.line * image.width() + span.end) as usize;
            let span_pixels = &mut image.as_mut()[span_start * 4..span_end * 4];
            assert!(
                span_pixels.len() % 4 == 0,
                "Span length is not a multiple of 4!"
            );

            // SAFETY: We have asserted that span_pixels length is a multiple of 4.
            let chunks = unsafe { span_pixels.as_chunks_unchecked_mut::<4>() };

            chunks.sort_by_key(|p| p[3]);
        }

        for p in image.pixels_mut() {
            p.0[3] = 255;
        }

        let image = self.current_image.take().expect("no image");
        let (width, height) = image.dimensions();

        let arc_buffer = GlitchImageBuffer::from_raw(width, height, image.into_raw().into())
            .expect("infallible");
        let handle = make_handle(&arc_buffer);

        (arc_buffer, handle)
    }

    /// Replaces each pixel's alpha channel with the generated value.
    fn generate_values(&mut self) {
        self.current_image
            .as_mut()
            .expect("no image")
            .pixels_mut()
            .for_each(|p| p.0[3] = V::generate([p.0[0], p.0[1], p.0[2]]));
    }

    fn generate_spans(&self) -> Vec<super::Span> {
        let mut spans = self.create_span_vec();
        let (width, height) = self.current_image.as_ref().expect("no image").dimensions();
        for (y, line) in self
            .current_image
            .as_ref()
            .expect("image empty")
            .enumerate_rows()
        {
            let mut span_start: Option<u32> = None;
            for (x, y, &pixel) in line {
                if self.is_in_threshold(pixel.0[3]) {
                    if span_start.is_none() {
                        span_start = Some(x);
                    }
                } else if let Some(start) = span_start {
                    spans.push(super::Span::new(y, start, x));
                    span_start = None;
                }
            }
            if let Some(start) = span_start {
                spans.push(super::Span::new(y, start, width));
            }
        }
        spans
    }

    /// Creates an empty vector with capacity estimation for spans.
    fn create_span_vec(&self) -> Vec<super::Span> {
        let (width, height) = self.current_image.as_ref().expect("no image").dimensions();
        // If the threshold distance is 0, we won't have any spans.
        match self.config.threshold_distance() {
            0 => Vec::new(),
            255 => Vec::with_capacity(height as usize),
            // This isn't the best solution, but for now it's fine, a better equation can be derived later.
            _ => Vec::with_capacity((height * 3) as usize),
        }
    }

    /// Checks if a value is within the configured thresholds.
    // Speed is critical here, so we inline this function.
    #[inline(always)]
    fn is_in_threshold(&self, value: u8) -> bool {
        value >= self.config.min_threshold && value <= self.config.max_threshold
    }
}

impl<V: ValueGenerator> Default for CpuSorter<V> {
    fn default() -> Self {
        Self::new(super::SortConfig::default())
    }
}

/// A trait for generating a value from a pixel for sorting purposes.
pub trait ValueGenerator {
    /// Generates a value from the given pixel.
    fn generate(pixel: [u8; 3]) -> u8;
}

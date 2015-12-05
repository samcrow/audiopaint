use audio_types::*;
use image::{ImageBuffer, Pixel};
use std::ops::Deref;
use num::traits::Bounded;
use num::traits::ToPrimitive;

/// Represents an audio clip in the frequency domain
pub struct FrequencyDomainAudio {
    /// The amplitude at various frequencies over time
    /// Each inner Vec contains the amplitude at various frequencies, on a base-10 log scale,
    /// from high frequency to low frequency
    samples: Vec<Vec<Amplitude>>,
    /// The duration of this audio clip in seconds. Must be positive.
    duration: f64,
    /// The lowest frequency, hertz
    low_frequency: f64,
    /// The highest frequency, hertz
    high_frequency: f64,
}

impl FrequencyDomainAudio {

    /// Creates a FrequencyDomainAudio from an image
    pub fn from_image<P, Container>(image: &ImageBuffer<P, Container>, duration: f64,
        low_frequency: f64, high_frequency: f64) -> FrequencyDomainAudio
        where P: Pixel + 'static, P::Subpixel: 'static, Container: Deref<Target=[P::Subpixel]>
    {
        let mut samples = Vec::with_capacity(image.width() as usize);
        // Create column vectors
        for _ in 0..image.width() {
            let mut column = Vec::with_capacity(image.height() as usize);
            for _ in 0..image.height() {
                column.push(Amplitude::from(0f64));
            }
            assert_eq!(image.height() as usize, column.len());
            samples.push(column);
        }
        assert_eq!(image.width() as usize, samples.len());

        for (x, y, pixel) in image.enumerate_pixels() {
            let luminosity = pixel.to_luma().data[0];
            let ratio = luminosity.to_f64().unwrap() / P::Subpixel::max_value().to_f64().unwrap();
            samples[x as usize][y as usize] = Amplitude::from(ratio);
        }

        FrequencyDomainAudio {
            samples: samples,
            duration: duration,
            low_frequency: low_frequency,
            high_frequency: high_frequency,
        }
    }


    /// Converts this frequency-domain audio clip into a time-domain vector of samples
    /// at the specified sample rate
    pub fn to_time_domain(&self, sample_rate: u32) -> Vec<i32> {
        let sample_count = (self.duration * (sample_rate as f64)) as usize;

        let mut values = Vec::with_capacity(sample_count);
        for i in 0..sample_count {
            let time = self.duration * (i as f64) / (sample_count as f64);
            values.push(self.evaluate(time));
        }

        Self::normalize(&mut values);

        // Convert from Values into i32s
        // TODO: dithering
        let mut samples = Vec::with_capacity(sample_count);
        for value in values {
            assert!(value.abs().value() <= 1f64);
            samples.push((value.value() * i32::max_value() as f64) as i32);
        }
        samples
    }

    /// Evaluates the frequency components of this clip at the specified time. The time must be
    /// less than or equal to self.duration.
    fn evaluate(&self, time: f64) -> Value {
        assert!(time <= self.duration);
        let mut sample = ((time / self.duration) * (self.samples.len() as f64)) as usize;
        if sample >= self.samples.len() {
            sample = self.samples.len() - 1;
        }

        // Calculate frequency ranges
        let log_low = self.low_frequency.log10();
        let log_high = self.high_frequency.log10();
        let log_delta = log_high - log_low;

        let ref frequencies = self.samples[sample];
        let mut i: usize = 0;
        let frequency_count = frequencies.len();
        let mut result = 0f64;
        for amplitude in frequencies {
            // Calculate the frequency
            let ratio = 1f64 - (i as f64) / (frequency_count as f64);
            let log_frequency = ratio * log_delta + log_low;
            let frequency = 10f64.powf(log_frequency);

            // Evaluate
            result += amplitude.value() * (time * frequency).sin();

            i += 1;
        }
        Value::from(result)
    }

    /// Normalizes a slice of samples so that the sample with the largest amplitude
    /// has an amplitude of 1.
    fn normalize(samples: &mut [Value]) {
        // Find maximum
        let max = samples.iter().map(|v| v.abs()).max();
        match max {
            Some(max) => {
                let multiplier = 1f64 / max.value();
                for i in 0..samples.len() {
                    samples[i] = Value::from(multiplier * samples[i].value());
                }
            },
            None => {},
        };
    }
}

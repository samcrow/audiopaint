extern crate clap;
use clap::{Arg, App};

extern crate hound;
extern crate image;
extern crate num;

use std::str::FromStr;
use std::fs::File;

mod audio_types;
mod frequency;
use frequency::FrequencyDomainAudio;

const DEFAULT_LENGTH: f64 = 10f64;
const DEFAULT_SAMPLE_RATE: u32 = 48000;
const LOW_FREQUENCY: f64 = 100f64;

fn main() {
    let matches = App::new("Audiopaint")
        .about("Converts a spectrogram over time (in an image) into an audio file")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .help("The length of the audio file to create (default 10), in seconds")
            .takes_value(true)
            .validator(|string| f64::from_str(&string)
                .map(|_| ())
                .map_err(|e| format!("length must be a number: {}", e) ) ))
        .arg(Arg::with_name("sample rate")
            .short("s")
            .long("samplerate")
            .help("The sample rate to write (default 48000 Hz). The top row of the image corresponds to a frequency of half the sample rate.")
            .takes_value(true)
            .validator(|string| u32::from_str(&string)
                .map(|_| ())
                .map_err(|e| format!("length must be an integer: {}", e) ) ))
        .arg(Arg::with_name("input file")
            .short("i")
            .long("in")
            .help("The image to read")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output file")
            .short("o")
            .long("out")
            .help("The file to write")
            .takes_value(true)
            .required(true))
        .get_matches();


    let length = matches.value_of("length")
        .map_or(DEFAULT_LENGTH, |lstr| f64::from_str(lstr).unwrap());
    let sample_rate = matches.value_of("sample rate")
        .map_or(DEFAULT_SAMPLE_RATE, |lstr| u32::from_str(lstr).unwrap());
    let in_file_name = matches.value_of("input file").unwrap();
    let out_file_name = matches.value_of("output file").unwrap();

    let image = image::open(in_file_name);
    match image {
        Ok(image) => {
            let audio = match image {
                image::DynamicImage::ImageLuma8(image) => FrequencyDomainAudio::from_image(&image, length, LOW_FREQUENCY, sample_rate as f64 / 2f64),
                image::DynamicImage::ImageLumaA8(image) => FrequencyDomainAudio::from_image(&image, length, LOW_FREQUENCY, sample_rate as f64 / 2f64),
                image::DynamicImage::ImageRgb8(image) => FrequencyDomainAudio::from_image(&image, length, LOW_FREQUENCY, sample_rate as f64 / 2f64),
                image::DynamicImage::ImageRgba8(image) => FrequencyDomainAudio::from_image(&image, length, LOW_FREQUENCY, sample_rate as f64 / 2f64),
            };

            let samples = audio.to_time_domain(sample_rate);
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: sample_rate,
                bits_per_sample: 32,
            };
            match File::create(out_file_name) {
                Ok(file) => {
                    let mut writer = hound::WavWriter::new(file, spec);
                    for sample in samples {
                        let result = writer.write_sample(sample);
                        if let Err(e) = result {
                            println!("Sample write error: {}", e);
                        }
                    }

                    let finalize_result = writer.finalize();
                    if let Err(e) = finalize_result {
                        println!("Output finalize error: {}", e);
                    }
                },
                Err(e) => println!("Failed to open output file: {}", e),
            }
        },
        Err(e) => println!("Failed to open image: {}", e),
    }
}

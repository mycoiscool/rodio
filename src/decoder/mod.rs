use std::io::{Read, Seek};

use Sample;
use Source;

mod vorbis;
mod wav;

/// Source of audio samples from decoding a file.
pub struct Decoder<R>(DecoderImpl<R>) where R: Read + Seek;

enum DecoderImpl<R> where R: Read + Seek {
    Wav(wav::WavDecoder<R>),
    Vorbis(vorbis::VorbisDecoder),
}

impl<R> Decoder<R> where R: Read + Seek + Send + 'static {
    pub fn new(data: R) -> Decoder<R> {
        let data = match wav::WavDecoder::new(data) {
            Err(data) => data,
            Ok(decoder) => {
                return Decoder(DecoderImpl::Wav(decoder));
            }
        };

        if let Ok(decoder) = vorbis::VorbisDecoder::new(data) {
            return Decoder(DecoderImpl::Vorbis(decoder));
        }

        panic!("Invalid format");
    }
}

impl<R> Iterator for Decoder<R> where R: Read + Seek {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        match self.0 {
            DecoderImpl::Wav(ref mut source) => source.next().map(|s| s.to_f32()),
            DecoderImpl::Vorbis(ref mut source) => source.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.size_hint(),
            DecoderImpl::Vorbis(ref source) => source.size_hint(),
        }
    }
}

// TODO: ExactSizeIterator

impl<R> Source for Decoder<R> where R: Read + Seek {
    #[inline]
    fn get_current_frame_len(&self) -> usize {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.get_current_frame_len(),
            DecoderImpl::Vorbis(ref source) => source.get_current_frame_len(),
        }
    }

    #[inline]
    fn get_channels(&self) -> u16 {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.get_channels(),
            DecoderImpl::Vorbis(ref source) => source.get_channels(),
        }
    }

    #[inline]
    fn get_samples_rate(&self) -> u32 {
        match self.0 {
            DecoderImpl::Wav(ref source) => source.get_samples_rate(),
            DecoderImpl::Vorbis(ref source) => source.get_samples_rate(),
        }
    }
}

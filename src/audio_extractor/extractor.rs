extern crate ffmpeg_next as ffmpeg;

use std::path::Path;

use ffmpeg::{codec, filter, format, frame, media};
use ffmpeg::{rescale, Rescale};

struct Transcoder {
    stream: usize,
    decoder: codec::decoder::Audio,
    encoder: codec::encoder::Audio,
    in_time_base: ffmpeg::Rational,
    out_time_base: ffmpeg::Rational,
}

fn transcode(
    input_format: &mut format::context::Input,
    output_format: &mut format::context::Output,
    output: &Path,
) -> Result<Transcoder, ffmpeg::Error> {
    let input = input_format
        .streams()
        .best(ffmpeg_next::media::Type::Audio)
        .expect("could not find best audio stream");

    let input_context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;

    let mut decoder = input_context.decoder().audio()?;

    let output_codec =
        ffmpeg::encoder::find(output_format.format().codec(&output, media::Type::Audio))
            .expect("failed to find encoder")
            .audio()?;

    let global = output_format
        .format()
        .flags()
        .contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);

    decoder.set_parameters(input.parameters())?;

    let mut output = output_format.add_stream(output_codec)?;
    let output_context = ffmpeg::codec::context::Context::from_parameters(output.parameters())?;
    let mut encoder = output_context.encoder().audio()?;

    let channel_layout = output_codec
        .channel_layouts()
        .map(|layout| layout.best(decoder.channel_layout().channels()))
        .unwrap_or(ffmpeg::channel_layout::ChannelLayout::STEREO);

    if global {
        encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
    }

    encoder.set_rate(decoder.rate() as i32);
    encoder.set_channel_layout(channel_layout);
    encoder.set_channels(channel_layout.channels());
    encoder.set_format(
        output_codec
            .formats()
            .expect("unknown supported formats")
            .next()
            .unwrap(),
    );
    encoder.set_bit_rate(decoder.bit_rate());
    encoder.set_max_bit_rate(decoder.max_bit_rate());
    encoder.set_time_base((1, decoder.rate() as i32));
    output.set_time_base((1, decoder.rate() as i32));

    let encoder = encoder.open_as(output_codec)?;
    output.set_parameters(&encoder);

    let in_time_base = decoder.time_base();
    let out_time_base = output.time_base();

    Ok(Transcoder {
        stream: input.index(),
        decoder,
        encoder,
        in_time_base,
        out_time_base,
    })
}

pub fn extract(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init().unwrap();
    let mut input_format = ffmpeg::format::input(&input).unwrap();
    let mut output_format = ffmpeg::format::output(&output).unwrap();
    let mut transcoder = transcode(&mut input_format, &mut output_format, &output).unwrap();

    Ok(())
}

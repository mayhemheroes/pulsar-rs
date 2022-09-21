#![no_main]
use asynchronous_codec::{Decoder as AsyncDecoder, Encoder as AsyncEncoder};
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use pulsar::message::Codec;
use tokio_util::codec::{Decoder as TokioDecoder, Encoder as TokioEncoder};

fuzz_target!(|data: &[u8]| {
    _ = fuzz(data);
});

fn fuzz(data: &[u8]) -> Result<(), ()> {
    let mut bytes_t = BytesMut::from(data);
    let mut bytes_a = BytesMut::from(data);

    let mut codec_t = Codec;
    let mut codec_a = Codec;

    // decode a message with each impl, and assert equal result
    let decoded_t = <Codec as TokioDecoder>::decode(&mut codec_t, &mut bytes_t);
    let decoded_a = <Codec as AsyncDecoder>::decode(&mut codec_a, &mut bytes_a);
    match (&decoded_t, &decoded_a) {
        (Ok(_), Ok(_)) => {}
        (Err(_), Err(_)) => return Ok(()),
        _ => panic!("decoder impls disagree"),
    }
    let decoded = decoded_t.unwrap().ok_or(())?;

    // encode the message with each impl, and assert equal result
    bytes_t.clear();
    bytes_a.clear();
    <Codec as TokioEncoder<_>>::encode(&mut codec_t, decoded.clone(), &mut bytes_t)
        .expect("Tokio encoder failed to encode");
    <Codec as AsyncEncoder>::encode(&mut codec_a, decoded.clone(), &mut bytes_a)
        .expect("Async encoder failed to encode");
    assert_eq!(&bytes_t, &bytes_a, "encoder impls disagree");

    // decode the encoded bytes, asserting both impls can decode something they encode
    match <Codec as TokioDecoder>::decode(&mut codec_t, &mut bytes_t) {
        Ok(Some(_)) => {}
        _ => panic!("Tokio decoder failed to roundtrip"),
    };
    match <Codec as AsyncDecoder>::decode(&mut codec_a, &mut bytes_a) {
        Ok(Some(_)) => {}
        _ => panic!("Async decoder failed to roundtrip"),
    };

    Ok(())
}

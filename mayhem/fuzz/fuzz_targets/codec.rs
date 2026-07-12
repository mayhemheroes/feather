#![no_main]
use libfuzzer_sys::fuzz_target;
use feather_protocol::MinecraftCodec;
use feather_protocol::ClientPlayPacket;

fuzz_target!(|data: (bool, &[u8])| {
    let (compress, data) = data;
    let mut codec = MinecraftCodec::new();
    if compress {
        codec.enable_compression(1);
    }

    codec.accept(data);
    let _: Result<Option<ClientPlayPacket>, _> = codec.next_packet(); 
});

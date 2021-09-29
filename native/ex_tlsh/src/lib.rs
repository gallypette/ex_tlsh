use tlsh::{BucketKind, ChecksumKind, TlshBuilder};
use rustler::Binary;

#[rustler::nif]
pub fn hash(b64: Binary) -> String {
    let mut builder = TlshBuilder::new(
        BucketKind::Bucket128,
        ChecksumKind::OneByte,
        tlsh::Version::Version4,
     );
     builder.update(&b64);
     let tlsh1 = builder.build().unwrap();
     tlsh1.hash()
}

rustler::init!("Elixir.ExTlsh", [hash]);
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct SpeciesFlags: u8 {
        const DEFAULT_FORM = 0b0000_0001;
        const BATTLE_ONLY = 0b0000_0010;
        const FORM_SWITCHABLE = 0b0000_0100;
        const MEGA = 0b0000_1000;
        const GMAX = 0b0001_0000;
        const LEGENDARY = 0b0010_0000;
        const MYTHICAL = 0b0100_0000;
    }
}

#[cfg(feature = "bitcode")]
mod codec {
    use crate::types::species_flags::SpeciesFlags;
    use bitcode::__private;
    use bitcode::__private::{Buffer, Decoder, Encoder, View};
    use std::mem::MaybeUninit;
    use std::num::NonZeroUsize;

    // Encode
    impl bitcode::Encode for SpeciesFlags {
        type Encoder = SpeciesFlagsEncoder;
    }

    #[derive(Default)]
    pub struct SpeciesFlagsEncoder {
        inner: <u8 as bitcode::Encode>::Encoder,
    }

    impl Encoder<SpeciesFlags> for SpeciesFlagsEncoder {
        fn encode(&mut self, v: &SpeciesFlags) {
            self.inner.encode(&v.bits());
        }

        fn encode_vectored<'v>(&mut self, i: impl Iterator<Item = &'v SpeciesFlags> + Clone)
        where
            SpeciesFlags: 'v,
        {
            for v in i {
                self.encode(v);
            }
        }
    }

    impl Buffer for SpeciesFlagsEncoder {
        fn collect_into(&mut self, out: &mut __private::Vec<u8>) {
            self.inner.collect_into(out);
        }

        fn reserve(&mut self, additional: NonZeroUsize) {
            self.inner.reserve(additional);
        }
    }

    // Decode
    impl<'de> bitcode::Decode<'de> for SpeciesFlags {
        type Decoder = SpeciesFlagsDecoder<'de>;
    }

    #[derive(Default)]
    pub struct SpeciesFlagsDecoder<'de> {
        inner: <u8 as bitcode::Decode<'de>>::Decoder,
    }

    impl<'de> View<'de> for SpeciesFlagsDecoder<'de> {
        fn populate(&mut self, input: &mut &'de [u8], length: usize) -> __private::Result<()> {
            self.inner.populate(input, length)
        }
    }

    impl<'de> Decoder<'de, SpeciesFlags> for SpeciesFlagsDecoder<'de> {
        fn decode_in_place(&mut self, out: &mut MaybeUninit<SpeciesFlags>) {
            let mut raw = MaybeUninit::<u8>::uninit();
            self.inner.decode_in_place(&mut raw);
            let bits = unsafe { raw.assume_init() };
            out.write(SpeciesFlags::from_bits_truncate(bits));
        }
    }
}

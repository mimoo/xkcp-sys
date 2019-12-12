mod c_stuff {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone)]
    struct KeccakWidth1600_SpongeInstance {
        state: [libc::c_uchar; 200],
        rate: libc::c_uint,
        byteIOIndex: libc::c_uint,
        squeezing: libc::c_int,
    }

    impl Default for KeccakWidth1600_SpongeInstance {
        fn default() -> Self {
            Self {
                state: [0u8; 200],
                rate: libc::c_uint::default(),
                byteIOIndex: libc::c_uint::default(),
                squeezing: libc::c_int::default(),
            }
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    enum KCP_Phases {
        NOT_INITIALIZED,
        ABSORBING,
        FINAL,
        SQUEEZING
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct cSHAKE_Instance {
        sponge: KeccakWidth1600_SpongeInstance,
        fixedOutputLength: libc::size_t,
        lastByteBitLen: libc::c_uint,
        lastByteValue: libc::c_uchar,
        emptyNameCustom: libc::c_int,
        phase: KCP_Phases,
    }

    impl Default for cSHAKE_Instance {
        fn default() -> Self {
            Self {
                sponge: KeccakWidth1600_SpongeInstance::default(),
                fixedOutputLength: libc::size_t::default(),
                lastByteBitLen: libc::c_uint::default(),
                lastByteValue: libc::c_uchar::default(),
                emptyNameCustom: libc::c_int::default(),
                phase: KCP_Phases::NOT_INITIALIZED,
            }
        }
    }


    #[link(name = "keccak")]
    extern "C" {
        pub fn cSHAKE128(
            input: *const libc::c_uchar,
            inputBitLen: libc::size_t,
            output: *mut libc::c_uchar,
            outputBitLen: libc::size_t,
            name: *const libc::c_uchar,
            nameBitLen: libc::size_t, 
            customization: *const libc::c_uchar,
            customBitLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn cSHAKE128_Initialize(
            cskInstance: *mut cSHAKE_Instance,
            outputBitLen: libc::size_t,
            name: *const libc::c_uchar,
            nameBitLen: libc::size_t,
            customization: *const libc::c_uchar,
            customBitLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn cSHAKE128_Update(
            cskInstance: *mut cSHAKE_Instance,
            input: *const libc::c_uchar,
            inputBitLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn cSHAKE128_Final(
            cskInstance: *mut cSHAKE_Instance,
            output: *mut libc::c_uchar,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn cSHAKE128_Squeeze(
            cskInstance: *mut cSHAKE_Instance,
            output: *mut libc::c_uchar,
            outputBitLen: libc::size_t,
        ) -> libc::c_int;
    }
}

pub fn cshake128(customization: &[u8], input: &[u8], output_len: usize) -> Vec<u8> {
    let mut output = vec![0; output_len];
    unsafe {
        assert_eq!(0,
            c_stuff::cSHAKE128(
                input.as_ptr(),
                // NIST API is in bits instead of bytes
                (input.len() * 8) as libc::size_t,
                output.as_mut_ptr(),
                (output_len * 8) as libc::size_t,
                // name is empty
                vec![].as_ptr(), 
                0 as libc::size_t, 
                customization.as_ptr(),
                (customization.len() * 8) as libc::size_t,
        ));
    }
    output
}

#[derive(Clone)]
pub struct CShake(c_stuff::cSHAKE_Instance);

impl CShake {
    pub fn new(customization: &[u8]) -> Self {
        let mut state = c_stuff::cSHAKE_Instance::default();
        unsafe {
            assert_eq!(0, c_stuff::cSHAKE128_Initialize(
                &mut state,
                (32 * 8) as libc::size_t,
                vec![].as_ptr(), // name is empty
                0 as libc::size_t,
                customization.as_ptr(),
                (customization.len() * 8) as libc::size_t,
            ));
        }
        Self(state)
    }

    pub fn update(&mut self, input: &[u8]) {
        unsafe {
            assert_eq!(0, c_stuff::cSHAKE128_Update(
                &mut self.0,
                input.as_ptr(),
                (input.len() * 8) as libc::size_t,
            ));
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        let mut output = vec![0; 32];
        unsafe {
            assert_eq!(0, c_stuff::cSHAKE128_Final(
                &mut self.0,
                output.as_mut_ptr(),
            ));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shake128() {
        let digest = cshake128(b"testing", b"someinput", 32);

        assert_eq!(digest, [169, 78, 48, 230, 118, 51, 183, 191, 229, 68, 138, 32, 153, 195, 93, 64, 169, 233, 231, 33, 211, 139, 46, 69, 29, 202, 109, 184, 29, 148, 143, 93]);

        let mut state = CShake::new(b"testing");
        state.update(b"someinput");
        let digest2 = state.finalize();

        assert_eq!(digest, digest2);
    }
}

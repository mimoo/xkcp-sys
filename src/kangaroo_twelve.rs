mod c_stuff {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    
    #[repr(C)]
    #[repr(align(8))]
    #[derive(Clone)]
    struct KeccakWidth1600_12rounds_SpongeInstance {
        state: [libc::c_uchar; 200],
        rate: libc::c_uint,
        byteIOIndex: libc::c_uint,
        squeezing: libc::c_int,
    }

    impl Default for KeccakWidth1600_12rounds_SpongeInstance {
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
    enum KangarooTwelve_Phases {
        NOT_INITIALIZED,
        ABSORBING,
        FINAL,
        SQUEEZING
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct KangarooTwelve_Instance {
        queueNode: KeccakWidth1600_12rounds_SpongeInstance,
        finalNode: KeccakWidth1600_12rounds_SpongeInstance,
        fixedOutputLength: libc::size_t,
        blockNumber: libc::size_t,
        queueAbsorbedLen: libc::c_uint,
        phase: KangarooTwelve_Phases,
    }

    impl Default for KangarooTwelve_Instance {
        fn default() -> Self {
            Self {
                queueNode: KeccakWidth1600_12rounds_SpongeInstance::default(),
                finalNode: KeccakWidth1600_12rounds_SpongeInstance::default(),
                fixedOutputLength: libc::size_t::default(),
                blockNumber: libc::size_t::default(),
                queueAbsorbedLen: libc::c_uint::default(),
                phase: KangarooTwelve_Phases::NOT_INITIALIZED,
            }
        }
    }


    #[link(name = "keccak")]
    extern "C" {
        pub fn KangarooTwelve(
            input: *const libc::c_uchar,
            inputByteLen: libc::size_t,
            output: *mut libc::c_uchar,
            outputByteLen: libc::size_t,
            customization: *const libc::c_uchar,
            customByteLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn KangarooTwelve_Initialize(
            ktInstance: *mut KangarooTwelve_Instance,
            outputByteLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn KangarooTwelve_Update(
            ktInstance: *mut KangarooTwelve_Instance,
            input: *const libc::c_uchar,
            inputByteLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn KangarooTwelve_Final(
            ktInstance: *mut KangarooTwelve_Instance,
            output: *mut libc::c_uchar,
            customization: *const libc::c_uchar,
            customByteLen: libc::size_t,
        ) -> libc::c_int;

        #[link(name = "keccak")]
        pub fn KangarooTwelve_Squeeze(
            ktInstance: *mut KangarooTwelve_Instance,
            output: *mut libc::c_uchar,
            outputByteLen: libc::size_t,
        ) -> libc::c_int;

    }
}

pub fn kangaroo_twelve(customization: &[u8], input: &[u8], output_len: usize) -> Vec<u8> {
    let mut output = vec![0; output_len];
    unsafe {
        assert_eq!(0, c_stuff::KangarooTwelve(
            input.as_ptr(),
            input.len() as libc::size_t,
            output.as_mut_ptr(),
            output_len as libc::size_t,
            customization.as_ptr(),
            customization.len() as libc::size_t,
        ));
    }
    output
}

#[derive(Clone)]
pub struct KangarooTwelve{
    state: c_stuff::KangarooTwelve_Instance,
    custom: Vec<u8>,
    output_len: usize,
}

impl KangarooTwelve {
    pub fn new(customization: &[u8], output_len: usize) -> Self {
        let mut state = c_stuff::KangarooTwelve_Instance::default();
        unsafe {
            assert_eq!(0, c_stuff::KangarooTwelve_Initialize(
                &mut state,
                output_len as libc::size_t,
            ));
        }
        Self {
            state: state,
            custom: customization.to_vec(),
            output_len: output_len,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        unsafe {
            assert_eq!(0, c_stuff::KangarooTwelve_Update(
                &mut self.state,
                input.as_ptr(),
                input.len() as libc::size_t,
            ));
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        let mut output = vec![0; self.output_len];
        unsafe {
            assert_eq!(0, c_stuff::KangarooTwelve_Final(
                &mut self.state,
                output.as_mut_ptr(),
                self.custom.as_ptr(),
                self.custom.len() as libc::size_t,
            ));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k12() {
        let digest = kangaroo_twelve(b"testing", b"someinput", 16);

        assert_eq!(
            digest,
            [187, 19, 67, 214, 73, 178, 187, 16, 174, 135, 82, 238, 25, 49, 129, 242]
        );

        let mut state = KangarooTwelve::new(b"testing", 16);
        state.update("someinput".as_bytes());
        let digest2 = state.finalize();

        assert_eq!(digest, digest2);
    }   
}

#[link(name = "keccak")]
extern "C" {
    /*
    fn cSHAKE128(
        input: *const u8,
        inputBitLen: size_t,
        output: *mut u8,
        outputBitLen: size_t,
        name: *const u8,
        nameBitLen: size_t,
        customization: *const u8,
        customBitLen: size_t,
    ) -> c_int;
    */

    fn KangarooTwelve(
        input: *const libc::c_uchar,
        inputByteLen: libc::size_t,
        output: *mut libc::c_uchar,
        outputByteLen: libc::size_t,
        customization: *const libc::c_uchar,
        customByteLen: libc::size_t,
    ) -> libc::c_int;
}

pub fn rust_k12(customization: &[u8], input: &[u8], output_len: usize) -> Vec<u8> {
    //    let mut output = Vec::with_capacity(output_len);
    let mut output = vec![0; output_len];
    unsafe {
        KangarooTwelve(
            input.as_ptr(),
            input.len() as libc::size_t,
            output.as_mut_ptr(),
            output_len as libc::size_t,
            customization.as_ptr(),
            customization.len() as libc::size_t,
        );
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shake128() {
        let digest = rust_k12("testing".as_bytes(), "someinput".as_bytes(), 16);
        println!("K12('testing', 'someinput', 16):  {:?}", digest);

        assert_eq!(
            digest,
            [187, 19, 67, 214, 73, 178, 187, 16, 174, 135, 82, 238, 25, 49, 129, 242]
        );
    }
}

use std::ffi::{CStr, CString};

extern "C" {
    fn ivy_eval(
        s: *const i8,
        buf: *mut u8,
        buflen: isize,
        errbuf: *mut u8,
        errbuflen: isize,
    ) -> isize;
}

pub fn eval(s: &str) -> Result<String, String> {
    let mut buf = vec![0; 128];
    let mut errbuf = vec![0; 128];
    let res = unsafe {
        ivy_eval(
            CString::new(s).unwrap().as_ptr(),
            buf.as_mut_ptr(),
            buf.len() as isize,
            errbuf.as_mut_ptr(),
            errbuf.len() as isize,
        )
    };
    // There should be no need for the below to be unsafe, but CStr only stops at NULL with from_ptr.
    if res != 0 {
        Err((unsafe { CStr::from_ptr(errbuf.as_ptr() as *const i8) })
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Ok((unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) })
            .to_str()
            .unwrap()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_addition() {
        assert_eq!(crate::eval("1+2+3+4"), Ok("10".to_string()));
    }

    #[test]
    fn empty_string() {
        assert_eq!(crate::eval(""), Ok("".to_string()));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(
            crate::eval(")"),
            Err("<args>:1: expected [Identifier Op], got EOF".to_string())
        );
    }

    #[test]
    fn fractional() {
        assert_eq!(crate::eval("1/2"), Ok("0.5".to_string()));
    }
}

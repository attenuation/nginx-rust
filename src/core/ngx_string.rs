use std::str;
use std::slice;
use std::fmt;
use std::ptr::copy_nonoverlapping;

use crate::bindings::{
    ngx_str_t, ngx_pool_t, ngx_palloc, u_char, ngx_create_pool, strcmp
};

fn str_to_uchar(pool: *mut ngx_pool_t, data: &str) -> *mut u_char {
    let ptr: *mut u_char = unsafe { ngx_palloc(pool, data.len() as _) as _ };
    unsafe {
        copy_nonoverlapping(data.as_ptr(), ptr, data.len());
    }
    ptr
}

impl ngx_str_t {
    // convert nginx string to str slice
    pub fn to_str(&self) -> &str  {

        unsafe {
            let slice = slice::from_raw_parts(self.data,self.len as _) ;
            return str::from_utf8(slice).unwrap();
        }            
   
    }

    // get string 
    pub fn to_string(&self) -> String  {
        return String::from(self.to_str());
    }

    // from string
    pub fn from_string(pool: *mut ngx_pool_t, data: String) -> Self {
        ngx_str_t {
            data: str_to_uchar(pool, data.as_str()),
            len: data.len() as _,
        }
    }
}

impl fmt::Display for ngx_str_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


impl PartialEq for ngx_str_t {
    fn eq(&self, other: &Self) -> bool {
        if self.len == other.len {
            if self.len == 0 {
                return true;
            }
            unsafe {
                return strcmp(self.data as _, other.data as _) == 0;
            }
        }
        return false;
    }
}

impl fmt::Debug for ngx_str_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ngx_str_t")
         .field("data", &self.to_str())
         .field("len", &self.len)
         .finish()
    }
}

#[macro_export]
macro_rules! ngx_string {
    ($s:expr) => {
        {
            ngx_str_t { len: $s.len() as _, data: concat!($s, "\0").as_ptr() as *mut u8 }
        }
    };
}

#[macro_export]
macro_rules! ngx_null_string {
    () => {
        ngx_str_t { len: 0, data: ::std::ptr::null_mut() }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_string() {
        let null_string = ngx_null_string!();
        let a = ngx_str_t { len: 0, data: ::std::ptr::null_mut() };
        assert_eq!(null_string, a);
    }

    #[test]
    fn test_string() {
        let string = ngx_string!("Hello, world!");
        let mut hello = String::from("Hello, world!");
        let a = ngx_str_t {
            len: (hello.len()) as u64,
            data: hello.as_mut_ptr()
        };
        assert_eq!(string, a);
        let string = ngx_string!("Hello, world");
        let mut hello = String::from("Hello, world!");
        let a = ngx_str_t {
            len: (hello.len()) as u64,
            data: hello.as_mut_ptr()
        };
        assert_ne!(string, a);
    }


    #[test]
    fn test_to_string() {
        let mut hello = String::from("Hello, world!");
        let a = ngx_str_t {
            len: (hello.len()) as u64,
            data: hello.as_mut_ptr()
        };
        assert_eq!(hello, a.to_string());
    }

    #[test]
    fn test_to_str() {
        let a = ngx_string!("hello, world!");
        assert_eq!(a.to_str(), "hello, world!")
    }
}
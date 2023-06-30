// Copyright (C) 2020 Alibaba Cloud. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

use std::io::Result;
use std::time::SystemTime;

use libc::c_char;
use lz4_sys::{LZ4_compressBound, LZ4_compress_default, LZ4_decompress_safe};

static mut com_timestamp: u128 = 0;
static mut decom_timestamp: u128 = 0;
static mut com_size: u128 = 0;
static mut decom_size: u128 = 0;
static mut dec_in_size: u128 = 0;

pub(super) fn lz4_compress(src: &[u8]) -> Result<Vec<u8>> {
    // 0 iff src too large
    let compress_bound: i32 = unsafe { LZ4_compressBound(src.len() as i32) };

    if src.len() > (i32::max_value() as usize) || compress_bound <= 0 {
        return Err(einval!("compression input data is too big"));
    }

    let mut dst_buf = Vec::with_capacity(compress_bound as usize);

    let com_now = SystemTime::now();
    let cmp_size = unsafe {
        LZ4_compress_default(
            src.as_ptr() as *const c_char,
            dst_buf.as_mut_ptr() as *mut c_char,
            src.len() as i32,
            compress_bound,
        )
    };
    if cmp_size <= 0 {
        return Err(eio!("compression failed"));
    }
    match com_now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            unsafe {
                com_timestamp += elapsed.as_nanos();
                com_size += src.len() as u128;
                decom_size += cmp_size as u128;
                info!(
                    "lz4_compress com_timestamp= {}, com_size= {}, decom_size= {}",
                    com_timestamp, com_size, decom_size
                );
            }
        }
        Err(_e) => {
            // an error occurred!
            return Err(einval!("timestamp error"));
        }
    };

    assert!(cmp_size as usize <= dst_buf.capacity());
    unsafe { dst_buf.set_len(cmp_size as usize) };

    Ok(dst_buf)
}

pub(super) fn lz4_decompress(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    if dst.len() >= std::i32::MAX as usize {
        return Err(einval!("the destination buffer is big than i32::MAX"));
    }
    let size = dst.len() as i32;

    if unsafe { LZ4_compressBound(size) } <= 0 {
        return Err(einval!("given size parameter is too big"));
    }

    let dcom_now = SystemTime::now();
    let dec_bytes = unsafe {
        LZ4_decompress_safe(
            src.as_ptr() as *const c_char,
            dst.as_mut_ptr() as *mut c_char,
            src.len() as i32,
            size,
        )
    };

    if dec_bytes < 0 {
        return Err(eio!("decompression failed"));
    }
    match dcom_now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            unsafe {
                decom_timestamp += elapsed.as_nanos();
                dec_in_size += src.len() as u128;
                info!(
                    "lz4_decompress decom_timestamp= {}, in_size= {}, out_size= {}",
                    decom_timestamp, dec_in_size, dec_bytes
                );
            }
        }
        Err(_e) => {
            // an error occurred!
            return Err(einval!("timestamp error"));
        }
    };

    Ok(dec_bytes as usize)
}

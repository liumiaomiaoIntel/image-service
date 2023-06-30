use crate::{
    qzClose as qzClose_r, qzCompress as qzCompress_r, qzDecompress as qzDecompress_r,
    qzInit as qzInit_r, qzMaxCompressedLength as qzMaxCompressedLength_r,
    qzSetDefaultsLZ4 as qzSetDefaultsLZ4_r, qzSetupSessionLZ4 as qzSetupSessionLZ4_r,
    qzTeardownSession as qzTeardownSession_r, QzSessionParamsCommon_T as QzSessionParamsCommon_St,
    QzSessionParamsLZ4_T as QzSessionParamsLZ4_St, QzSession_T as QzSession_St,
};
use libc::c_uchar;
use libc::c_uint;
use nydus_error::einval;
use nydus_error::eio;
use std::io::Result;
use std::mem;
use std::time::SystemTime;

static mut com_timestamp: u128 = 0;
static mut decom_timestamp: u128 = 0;
static mut com_size: u128 = 0;
static mut decom_size: u128 = 0;
static mut dec_in_size: u128 = 0;

pub(super) fn lz4_qatzip_compress(src: &[u8]) -> Result<Vec<u8>> {
    unsafe {
        let mut session: QzSession_St = mem::zeroed();

        let result1 = qzInit_r(&mut session as *mut QzSession_St, 1);
        assert!(
            result1 == 0 || result1 == 1,
            "qzInit failed when compressing"
        );

        let mut commom_params: QzSessionParamsCommon_St = mem::zeroed();
        commom_params.direction = 2; //QZ_DIR_BOTH
        commom_params.comp_lvl = 1;
        commom_params.comp_algorithm = b'4'; //'4' QZ_LZ4
        commom_params.max_forks = 3; //QZ_MAX_FORK_DEFAULT
        commom_params.sw_backup = 1;
        commom_params.hw_buff_sz = 64 * 1024; //QZ_HW_BUFF_SZ
        commom_params.strm_buff_sz = 64 * 1024; //QZ_HW_BUFF_SZ
        commom_params.input_sz_thrshold = 1024; //QZ_COMP_THRESHOLD_DEFAULT
        commom_params.req_cnt_thrshold = 32; //NUM_BUFF
        commom_params.wait_cnt_thrshold = 8;
        commom_params.polling_mode = 0; //QZ_PERIODICAL_POLLING

        let mut params_lz4: QzSessionParamsLZ4_St = mem::zeroed();
        params_lz4.common_params = commom_params;
        let result2 = qzSetDefaultsLZ4_r(&mut params_lz4 as *mut QzSessionParamsLZ4_St);
        assert_eq!(result2, 0, "qzSetDefaultsLZ4_r failed when compressing");

        let result3 = qzSetupSessionLZ4_r(
            &mut session as *mut QzSession_St,
            &mut params_lz4 as *mut QzSessionParamsLZ4_St,
        );
        assert_eq!(result3, 0, "qzSetupSessionLZ4 failed when compressing");

        let mut srclen = src.len() as c_uint;

        let mut dest_sz =
            qzMaxCompressedLength_r(srclen as c_uint, &mut session as *mut QzSession_St);
        assert_ne!(dest_sz, 0, "compression integer overflow happens");
        assert_ne!(dest_sz, 34, "compression input length is 0");
        let mut dest_buf = Vec::with_capacity(dest_sz as usize);

        let com_now = SystemTime::now();
        let result4 = qzCompress_r(
            &mut session as *mut QzSession_St,
            src.as_ptr() as *const c_uchar,
            &mut srclen as *mut c_uint,
            dest_buf.as_mut_ptr() as *mut c_uchar,
            &mut dest_sz as *mut c_uint,
            1,
        );

        if result4 != 0 {
            return Err(eio!("compression failed"));
        }
        match com_now.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                com_timestamp += elapsed.as_nanos();
                com_size += srclen as u128;
                decom_size += dest_sz as u128;
                info!(
                    "lz4qatzip_compress com_timestamp= {}, com_size= {}, decom_size= {}",
                    com_timestamp, com_size, decom_size
                );
            }
            Err(_e) => {
                // an error occurred!
                return Err(einval!("timestamp error"));
            }
        };

        let result5 = qzTeardownSession_r(&mut session as *mut QzSession_St);
        assert_eq!(result5, 0, "qzTeardownSession failed when compressing");
        let result6 = qzClose_r(&mut session as *mut QzSession_St);
        assert_eq!(result6, 0, "qzClose failed when compressing");

        assert!(dest_sz as usize <= dest_buf.capacity());
        dest_buf.set_len(dest_sz as usize);

        Ok(dest_buf)
    }
}

pub(super) fn lz4_qatzip_decompress(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    if dst.len() >= std::i32::MAX as usize {
        return Err(einval!("the destination buffer is big than i32::MAX"));
    }

    unsafe {
        let mut session: QzSession_St = mem::zeroed();

        let result_1 = qzInit_r(&mut session as *mut QzSession_St, 1);
        assert!(
            result_1 == 0 || result_1 == 1,
            "qzInit failed when decompressing"
        );

        let mut commom_params: QzSessionParamsCommon_St = mem::zeroed();
        commom_params.direction = 2; //QZ_DIR_BOTH
        commom_params.comp_lvl = 1;
        commom_params.comp_algorithm = b'4'; //'4' QZ_LZ4
        commom_params.max_forks = 3; //QZ_MAX_FORK_DEFAULT
        commom_params.sw_backup = 1;
        commom_params.hw_buff_sz = 64 * 1024; //QZ_HW_BUFF_SZ
        commom_params.strm_buff_sz = 64 * 1024; //QZ_HW_BUFF_SZ
        commom_params.input_sz_thrshold = 1024; //QZ_COMP_THRESHOLD_DEFAULT
        commom_params.req_cnt_thrshold = 32; //NUM_BUFF
        commom_params.wait_cnt_thrshold = 8;
        commom_params.polling_mode = 0; //QZ_PERIODICAL_POLLING

        let mut params_lz4: QzSessionParamsLZ4_St = mem::zeroed();
        params_lz4.common_params = commom_params;
        let result_2 = qzSetDefaultsLZ4_r(&mut params_lz4 as *mut QzSessionParamsLZ4_St);
        assert_eq!(result_2, 0, "qzSetDefaultsLZ4_r failed when decompressing");

        let result_3 = qzSetupSessionLZ4_r(
            &mut session as *mut QzSession_St,
            &mut params_lz4 as *mut QzSessionParamsLZ4_St,
        );
        assert_eq!(result_3, 0, "qzSetupSessionLZ4 failed when decompressing");

        let mut srclen = src.len() as c_uint;
        let mut destlen = dst.len() as c_uint;

        let dcom_now = SystemTime::now();
        let result_4 = qzDecompress_r(
            &mut session as *mut QzSession_St,
            src.as_ptr() as *const c_uchar,
            &mut srclen as *mut c_uint,
            dst.as_mut_ptr() as *mut c_uchar,
            &mut destlen as *mut c_uint,
        );
        if result_4 != 0 {
            return Err(eio!("decompression failed"));
        }
        match dcom_now.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                decom_timestamp += elapsed.as_nanos();
                dec_in_size += srclen as u128;
                info!(
                    "lz4qatip_decompress decom_timestamp= {}, in_size= {}, out_size= {}",
                    decom_timestamp, dec_in_size, destlen
                );
            }
            Err(_e) => {
                // an error occurred!
                return Err(einval!("timestamp error"));
            }
        };

        let result_5 = qzTeardownSession_r(&mut session as *mut QzSession_St);
        assert_eq!(result_5, 0, "qzTeardownSession failed when decompressing");
        let result_6 = qzClose_r(&mut session as *mut QzSession_St);
        assert_eq!(result_6, 0, "qzClose failed when decompressing");

        Ok(destlen as usize)
    }
}

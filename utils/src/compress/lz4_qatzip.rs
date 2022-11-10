use crate::{QzSession_T as QzSession_St, QzSessionParams_T as QzSessionParams_St, qzInit as qzInit_r, qzSetupSession as qzSetupSession_r, qzMaxCompressedLength as qzMaxCompressedLength_r, qzCompress as qzCompress_r, qzDecompress as qzDecompress_r, qzTeardownSession as qzTeardownSession_r, qzClose as qzClose_r};
use std::io::Result;
use libc::c_uchar;
use libc::c_uint;
use std::mem;
use nydus_error::einval;
use nydus_error::eio;

pub(super) fn lz4_qatzip_compress(src: &[u8]) -> Result<Vec<u8>> {
    unsafe{
        let mut session: QzSession_St = mem::zeroed();

        let result = qzInit_r(&mut session as *mut QzSession_St, 1);
        assert!(result == 0 || result == 1, "qzInit failed when compressing");
       
        let mut params: QzSessionParams_St = mem::zeroed();
        params.direction = 2;//QZ_DIR_BOTH
        params.data_fmt = 4;//4 for QZ_LZ4_FH
        params.comp_lvl =1 ;
        params.comp_algorithm = '4' as u8;//'4' QZ_LZ4
        params.max_forks = 3;//QZ_MAX_FORK_DEFAULT
        params.sw_backup = 1;
        params.hw_buff_sz = 64*1024;//QZ_HW_BUFF_SZ
        params.strm_buff_sz = 64*1024;//QZ_HW_BUFF_SZ
        params.input_sz_thrshold = 1024;//QZ_COMP_THRESHOLD_DEFAULT
        params.req_cnt_thrshold = 32;//NUM_BUFF
        params.wait_cnt_thrshold = 8;
        params.is_busy_polling = 0;//QZ_PERIODICAL_POLLING

        let result = qzSetupSession_r( &mut session as *mut QzSession_St, &mut params as *mut QzSessionParams_St);
        assert_eq!(result, 0, "qzSetupSession failed when compressing");

        let mut srclen = src.len() as c_uint;

        let mut dest_sz = qzMaxCompressedLength_r(srclen as c_uint, &mut session as *mut QzSession_St);
        assert_ne!(dest_sz, 0, "compression integer overflow happens");
        assert_ne!(dest_sz, 34, "compression input length is 0");
        let mut dest_buf = Vec::with_capacity(dest_sz as usize);

        let result = qzCompress_r(&mut session as *mut QzSession_St, 
            src.as_ptr() as *const c_uchar, &mut srclen as *mut c_uint, 
           dest_buf.as_mut_ptr() as *mut c_uchar, &mut dest_sz as *mut c_uint, 1);
        
        if result != 0 {
            return Err(eio!("compression failed"));
        }

        let result = qzTeardownSession_r(&mut session as *mut QzSession_St); 
        assert_eq!(result, 0, "qzTeardownSession failed when compressing");
        let result = qzClose_r(&mut session as *mut QzSession_St);
        assert_eq!(result, 0, "qzClose failed when compressing");

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

        let result = qzInit_r(&mut session as *mut QzSession_St, 1);
        assert!(result == 0 || result == 1, "qzInit failed when decompressing");
                
        let mut params: QzSessionParams_St = mem::zeroed();
        params.direction = 2;//QZ_DIR_BOTH
        params.data_fmt = 4;//4 for QZ_LZ4_FH
        params.comp_lvl =1 ;
        params.comp_algorithm = '4' as u8;//QZ_LZ4
        params.max_forks = 3;//QZ_MAX_FORK_DEFAULT
        params.sw_backup = 1;
        params.hw_buff_sz = 64*1024;//QZ_HW_BUFF_SZ
        params.strm_buff_sz = 64*1024;//QZ_HW_BUFF_SZ
        params.input_sz_thrshold = 1024;//QZ_COMP_THRESHOLD_DEFAULT
        params.req_cnt_thrshold = 32;//NUM_BUFF
        params.wait_cnt_thrshold = 8;
        params.is_busy_polling = 0;//QZ_PERIODICAL_POLLING
        
        let result = qzSetupSession_r( &mut session as *mut QzSession_St, &mut params as *mut QzSessionParams_St);
        assert_eq!(result, 0, "qzSetupSession failed when decompressing");
        
        let mut srclen = src.len() as c_uint;
        let mut destlen = dst.len() as c_uint;
        
        let result_de = qzDecompress_r(&mut session as *mut QzSession_St, 
            src.as_ptr() as *const c_uchar, &mut srclen as *mut c_uint, 
            dst.as_mut_ptr() as *mut c_uchar, &mut destlen as *mut c_uint);
        assert_eq!(result, 0, "qzDecompress failed when decompressing");
        
        let result = qzTeardownSession_r(&mut session as *mut QzSession_St); 
        assert_eq!(result, 0, "qzTeardownSession failed when decompressing");
        let result = qzClose_r(&mut session as *mut QzSession_St);
        assert_eq!(result, 0, "qzClose failed when decompressing");
        
        if result_de != 0 {
            return Err(eio!("decompression failed"));
        }
    
        Ok(destlen as usize)
    }    
}

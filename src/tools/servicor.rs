use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// 包含C头文件
#[allow(dead_code)]
#[link(name = "servicor")]
extern "C" {
    fn Search(query: *const c_char) -> *mut c_char;
    fn Visit(url: *const c_char) -> *mut c_char;
    fn DownloadNovel(url: *const c_char) -> *mut c_char;
    fn FreeString(s: *mut c_char);
}

pub fn search(query: &str) -> Result<String, String> {
    let c_query = CString::new(query).map_err(|e| e.to_string())?;
    let result_ptr = unsafe { Search(c_query.as_ptr()) };
    if result_ptr.is_null() {
        return Err("Search returned null pointer".to_string());
    }
    let result_str = unsafe { CStr::from_ptr(result_ptr).to_string_lossy().into_owned() };
    unsafe { FreeString(result_ptr) };
    if result_str.starts_with("Error:") {
        Err(result_str)
    } else {
        Ok(result_str)
    }
}

pub fn visit(url: &str) -> Result<String, String> {
    let c_url = CString::new(url).map_err(|e| e.to_string())?;
    let result_ptr = unsafe { Visit(c_url.as_ptr()) };
    if result_ptr.is_null() {
        return Err("Visit returned null pointer".to_string());
    }
    let result_str = unsafe { CStr::from_ptr(result_ptr).to_string_lossy().into_owned() };
    unsafe { FreeString(result_ptr) };
    if result_str.starts_with("Error:") {
        Err(result_str)
    } else {
        Ok(result_str)
    }
}

pub fn download_novel(url: &str) -> Result<String, String> {
    let c_url = CString::new(url).map_err(|e| e.to_string())?;
    let result_ptr = unsafe { DownloadNovel(c_url.as_ptr()) };
    if result_ptr.is_null() {
        return Err("DownloadNovel returned null pointer".to_string());
    }
    let result_str = unsafe { CStr::from_ptr(result_ptr).to_string_lossy().into_owned() };
    unsafe { FreeString(result_ptr) };
    if result_str.starts_with("Error:") {
        Err(result_str)
    } else {
        Ok(result_str)
    }
}

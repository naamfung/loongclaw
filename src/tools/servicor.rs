use std::ffi::CString;
use std::os::raw::c_char;

// 声明从Go静态库导入的函数
#[link(name = "servico")]
extern "C" {
    fn Search(keyword: *const c_char);
    fn Visit(url: *const c_char);
    fn Download(novelURL: *const c_char);
}

pub fn search(query: &str) {
    let c_query = CString::new(query).expect("CString::new failed");
    unsafe {
        Search(c_query.as_ptr());
    }
}

pub fn visit(url: &str) {
    let c_url = CString::new(url).expect("CString::new failed");
    unsafe {
        Visit(c_url.as_ptr());
    }
}

pub fn download(novel_url: &str) {
    let c_novel_url = CString::new(novel_url).expect("CString::new failed");
    unsafe {
        Download(c_novel_url.as_ptr());
    }
}

use crate::clib;
use crate::question::{alpm_question_t, QuestionArgs};
use std::os::raw::c_char;

use super::{alpm_handle_t, Handle};

#[link(name = "alpm")]
extern "C" {
    fn alpm_option_set_logcb(
        handle: *mut alpm_handle_t,
        f: extern "C" fn(i32, *mut c_char, clib::VaList),
    ) -> i32;
    fn alpm_option_set_dlcb(
        handle: *mut alpm_handle_t,
        f: extern "C" fn(filename: *mut c_char, xfered: i64, total: i64),
    ) -> i32;
    fn alpm_option_set_questioncb(
        handle: *mut alpm_handle_t,
        f: extern "C" fn(question: *mut alpm_question_t),
    ) -> i32;
}

/*
type LogCallback = Option<Fn(i32,String)>;
type DownloadCallback<T: Fn(&str,i64,i64)> = T;
type QuestionCallback<T: Fn(QuestionArgs)> = T;


static mut LOG_CALLBACK : Option<Box<LogCallback>> = None;
static mut DOWNLOAD_CALLBACK : Option<DownloadCallback> = None;
static mut QUESTION_CALLBACK : Option<QuestionCallback> = None;
*/

type LogCallback = dyn Fn(i32, String);
type DownloadCallback = dyn Fn(&str, i64, i64);
type QuestionCallback = dyn Fn(QuestionArgs);

static mut LOG_CALLBACK: Option<Box<LogCallback>> = None;
static mut DOWNLOAD_CALLBACK: Option<Box<DownloadCallback>> = None;
static mut QUESTION_CALLBACK: Option<Box<QuestionCallback>> = None;

extern "C" fn alpm_log_cb_handler(level: i32, fmt: *mut c_char, args: clib::VaList) {
    unsafe {
        if let Some(f) = &LOG_CALLBACK {
            let out = clib::vsn_printf(fmt, args);
            f(level, out);
        }
    }
}

pub fn set_log_callback<T: Fn(i32, String) + 'static>(h: &Handle, cb: T) {
    unsafe {
        match LOG_CALLBACK {
            None => {
                alpm_option_set_logcb(h.alpm_handle, alpm_log_cb_handler);
            }
            _ => {}
        }
        LOG_CALLBACK = Some(Box::new(cb));
    }
}

extern "C" fn alpm_log_download_handler(filename: *mut c_char, xfered: i64, total: i64) {
    unsafe {
        if let Some(f) = &DOWNLOAD_CALLBACK {
            f(cstr!(filename), xfered, total);
        }
    }
}

pub fn set_download_callback<T: Fn(&str, i64, i64) + 'static>(h: &Handle, cb: T) {
    unsafe {
        match DOWNLOAD_CALLBACK {
            None => {
                alpm_option_set_dlcb(h.alpm_handle, alpm_log_download_handler);
            }
            _ => {}
        }
        DOWNLOAD_CALLBACK = Some(Box::new(cb));
    }
}

extern "C" fn alpm_question_handler(q_raw: *mut alpm_question_t) {
    unsafe {
        if let Some(f) = &QUESTION_CALLBACK {
            f(q_raw.into());
        }
    }
}

pub fn set_question_callback<T: Fn(QuestionArgs) + 'static>(h: &Handle, cb: T) {
    unsafe {
        match QUESTION_CALLBACK {
            None => {
                alpm_option_set_questioncb(h.alpm_handle, alpm_question_handler);
            }
            _ => {}
        }
        QUESTION_CALLBACK = Some(Box::new(cb));
    }
}

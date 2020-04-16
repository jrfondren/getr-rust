use libc::{c_int, c_long};
use std::mem::MaybeUninit;

#[repr(C)]
enum Who {
    RusageChildren = -1,
}

#[allow(dead_code)]
#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[allow(dead_code)]
#[repr(C)]
pub struct RUsage {
    pub ru_utime: Timeval,
    pub ru_stime: Timeval,
    pub ru_maxrss: c_long,
    pub ru_ixrss: c_long,
    pub ru_idrss: c_long,
    pub ru_isrss: c_long,
    pub ru_minflt: c_long,
    pub ru_majflt: c_long,
    pub ru_nswap: c_long,
    pub ru_inblock: c_long,
    pub ru_oublock: c_long,
    pub ru_msgsnd: c_long,
    pub ru_msgrcv: c_long,
    pub ru_nsignals: c_long,
    pub ru_nvcsw: c_long,
    pub ru_nivcsw: c_long,
}

extern "C" {
    fn getrusage(who: Who, usage: *mut RUsage) -> c_int;
}

impl RUsage {
    pub fn new() -> Self {
        unsafe {
            let mut usage: RUsage = MaybeUninit::uninit().assume_init();
            getrusage(Who::RusageChildren, &mut usage);
            usage
        }
    }

    pub fn report(&self, times: i32) -> String {
        let seconds = self.ru_utime.tv_sec + self.ru_stime.tv_sec;
        let microseconds = self.ru_utime.tv_usec + self.ru_stime.tv_usec;
        let milliseconds = seconds * 1000 + microseconds / 1000;
        let ms_per = milliseconds  as f64 / times as f64;
        format!(r#"User time      : {} s, {} us
System time    : {} s, {} us
Time           : {} ms ({:.3} ms/per)
Max RSS        : {}
Page reclaims  : {}
Page faults    : {}
Block inputs   : {}
Block outputs  : {}
vol ctx switches   : {}
invol ctx switches : {}"#,
            self.ru_utime.tv_sec, self.ru_utime.tv_usec,
            self.ru_stime.tv_sec, self.ru_stime.tv_usec,
            milliseconds, ms_per,
            readable_rss(self.ru_maxrss as f64), self.ru_minflt, self.ru_majflt,
            self.ru_inblock, self.ru_oublock, self.ru_nvcsw, self.ru_nivcsw)
    }
}

fn readable_rss(kb: f64) -> String {
    if kb < 1024.0 {
        format!("{0:.0} kB", kb)
    } else if kb < 1024.0 * 1024.0 {
        format!("{0:.1} MB", kb / 1024.0)
    } else {
        format!("{0:.2} GB", (kb / (1024.0 * 1024.0)))
    }
}

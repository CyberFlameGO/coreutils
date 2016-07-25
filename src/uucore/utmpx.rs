#![allow(dead_code, non_camel_case_types)]

extern crate libc;

pub use self::utmpx::{UT_NAMESIZE,UT_LINESIZE,DEFAULT_FILE,USER_PROCESS,BOOT_TIME,c_utmp};
#[cfg(target_os = "linux")]
mod utmpx {
    use super::libc;

    pub static DEFAULT_FILE: &'static str = "/var/run/utmp";

    pub const UT_LINESIZE: usize = 32;
    pub const UT_NAMESIZE: usize = 32;
    pub const UT_IDSIZE: usize = 4;
    pub const UT_HOSTSIZE: usize = 256;

    pub const EMPTY: libc::c_short = 0;
    pub const RUN_LVL: libc::c_short = 1;
    pub const BOOT_TIME: libc::c_short = 2;
    pub const NEW_TIME: libc::c_short = 3;
    pub const OLD_TIME: libc::c_short = 4;
    pub const INIT_PROCESS: libc::c_short = 5;
    pub const LOGIN_PROCESS: libc::c_short = 6;
    pub const USER_PROCESS: libc::c_short = 7;
    pub const DEAD_PROCESS: libc::c_short = 8;
    pub const ACCOUNTING: libc::c_short = 9;

    #[repr(C)]
    pub struct timeval {
        pub tv_sec: libc::int32_t,
        pub tv_usec: libc::int32_t,
    }

    #[repr(C)]
    pub struct c_exit_status {
        pub e_termination: libc::c_short,
        pub e_exit: libc::c_short,
    }

    #[repr(C)]
    pub struct c_utmp {
        pub ut_type: libc::c_short,
        pub ut_pid: libc::pid_t,
        pub ut_line: [libc::c_char; UT_LINESIZE],
        pub ut_id: [libc::c_char; UT_IDSIZE],

        pub ut_user: [libc::c_char; UT_NAMESIZE],
        pub ut_host: [libc::c_char; UT_HOSTSIZE],
        pub ut_exit: c_exit_status,
        pub ut_session: libc::int32_t,
        pub ut_tv: timeval,

        pub ut_addr_v6: [libc::int32_t; 4],
        pub __unused: [libc::c_char; 20],
    }
}

#[cfg(target_os = "macos")]
mod utmpx {
    use super::libc;

    pub static DEFAULT_FILE: &'static str = "/var/run/utmpx";

    pub const UT_LINESIZE: usize = 32;
    pub const UT_NAMESIZE: usize = 256;
    pub const UT_IDSIZE: usize = 4;
    pub const UT_HOSTSIZE: usize = 256;

    pub const EMPTY: libc::c_short = 0;
    pub const RUN_LVL: libc::c_short = 1;
    pub const BOOT_TIME: libc::c_short = 2;
    pub const OLD_TIME: libc::c_short = 3;
    pub const NEW_TIME: libc::c_short = 4;
    pub const INIT_PROCESS: libc::c_short = 5;
    pub const LOGIN_PROCESS: libc::c_short = 6;
    pub const USER_PROCESS: libc::c_short = 7;
    pub const DEAD_PROCESS: libc::c_short = 8;
    pub const ACCOUNTING: libc::c_short = 9;

    #[repr(C)]
    pub struct c_exit_status {
        pub e_termination: libc::c_short,
        pub e_exit: libc::c_short,
    }

    #[repr(C)]
    pub struct c_utmp {
        pub ut_user: [libc::c_char; UT_NAMESIZE],
        pub ut_id: [libc::c_char; UT_IDSIZE],
        pub ut_line: [libc::c_char; UT_LINESIZE],
        pub ut_pid: libc::pid_t,
        pub ut_type: libc::c_short,
        pub ut_tv: libc::timeval,
        pub ut_host: [libc::c_char; UT_HOSTSIZE],
        pub __unused: [libc::c_char; 16]
    }
}

#[cfg(target_os = "freebsd")]
mod utmpx {
    use super::libc;

    pub static DEFAULT_FILE : &'static str = "";

    pub const UT_LINESIZE : usize = 16;
    pub const UT_NAMESIZE : usize = 32;
    pub const UT_IDSIZE   : usize = 8;
    pub const UT_HOSTSIZE : usize = 128;

    pub const EMPTY         : libc::c_short = 0;
    pub const BOOT_TIME     : libc::c_short = 1;
    pub const OLD_TIME      : libc::c_short = 2;
    pub const NEW_TIME      : libc::c_short = 3;
    pub const USER_PROCESS  : libc::c_short = 4;
    pub const INIT_PROCESS  : libc::c_short = 5;
    pub const LOGIN_PROCESS : libc::c_short = 6;
    pub const DEAD_PROCESS  : libc::c_short = 7;
    pub const SHUTDOWN_TIME : libc::c_short = 8;

    #[repr(C)]
    pub struct c_utmp {
        pub ut_type : libc::c_short,
        pub ut_tv   : libc::timeval,
        pub ut_id   : [libc::c_char; UT_IDSIZE],
        pub ut_pid  : libc::pid_t,
        pub ut_user : [libc::c_char; UT_NAMESIZE],
        pub ut_line : [libc::c_char; UT_LINESIZE],
        pub ut_host : [libc::c_char; UT_HOSTSIZE],
        pub ut_spare : [libc::c_char; 64],
    }
}

use std::ffi::{CStr, CString};
use std::io;
use std::io::prelude::*;
use std::ptr;

use csv;
use libc;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use zip;

#[derive(Serialize, Deserialize)]
pub struct Tick {
    pub time: i64,
    pub trade_sale: i64,
    pub trade_volume: i64,
    pub exchange: char,
    pub sale_condition: Option<char>,
    pub suspicious: i64
}

#[repr(C)]
pub struct TickResult {
    pub size: i64,
    pub buf: *mut i64
}

#[no_mangle]
pub extern fn ParseEquityTickExtern(path: *const libc::c_char, dataTimeZone: i64, exchange_time_zone: i64) -> TickResult {
    let data_path = unsafe { CStr::from_ptr(path) }.to_str();

    if data_path.is_err() {
        println!("Failed to read string");
        return TickResult {
            size: 0,
            buf: [].as_mut_ptr()
        }
    }

    let file = std::fs::File::open(&data_path.unwrap());

    if file.is_err() {
        println!("Failed to open file {}", data_path.unwrap());
        return TickResult {
            size: 0,
            buf: [].as_mut_ptr()
        }
    }

    let archive = zip::ZipArchive::new(file.unwrap());

    if archive.is_err() {
        println!("Failed to parse zip file {}", data_path.unwrap());
        return TickResult {
            size: 0,
            buf: [].as_mut_ptr()
        }
    }

    let bytes = archive.unwrap()
        .by_index(0)
        .expect("Failed to find file inside zip")
        .bytes()
        .map(|i| i.expect("Failed to get thing"))
        .collect::<Vec<u8>>();

    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&bytes[..]);

    let iter = csv.deserialize::<Tick>().collect::<Vec<Result<Tick, _>>>();//.map(|i| i.unwrap()).collect();
    //let mut data = Vec::with_capacity(iter.len());

    let mut data: Vec<i64> = iter.par_iter().map(|i| i.unwrap()).map(|tick| {
        let formatted = vec![
            tick.time, 
            tick.trade_sale, 
            tick.trade_volume, 
            (tick.exchange as u32 - '0' as u32) as i64,
            (tick.sale_condition.unwrap() as u32 - '0' as u32) as i64,
            tick.suspicious
        ];

        formatted })
        .flatten()
        .collect();

    let len = data.len();
    let ptr = data.as_mut_ptr();

    // Have to deallocate after the fact
    std::mem::forget(data);

    TickResult {
        size: len as i64,
        buf: ptr
    }
}

#[no_mangle]
pub extern fn DisposeExtern(result: TickResult) {
    unsafe { drop(Vec::from_raw_parts(result.buf, result.size as usize, result.size as usize)); }
}
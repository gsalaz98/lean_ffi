use std::ffi::CStr;
use std::io::prelude::*;

use csv;
use lexical;
use libc;
use serde::{Serialize, Deserialize};
use zip;

/*
#[no_mangle]
pub extern fn ParseEquityTickExtern(data: *mut i32, buf_size: i64, path: *const libc::c_char) -> i64 {
    let data_path = unsafe { CStr::from_ptr(path) }.to_str();

    if data_path.is_err() {
        println!("Failed to read string");
        return 0
    }

    let file = std::fs::File::open(&data_path.unwrap());

    if file.is_err() {
        println!("Failed to open file {}", data_path.unwrap());
        return 0
    }

    let mut archive = zip::ZipArchive::new(file.unwrap()).expect("Archive");
    let file_zip = archive.by_index(0).expect("Missing file");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file_zip);

    let mut record = csv::ByteRecord::with_capacity(60, 6);
    let mut i: isize = 0;

    unsafe { 
        while reader.read_byte_record(&mut record).expect("Read byte record") {
            let tick = record.deserialize::<Tick>(None).expect("Deserialize");

            let time = data.offset(i);
            *time = tick.time;

            let trade_sale = data.offset(i + 1);
            *trade_sale = tick.trade_sale;

            let trade_volume = data.offset(i + 2);
            *trade_volume = tick.trade_volume;
            
            let exchange = data.offset(i + 3);
            *exchange = tick.exchange as i32 - '0' as i32;
            
            let sale_condition = data.offset(i + 4);
            *sale_condition = tick.sale_condition.map_or(0, |c| c as i32 - '0' as i32);
            
            let suspicious = data.offset(i + 5);
            *suspicious = tick.suspicious;

            i += 6;
        }
    }

    return i as i64;
}
*/

#[no_mangle]
pub extern fn ParseTickCsvLine(buf: *mut i32, line: *const libc::c_char) {
    let csv = unsafe { CStr::from_ptr(line) }.to_str().unwrap();

    unsafe {
        for (i, data) in csv.split(',').enumerate() {
            let offset = buf.offset(i as isize);

            if i >= 3 && i < 5 {
                *offset = data.chars().next().unwrap_or('0') as i32 - '0' as i32;
                continue;
            }

            *offset = lexical::parse::<i32, _>(data).unwrap();
        }
    }
}
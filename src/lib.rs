use std::panic;

// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate multiaddr;

use multiaddr::{Protocol, Multiaddr};

mod unsigned_varint;
mod maddr_parse;
// use multiformats::Protocol;
// use multiformats::convert_to_string;
//     convert_to_string, convert_to_bytes, get_protocol, Protocol, StringTuple, Tuple,
// };

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    // console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

/**
 * This method will receive Uint8Array from JavaScript and convert it to a Multiaddr string
 * @param input Uint8Array
 * @returns string
 */
#[wasm_bindgen]
// pub fn bytes_to_string(serialized: &Vec<u8>, length: u32) -> Result<JsValue, JsValue> {
pub fn bytes_to_string(serialized: &js_sys::Uint8Array, length: usize) -> Result<JsValue, JsValue> {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  console::log_1(&JsValue::from_str("Executing Rust function bytes_to_string"));
  let uint8array = js_sys::Uint8Array::new(serialized);
  // let size = uint8array.length().try_into().unwrap();
  let mut slice = vec![0; length];
  uint8array.copy_to(&mut slice[..]);

  // #[cfg(debug_assertions)]
  // console_error_panic_hook::set_once();
  // output the raw bytes to the console
  // convert the Uint8Array into a &Vec<[u8]>

  // console::log_1(&JsValue::from_str("Converting Uint8Array to Vec<u8>"));
  // let serialized_vec = serialized.to_vec();
  // let serialized_string = from_utf8(serialized_vec).unwrap();
  // console::log_1(&JsValue::from_str("Converted Uint8Array to Vec<u8>"));
  // console::log_1( &JsValue::from_str(serialized_string));
  // convert Uint8Array into &[u8]
  // // let serialized: &[u8] = &serialized.to_vec();
  // let deserialized: Multiaddr = bincode::deserialize(&serialized).unwrap();
  // use bincode to deserialize the js_sys::Uint8Array into a Multiaddr
  // let thingy = serialized
  // convert serialized_vec into a Reader that deserialize_from will accept
  console::log_1(&JsValue::from_str("Getting a reader"));
  // let mut cursor = &serialized[..];
  console::log_1(&JsValue::from_str("Got a reader"));
  console::log_1(&JsValue::from_str("Trying to deserialize bytes into a Multiaddr"));
  // let deserialized: Multiaddr = bincode::deserialize(&mut slice[..]).unwrap();
  let cursor = std::io::Cursor::new(&mut slice[..]);
  let deserialized: Multiaddr = bincode::deserialize_from(cursor).unwrap();
  // let deserialized: Multiaddr = bincode::deserialize(&serialized.to_vec()).unwrap();

  console::log_1(&JsValue::from_str("Deserialized bytes into a Multiaddr"));

  let multiaddr_string = deserialized.to_string();

  console::log_1(&JsValue::from_str(&multiaddr_string));
  // convert
  Ok(JsValue::from_str(&multiaddr_string))
}


// /**
//  * This method will receive Uint8Array from JavaScript and convert it to a Multiaddr string
//  * @param input Uint8Array
//  * @returns string
//  */
// #[wasm_bindgen]
// pub fn bytes_to_string(serialized: &[u8], _length: Option<usize>) -> Result<JsValue, JsValue> {
//     panic::set_hook(Box::new(console_error_panic_hook::hook));
//     console::log_1(&JsValue::from_str("Executing Rust function bytes_to_string"));

//     // let uint8array = Uint8Array::new(serialized);
//     // let mut slice = vec![0; length];
//     // uint8array.copy_to(&mut slice[..]);

//     // console::log_1(&JsValue::from_str("Getting a reader"));
//     // let cursor = std::io::Cursor::new(&mut slice[..]);
//     // console::log_1(&JsValue::from_str("Got a reader"));

//     let converted = unsigned_varint::decode_uint8_array(&serialized, 0);

//     console::log_1(&JsValue::from_str("Trying to deserialize bytes into a Multiaddr"));
//     // let result: Multiaddr = Multiaddr::from(converted);
//     // let deserialized: Result<Multiaddr, bincode::Error> = bincode::deserialize_from(converted);
//     let result = maddr_parse::bytes_to_multiaddr_parts(&serialized);
//     // let maResult = Multiaddr::from(result);
//     let protocol_bytes = Protocol::from_bytes(&serialized);
//     match result {
//         Ok(multiaddr_parts) => {
//             console::log_1(&JsValue::from_str("Deserialized bytes into a Multiaddr"));
//             let multiaddr_string = multiaddr_parts.string;
//             console::log_1(&JsValue::from_str(&multiaddr_string));
//             Ok(JsValue::from_str(&multiaddr_string))
//         },
//         Err(err) => {
//             console::log_1(&JsValue::from_str(&format!("Deserialization error: {:?}", err)));
//             // Handle the error gracefully or return an error to JavaScript.
//             Err(JsValue::from_str("Deserialization error"))
//         }
//     }

//     // match deserialized {
//     //     Ok(deserialized) => {
//     //         console::log_1(&JsValue::from_str("Deserialized bytes into a Multiaddr"));
//     //         let multiaddr_string = deserialized.to_string();
//     //         console::log_1(&JsValue::from_str(&multiaddr_string));
//     //         Ok(JsValue::from_str(&multiaddr_string))
//     //     },
//     //     Err(err) => {
//     //         console::log_1(&JsValue::from_str(&format!("Deserialization error: {:?}", err)));
//     //         // Handle the error gracefully or return an error to JavaScript.
//     //         Err(JsValue::from_str("Deserialization error"))
//     //     }
//     // }
// }





// #[derive(Debug)]
// pub struct MultiaddrParts {
//     // pub bytes: Vec<u8>,
//     pub string: String,
//     // pub tuples: Vec<Tuple>,
//     // pub string_tuples: Vec<StringTuple>,
//     // pub path: Option<String>,
// }


// pub fn bytes_to_multiaddr_string(bytes: &[u8]) -> Result<MultiaddrParts, String> {
//     // use unsigned_varint::decode;
//     let mut tuples = Vec::new();
//     let mut string_tuples = Vec::new();
//     let mut path: Option<String> = None;

//     let mut i = 0;
//     while i < bytes.len() {
//         let code = unsigned_varint::decode_uint8_array(&bytes[..], i);
//         let n = unsigned_varint::encoding_length(code);
//         let p = maddr_parse::get_protocol(code);
//         let size = maddr_parse::size_for_addr(&p, &bytes[i + n..]);

//         if size == 0 {
//             tuples.push(vec![code]);
//             string_tuples.push(vec![code]);
//             i += n;
//             continue;
//         }

//         let addr = &bytes[i + n..i + n + size];
//         i += size + n;

//         if i > bytes.len() {
//             return Err("Invalid address Uint8Array".to_string());
//         }

//         tuples.push(vec![code, addr.to_vec()]);
//         let string_addr = convert_to_string(code, addr).to_string();
//         string_tuples.push(vec![code, string_addr.clone()]);
//         if p.path {
//             path = Some(string_addr);
//             break;
//         }
//     }

//     Ok(MultiaddrParts {
//         // bytes: bytes.to_vec(),
//         // string: string_tuples_to_string(&string_tuples),
//         // tuples,
//         // string_tuples,
//         // path,
//     })
// }

// // fn size_for_addr(p: &Protocol, addr: &[u8]) -> usize {
// //     if p.size > 0 {
// //         (p.size / 8) as usize
// //     } else if p.size == 0 {
// //         0
// //     } else {
// //         let size = decode(addr).unwrap();
// //         size + size.varint_encoded_length()
// //     }
// // }

// fn string_tuples_to_string(tuples: &Vec<StringTuple>) -> String {
//     let parts: Vec<String> = tuples
//         .iter()
//         .flat_map(|tup| {
//             let proto = get_protocol(tup[0]);
//             let mut part = vec![proto.name.clone()];
//             if tup.len() > 1 && tup[1].is_some() {
//                 part.push(tup[1].as_ref().unwrap().clone());
//             }
//             part
//         })
//         .collect();
//     clean_path(&parts.join("/"))
// }

// fn clean_path(str: &str) -> String {
//     let str = String::from("/") + str.trim_matches('/').split('/').filter(|&a| !a.is_empty()).collect();
//     str
// }

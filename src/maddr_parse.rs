// // export function bytesToMultiaddrParts (bytes: Uint8Array): MultiaddrParts {
// //   const tuples: Tuple[] = []
// //   const stringTuples: StringTuple[] = []
// //   let path: string | null = null

// //   let i = 0
// //   while (i < bytes.length) {
// //     const code = varint.decode(bytes, i)
// //     const n = varint.encodingLength(code)

// //     const p = getProtocol(code)

// //     const size = sizeForAddr(p, bytes.slice(i + n))

// //     if (size === 0) {
// //       tuples.push([code])
// //       stringTuples.push([code])
// //       i += n
// //       // eslint-disable-next-line no-continue
// //       continue
// //     }

// //     const addr = bytes.slice(i + n, i + n + size)

// //     i += (size + n)

// //     if (i > bytes.length) { // did not end _exactly_ at buffer.length
// //       throw ParseError('Invalid address Uint8Array: ' + uint8ArrayToString(bytes, 'base16'))
// //     }

// //     // ok, tuple seems good.
// //     tuples.push([code, addr])
// //     const stringAddr = convertToString(code, addr)
// //     stringTuples.push([code, stringAddr])
// //     if (p.path === true) {
// //       // should we need to check each path part to see if it's a proto?
// //       // This would allow for other protocols to be added after a unix path,
// //       // however it would have issues if the path had a protocol name in the path
// //       path = stringAddr
// //       break
// //     }
// //   }

// //   return {
// //     bytes: Uint8Array.from(bytes),
// //     string: stringTuplesToString(stringTuples),
// //     tuples,
// //     stringTuples,
// //     path
// //   }
// // }
// use std::convert::TryInto;

// use crate::unsigned_varint;

// pub fn bytes_to_multiaddr_parts(bytes: &[u8]) -> MultiaddrParts {
//   let mut tuples: Vec<Tuple> = Vec::new();
//   let mut string_tuples: Vec<StringTuple> = Vec::new();
//   let mut path: Option<String> = None;

//   let mut i = 0;
//   while i < bytes.len() {
//     let (code, n) = unsigned_varint::decode_uint8_array(bytes, i).unwrap();
//     let p = get_protocol(code);

//     let size = size_for_addr(p, &bytes[i + n..]);

//     if size == 0 {
//       tuples.push(vec![code]);
//       string_tuples.push(vec![code]);
//       i += n;
//       continue;
//     }

//     let addr = &bytes[i + n..i + n + size];

//     i += size + n;

//     if i > bytes.len() {
//       panic!("Invalid address Uint8Array: {}", hex::encode(bytes));
//     }

//     tuples.push(vec![code, addr.to_vec()]);
//     let string_addr = convert_to_string(code, addr);
//     string_tuples.push(vec![code, string_addr.clone()]);
//     if p.path {
//       path = Some(string_addr);
//       break;
//     }
//   }

//   MultiaddrParts {
//     bytes: bytes.to_vec(),
//     string: string_tuples_to_string(&string_tuples),
//     tuples,
//     string_tuples,
//     path,
//   }
// }

// pub fn get_protocol(code: u64) -> &'static Protocol {
//   match code {
//     4 => &IPV4,
//     6 => &TCP,
//     33 => &DNS,
//     41 => &IPV6,
//     42 => &IPV6ZONE,
//     53 => &DNS4,
//     54 => &DNS6,
//     55 => &DNSADDR,
//     56 => &PROTOBUF,
//     421 => &P2P_WEBRTC_STAR,
//     422 => &P2P_WEBRTC_DIRECT,
//     443 => &P2P_CIRCUIT,
//     444 => &P2P_QUIC,
//     480 => &UNIX,
//     443 => &P2P_CIRCUIT,
//     480 => &UNIX,
//     777 => &MEMORY,
//     _ => &UNKNOWN,
//   }
// }

// pub fn size_for_addr(p: &Protocol, bytes: &[u8]) -> usize {
//   match p.size {
//     Size::Var => unsigned_varint::decode_uint8_array(bytes, 0).unwrap_or().1 as usize,
//     Size::Constant(n) => n,
//     Size::None => 0,
//   }
// }

// fn convert_to_string(code: u64, bytes: &[u8]) -> String {
//   match code {
//     4 | 41 => {
//       let ip = bytes.try_into().unwrap();
//       ip.to_string()
//     }
//     6 => {
//       let port = u16::from_be_bytes(bytes.try_into().unwrap());
//       port.to_string()
//     }
//     421 | 422 | 443 | 444 => {
//       let cid = Cid::try_from(bytes.to_vec()).unwrap();
//       cid.to_string()
//     }
//     480 => {
//       let path = std::path::PathBuf::from(std::str::from_utf8(bytes).unwrap());
//       path.to_string_lossy().to_string()
//     }
//     _ => hex::encode(bytes),
//   }
// }

// fn string_tuples_to_string(string_tuples: &[StringTuple]) -> String {
//   string_tuples
//     .iter()
//     .map(|tuple| {
//       tuple
//         .iter()
//         .map(|(code, string_addr)| format!("/{}{}", code, string_addr))
//         .collect::<Vec<String>>()
//         .join("")
//     })
//     .collect::<Vec<String>>()
//     .join("")
// }

// #[derive(Debug)]
// pub struct MultiaddrParts {
//   pub bytes: Vec<u8>,
//   pub string: String,
//   pub tuples: Vec<Tuple>,
//   pub string_tuples: Vec<StringTuple>,
//   pub path: Option<String>,
// }

// type Tuple = Vec<u8>;
// type StringTuple = Vec<(u64, String)>;

// #[derive(Debug)]
// pub struct Protocol {
//   pub code: u64,
//   pub name: &'static str,
//   pub size: Size,
//   pub path: bool,
// }

// #[derive(Debug)]
// pub enum Size {
//   Var,
//   Constant(usize),
//   None,
// }

// pub const IPV4: Protocol = Protocol {
//   code: 4,
//   name: "ip4",
//   size: Size::Constant(4),
//   path: false,
// };

// pub const TCP: Protocol = Protocol {
//   code: 6,
//   name: "tcp",
//   size: Size::Constant(2),
//   path: false,
// };

// pub const DNS: Protocol = Protocol {
//   code: 33,
//   name: "dns",
//   size: Size::Var,
//   path: false,
// };

// pub const IPV6: Protocol = Protocol {
//   code: 41,
//   name: "ip6",
//   size: Size::Constant(16),
//   path: false,
// };

// pub const IPV6ZONE: Protocol = Protocol {
//   code: 42,
//   name: "ip6zone",
//   size: Size::Var,
//   path: false,
// };

// pub const DNS4: Protocol = Protocol {
//   code: 53,
//   name: "dns4",
//   size: Size::Var,
//   path: false,
// };

// pub const DNS6: Protocol = Protocol {
//   code: 54,
//   name: "dns6",
//   size: Size::Var,
//   path: false,
// };

// pub const DNSADDR: Protocol = Protocol {
//   code: 55,
//   name: "dnsaddr",
//   size: Size::Var,
//   path: false,
// };

// pub const PROTOBUF: Protocol = Protocol {
//   code: 56,
//   name: "protobuf",
//   size: Size::Var,
//   path: false,
// };

// pub const P2P_WEBRTC_STAR: Protocol = Protocol {
//   code: 421,
//   name: "p2p-webrtc-star",
//   size: Size::Var,
//   path: false,
// };

// pub const P2P_WEBRTC_DIRECT: Protocol = Protocol {
//   code: 422,
//   name: "p2p-webrtc-direct",
//   size: Size::Var,
//   path: false,
// };

// pub const P2P_CIRCUIT: Protocol = Protocol {
//   code: 443,
//   name: "p2p-circuit",
//   size: Size::Var,
//   path: false,
// };

// pub const P2P_QUIC: Protocol = Protocol {
//   code: 444,
//   name: "p2p-quic",
//   size: Size::Var,
//   path: false,
// };

// pub const UNIX: Protocol = Protocol {
//   code: 480,
//   name: "unix",
//   size: Size::None,
//   path: true,
// };

// pub const MEMORY: Protocol = Protocol {
//   code: 777,
//   name: "memory",
//   size: Size::Var,
//   path: false,
// };

// pub const UNKNOWN: Protocol = Protocol {
//   code: 0,
//   name: "unknown",
//   size: Size::None,
//   path: false,
// };

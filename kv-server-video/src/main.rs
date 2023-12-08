pub mod pb;

use prost::Message;

use pb::Request;

use crate::pb::{request, RequestGet, RequestPut, Response};

fn main() {
    let request = RequestPut {
        key: "a".to_string(),
        value: vec![1, 2],
    };
    let mut buf: Vec<u8> = vec![];
    request.encode(&mut buf).unwrap();
    println!(
        "encoded: {:?}",
        buf.iter()
            .map(|n| format!("{:0>8}", format!("{n:b}")))
            .collect::<Vec<_>>()
    );
    let request = Request {
        command: Some(request::Command::Get(RequestGet {
            key: "a".to_string(),
        })),
    };
    let mut buf: Vec<u8> = vec![];
    request.encode(&mut buf).unwrap();
    println!(
        "encoded: {:?}",
        buf.iter()
            .map(|n| format!("{:0>8}", format!("{n:b}")))
            .collect::<Vec<_>>()
    );
    let request = Request {
        command: Some(request::Command::Put(RequestPut {
            key: "a".to_string(),
            value: vec![1, 2, 2],
        })),
    };
    let mut buf: Vec<u8> = vec![];
    request.encode(&mut buf).unwrap();
    println!(
        "encoded: {:?}",
        buf.iter()
            .map(|n| format!("{:0>8}", format!("{n:b}")))
            .collect::<Vec<_>>()
    );
    // let request = RequestGet {
    //     key: "a".to_string(),
    // };
    let request = Response {
        code: 1,
        key: "a".into(),
        value: "aa".into(),
    };
    let mut buf: Vec<u8> = vec![];
    request.encode(&mut buf).unwrap();
    println!(
        "encoded: {:?}",
        buf.iter()
            .map(|n| format!("{:0>8}", format!("{n:b}")))
            .collect::<Vec<_>>()
    );
}

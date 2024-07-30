/*
Implementation heavily based on https://github.com/horrorho/burnt-cookie/blob/master/src/cookies.rs

The MIT License

Copyright 2016 Ahseya.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

use std::io;
use std::io::{Error, ErrorKind};

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use log::warn;

pub struct Cookie {
    #[allow(dead_code)]
    pub prefix: String,
    #[allow(dead_code)]
    pub url: String,
    #[allow(dead_code)]
    pub is_raw: bool,
    #[allow(dead_code)]
    pub path: String,
    #[allow(dead_code)]
    pub is_secure: bool,
    #[allow(dead_code)]
    pub expiry: f64,
    pub name: String,
    pub value: String,
}

pub struct Cookies {
    http_only: bool,
    pub cookies: Vec<Cookie>,
}

impl Cookies {
    pub fn new(http_only: bool) -> Cookies {
        Cookies {
            http_only,
            cookies: Vec::new(),
        }
    }

    pub fn parse_content(&mut self, bs: &[u8]) -> io::Result<()> {
        // Magic bytes: "COOK" = 0x636F6F6B
        if slice(bs, 0, 4)? != [0x63, 0x6F, 0x6F, 0x6B] {
            return Err(Error::new(ErrorKind::InvalidData, "not a cookie file"));
        }

        let count = slice(bs, 4, 4).map(BigEndian::read_u32)? as usize;
        parse_table::<BigEndian>(&bs[8..], count)?
            .iter()
            .fold(count * 4 + 8, |off, &len| {
                slice(bs, off, len)
                    .and_then(|u| self.parse_page(u))
                    .unwrap_or_else(|err| warn!("page parse failure: {}", err));
                off + len
            });
        Ok(())
    }

    fn parse_page(&mut self, bs: &[u8]) -> io::Result<()> {
        if slice(bs, 0, 4)? != [0x00, 0x00, 0x01, 0x00] {
            return Err(Error::new(ErrorKind::InvalidData, "bad page header"));
        }

        let count = slice(bs, 4, 4).map(LittleEndian::read_u32)? as usize;
        parse_table::<LittleEndian>(&bs[8..], count)?
            .iter()
            .fold((), |_, &off| {
                slice(bs, off, 4)
                    .map(LittleEndian::read_u32)
                    .and_then(|len| slice(bs, off, len as usize))
                    .and_then(|u| self.parse_cookie::<LittleEndian>(u))
                    .unwrap_or_else(|err| warn!("cookie parse failure: {}", err))
            });

        if slice(bs, count * 4 + 8, 4)? != [0x00, 0x00, 0x00, 0x00] {
            return Err(Error::new(ErrorKind::InvalidData, "bad page trailer"));
        }
        Ok(())
    }

    fn parse_cookie<T: ByteOrder>(&mut self, bs: &[u8]) -> io::Result<()> {
        if bs.len() < 0x30 {
            return Err(Error::new(ErrorKind::InvalidData, "cookie data underflow"));
        }
        let flags = T::read_u32(&bs[0x08..0x0C]);

        let url_off = T::read_u32(&bs[0x10..0x14]) as usize;
        let name_off = T::read_u32(&bs[0x14..0x18]) as usize;
        let path_off = T::read_u32(&bs[0x18..0x1C]) as usize;
        let value_off = T::read_u32(&bs[0x1C..0x20]) as usize;

        // i/OS/X to Unix timestamp +(1 Jan 2001 epoch seconds).
        let expiry = T::read_f64(&bs[0x28..0x30]) + 978307200f64;

        let url = slice_to(bs, url_off, name_off).and_then(c_str)?;
        let name = slice_to(bs, name_off, path_off).and_then(c_str)?;
        let path = slice_to(bs, path_off, value_off).and_then(c_str)?;
        let value = slice_to(bs, value_off, bs.len()).and_then(c_str)?;

        let is_raw = url.starts_with('.');

        let is_secure = flags & 0x01 == 0x01;
        let is_http_only = flags & 0x04 == 0x04;

        let prefix = if is_http_only && self.http_only {
            "#HttpOnly_".to_owned()
        } else {
            "".to_owned()
        };

        self.cookies.push(Cookie {
            prefix,
            url,
            is_raw,
            path,
            is_secure,
            expiry,
            name,
            value,
        });

        Ok(())
    }
}

fn slice(bs: &[u8], off: usize, len: usize) -> io::Result<&[u8]> {
    if off + len > bs.len() {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("data underflow: {}", off + len - bs.len()),
        ))
    } else {
        Ok(&bs[off..off + len])
    }
}

fn parse_table<T: ByteOrder>(bs: &[u8], count: usize) -> io::Result<Vec<usize>> {
    let end = count * 4;
    if end > bs.len() {
        return Err(Error::new(ErrorKind::InvalidData, "table data underflow"));
    }
    let data = (bs[..end])
        .chunks(4)
        .map(|u| T::read_u32(u) as usize)
        .collect();
    Ok(data)
}

fn slice_to(bs: &[u8], off: usize, to: usize) -> io::Result<&[u8]> {
    if to < off {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("negative data length: {}", to - off),
        ))
    } else {
        slice(bs, off, to - off)
    }
}

fn c_str(bs: &[u8]) -> io::Result<String> {
    bs.split_last()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "null c string"))
        .and_then(|(&last, elements)| {
            if last == 0x00 {
                Ok(elements)
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "c string non null terminator",
                ))
            }
        })
        .and_then(|elements| {
            String::from_utf8(elements.to_vec())
                .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
        })
}

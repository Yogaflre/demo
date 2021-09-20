use std::{
    fs::File,
    io::{self, Error, ErrorKind, Read},
    path::Path,
};

use memmap::{Mmap, MmapOptions};

pub struct Reader;

impl Reader {
    pub fn search_by_key(path: &Path, key: &String) -> io::Result<Option<String>> {
        let buf: Mmap = unsafe { MmapOptions::new().map(&File::open(path)?)? };
        let mut offset = 0;
        while let Some((k, v)) = Self::read_by_mmap(&buf, &mut offset)? {
            if &k == key {
                return Ok(v);
            }
        }
        return Ok(None);
    }

    pub fn read_by_mmap(
        buf: &Mmap,
        offset: &mut usize,
    ) -> Result<Option<(String, Option<String>)>, Error> {
        if *offset < buf.len() {
            let is_delete = buf[*offset] == 1_u8;
            *offset += 1;
            let kv: (String, Option<String>);
            if is_delete {
                let key_size = buf[*offset];
                *offset += 1;
                let start = *offset;
                *offset += key_size as usize;
                let key = String::from_utf8(buf[start..*offset].to_vec()).unwrap();
                kv = (key, None);
            } else {
                let key_size = buf[*offset] as usize;
                *offset += 1;
                let val_size = buf[*offset] as usize;
                *offset += 1;
                let start = *offset;
                *offset += key_size;
                let key = String::from_utf8(buf[start..*offset].to_vec()).unwrap();
                let start = *offset;
                *offset += val_size;
                let val = String::from_utf8(buf[start..*offset].to_vec()).unwrap();
                kv = (key, Some(val));
            }
            return Ok(Some(kv));
        } else {
            return Ok(None);
        }
    }

    pub fn read_by_seek(reader: &mut dyn Read) -> Result<Option<(String, Option<String>)>, Error> {
        let mut buf: Vec<u8> = vec![0; 1];
        match reader.read_exact(&mut buf) {
            Ok(_) => {
                let is_delete = buf[0] == 1_u8;
                if is_delete {
                    buf = vec![0; 1];
                    reader.read_exact(&mut buf).unwrap();
                    let key_size = buf[0];

                    buf = vec![0; key_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let key = String::from_utf8(buf.clone()).unwrap();

                    return Ok(Some((key, None)));
                } else {
                    buf = vec![0; 2];
                    reader.read_exact(&mut buf).unwrap();
                    let key_size = buf[0];
                    let value_size = buf[1];

                    buf = vec![0; key_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let key = String::from_utf8(buf.clone()).unwrap();

                    buf = vec![0; value_size as usize];
                    reader.read_exact(&mut buf).unwrap();
                    let val = String::from_utf8(buf.clone()).unwrap();

                    return Ok(Some((key, Some(val))));
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::UnexpectedEof => return Ok(None),
                _ => panic!("read file error: {}", error),
            },
        }
    }
}

use std::io;
use std::io::Write;

pub struct Writer;

impl Writer {
    pub fn write_by_seek(
        writer: &mut dyn Write,
        key: &String,
        val: Option<&String>,
    ) -> io::Result<()> {
        let key_bytes = key.as_bytes();
        if key_bytes.len() > 255 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("key size invalid! key bytes len:{}", key_bytes.len(),),
            ));
        }
        if let Some(v) = val {
            let val_bytes = v.as_bytes();
            if val_bytes.len() > 255 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("value size invalid! value bytes len:{}", val_bytes.len()),
                ));
            }
            writer.write_all(&(false as u8).to_le_bytes())?;
            writer.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(&(val_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(key_bytes)?;
            writer.write_all(val_bytes)?;
        } else {
            writer.write_all(&(true as u8).to_le_bytes())?;
            writer.write_all(&(key_bytes.len() as u8).to_le_bytes())?;
            writer.write_all(key_bytes)?;
        }
        return Ok(());
    }
}

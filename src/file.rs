use std::{io::{Read, Write}, ops::{Deref, DerefMut}};

pub struct FileBuffer(Vec<u8>);

impl FileBuffer {
    pub fn make(buffer: impl Into<Vec<u8>>) -> Self {
        return Self(buffer.into());
    }
}

impl FileBuffer {
    pub fn get_buf(self) -> Vec<u8> {
        return self.0;
    }
}

impl Write for FileBuffer {
    fn by_ref(&mut self) -> &mut Self
        where
            Self: Sized, {
        return self;
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        return self.0.write(buf);
    }

    fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        return self.0.write_all(buf);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        return self.0.flush();
    }
}

impl Deref for FileBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for FileBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
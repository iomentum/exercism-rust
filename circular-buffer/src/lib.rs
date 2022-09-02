use std::marker::PhantomData;

pub struct CircularBuffer<T> {
    capacity: usize,
    buffer: Vec<Option<T>>,
    read_id: usize,
    write_id: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            buffer: (0..capacity).map(|_| None).collect(),
            read_id: 0,
            write_id: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer[self.write_id].is_some() {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.write_id] = Some(element);
            self.increment_id("write_id".to_string());
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Ok(element) = self.buffer[self.read_id].take().ok_or(Error::EmptyBuffer) {
            self.increment_id("read_id".to_string());
            Ok(element)
        } else {
            return Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.buffer = (0..self.capacity).map(|_| None).collect();
        self.write_id = 0;
        self.read_id = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        let is_written = self.buffer[self.write_id].is_some();
        self.buffer[self.write_id] = Some(element);

        if is_written {
            self.increment_id("read_id".to_string());
        }
        self.increment_id("write_id".to_string());
    }

    fn increment_id(&mut self, id_name: String) {
        if id_name == "read_id".to_string() {
            if self.read_id == self.capacity - 1 {
                self.read_id = 0;
            } else {
                self.read_id += 1;
            }
        } else {
            if self.write_id == self.capacity - 1 {
                self.write_id = 0;
            } else {
                self.write_id += 1;
            }
        }
    }
}

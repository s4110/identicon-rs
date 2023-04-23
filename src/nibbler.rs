use std::slice::Iter;

pub struct Nibbler<'a> {
    byte: Option<u8>,
    bytes: Iter<'a, u8>,
}

impl <'a> Nibbler<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes: bytes.iter(), byte: None }
    }
}

impl <'a> Iterator for Nibbler<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.byte {
            Some(v) => {
                self.byte = None;
                Some(v)
            }
            None => match self.bytes.next() {
                Some(v) => {
                    let hi = *v & 0xf0;
                    let lo = *v & 0x0f;
                    self.byte = Some(lo);
                    Some(hi >> 4)
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Nibbler;

    #[test]
    fn nibbles_iter() {
        let bytes = vec![0x2a];
        let nibbles = Nibbler::new(&bytes);
        let result: Vec<u8> = nibbles.collect();

        assert_eq!(vec![0x02, 0x0a], result);
    }
}
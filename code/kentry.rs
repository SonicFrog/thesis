struct RedisKeyEntry {
    pkt: Vec<CrossPacket>,
    offset: usize,
    len: usize,
}

impl Deref for RedisKEntry {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(self.pkt.get_payload(self.offset),
                                  self.len)
        }
    }
}

impl Clone for CrossPacket {
    fn clone(&self) -> Self {
        unsafe {
            mbuf_ref(self.payload as *mut MBuf);
        }

        CrossPacket::new(self.payload)
    }
}

impl Drop for CrossPacket {
    fn drop(&mut self) {
        unsafe {
            mbuf_free(self.payload as *mut MBuf);
        }
    }
}

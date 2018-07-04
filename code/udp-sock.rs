pub fn bind<F>(&self,
               addr: Ipv4Addr,
               port: u16,
               read_cb: F) -> Arc<UdpSocket>
where F: Fn(CrossPacket, Ipv4Addr, u16) + 'static;

pub fn send(&self, pkt: &CrossPacket,
            addr: &Ipv4Addr,
            port: u16);

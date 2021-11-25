extern crate binread;

mod vicon_udp {
    use binread::BinRead;

    #[derive(BinRead,Debug)]
    #[repr(C)]
    pub struct TrackerObject {
        pub name: [u8;24],
        pub trans_x: f64,
        pub trans_y: f64,
        pub trans_z: f64,
        pub rot_x: f64,
        pub rot_y: f64,
        pub rot_z: f64,
    }

    use std::fmt;
    impl fmt::Display for TrackerObject {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let string_end = self.name.iter().position(|&c| c == b'\0').unwrap_or(self.name.len()-1);
            let name = std::ffi::CStr::from_bytes_with_nul(&self.name[0..string_end+1]).unwrap();
            write!(f, "{{\n  \"name\": {:#?},\n  \"translation\": [{:>7.3},{:>7.3},{:>7.3}],\n  \"rotation\": [{:>7.4},{:>7.4},{:>7.4}]\n}}",
                name,
                self.trans_x/1000.0, self.trans_y/1000.0, self.trans_z/1000.0,
                self.rot_x, self.rot_y, self.rot_z
            )
        }
    }

    #[derive(BinRead)]
    #[repr(C)]
    pub struct Item {
        pub item_id: u8,
        pub data_size: u16,
        pub object: TrackerObject,
    }

    #[derive(BinRead)]
    #[repr(C)]
    pub struct Packet {
        pub frame_number: u32,
        pub items_in_block: u8,
        #[br(count=items_in_block)]
        pub items: Vec<Item>
}

}

use std::net::UdpSocket;
use std::io::Cursor;

use binread::prelude::*;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:51001")?;

    loop {
        let mut buf = [0;1024];
        socket.recv_from(&mut buf)?;

        let packet: vicon_udp::Packet = Cursor::new(buf).read_ne().unwrap();

        for vicon_udp::Item { object, .. } in packet.items {
            println!("{}",object);
        }
    }

    Ok(())
}

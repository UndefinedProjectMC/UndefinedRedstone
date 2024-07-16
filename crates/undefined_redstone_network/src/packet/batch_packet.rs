use std::io::Error;
use binary_util::{ByteReader, ByteWriter};
use binary_util::interfaces::{Reader, Writer};
use undefined_redstone_protocol::MinecraftPacket;

pub struct BatchPacket {
    packets: Vec<MinecraftPacket>,
    index: usize,
}

impl BatchPacket {
    pub fn new() -> Self {
        Self {
            packets: vec![],
            index: 0,
        }
    }

    pub fn from_vec(packets: Vec<MinecraftPacket>) -> Self {
        Self {
            packets,
            index: 0,
        }
    }

    pub fn single(packet: MinecraftPacket) -> Self {
        Self {
            packets: vec![packet],
            index: 0,
        }
    }

    pub fn push_packet(&mut self, packet: MinecraftPacket) -> &mut Self {
        self.packets.push(packet);
        self
    }
}

impl Iterator for BatchPacket {
    type Item = MinecraftPacket;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.packets.get(self.index - 1).and_then(|packet| {
            Some(packet.clone())
        })
    }
}

impl Reader<BatchPacket> for BatchPacket {
    fn read(buf: &mut ByteReader) -> Result<BatchPacket, Error> {
        if buf.read_u8()? == 0xfe {//是否为batch packet
            let mut packets = Vec::new();
            //读取BatchPacket
            while buf.as_slice().len() != 0 {
                //读取packet
                if let Ok(packet) = buf.read_sized_slice() {
                    //处理packet
                    let mut packet_reader = ByteReader::from(packet);
                    let packet_flags = packet_reader.read_var_u32()?;//读取packet flags
                    let packet_id = (packet_flags & 0x3ff) as u8;
                    println!("BATCH PACKET: PACKET ID: {}", packet_id);
                    let packet_data = packet_reader.as_slice();//读取包内容

                    //编为可以供MinecraftPacket解析的slice
                    let mut byte_writer = ByteWriter::new();
                    byte_writer.write_u8(packet_id)?;
                    byte_writer.write(packet_data)?;
                    let mut packet_reader = ByteReader::from(byte_writer.as_slice());

                    packets.push(MinecraftPacket::read(&mut packet_reader)?);
                }
            }
            Ok(BatchPacket {
                packets,
                index: 0,
            })
        }else {
            Err(Error::other("Not batch packet"))
        }
    }
}

impl Writer for BatchPacket {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        buf.write_u8(0xfe)?;
        for packet in &self.packets {
            //分离packet id和data
            let byte_writer = packet.write_to_bytes()?;
            let packet_bytes = byte_writer.as_slice();
            let packet_id = packet_bytes[0..1].get(0).unwrap().clone();
            let packet_data = &packet_bytes[1..packet_bytes.len()];

            //添加序列
            let mut byte_writer = ByteWriter::new();
            byte_writer.write_var_u32(packet_id as u32 | (0 << 10) | (0 << 12))?;//写入packet id
            byte_writer.write(packet_data)?;
            let packet_bytes = byte_writer.as_slice();

            buf.write_slice(packet_bytes)?;
        }
        Ok(())
    }
}
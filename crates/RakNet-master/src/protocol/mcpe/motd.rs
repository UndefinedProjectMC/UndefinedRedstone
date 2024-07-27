use binary_util::interfaces::{Reader, Writer};
use binary_util::io::{ByteReader, ByteWriter};
use undefined_redstone_type::gamemode::Gamemode;

/// Protocol wise, motd is just a string
/// However we're using this struct to represent the motd
#[derive(Debug, Clone)]
pub struct Motd {
    /// The name of the server
    pub motd: String,
    /// The protocol version
    pub protocol: u16,
    /// The version of the server
    pub version: String,
    /// The number of players online
    pub player_online: u32,
    /// The maximum number of players
    pub player_max: u32,
    /// The server's GUID
    pub server_guid: u64,
    /// The level's name
    pub level_name: String,
    /// The gamemode of the server
    pub gamemode: Gamemode,
}

impl Motd {
    pub fn new<S: Into<String>>(server_guid: u64, port: S) -> Self {
        Self {
            motd: "UndefinedRedstore Server".to_string(),
            protocol: 121,
            version: "1.0".into(),
            player_online: 0,
            player_max: 100,
            server_guid,
            level_name: "UndefinedRedstore Server".to_string(),
            gamemode: Gamemode::Survival,
        }
    }

    /// Takes the Motd and parses it into a valid MCPE
    /// MOTD buffer.
    pub fn write(&self) -> String {
        let props: Vec<String> = vec![
            "MCPE".into(),
            self.motd.clone(),
            self.protocol.to_string(),
            self.version.clone(),
            self.player_online.to_string(),
            self.player_max.to_string(),
            self.server_guid.to_string(),
            self.level_name.to_string(),
            self.gamemode.as_str().to_string(),
            "1".to_string(),
        ];
        props.join(";").into()
    }
}

impl Reader<Motd> for Motd {
    fn read(buf: &mut ByteReader) -> Result<Motd, std::io::Error> {
        let str_len = buf.read_u16()?;
        let mut str_buf = vec![0; str_len as usize];

        buf.read(&mut str_buf)?;

        let motd = String::from_utf8(str_buf).unwrap();

        let parts = motd
            .split(";")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let motd = parts
            .get(1)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd name",
            ))?
            .clone();

        let protocol = parts
            .get(2)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd protocol",
            ))?
            .clone();

        let version = parts
            .get(3)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd version",
            ))?
            .clone();

        let player_online = parts
            .get(4)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd player online",
            ))?
            .clone();

        let player_max = parts
            .get(5)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd player max",
            ))?
            .clone();

        let server_guid = parts
            .get(6)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd server guid",
            ))?
            .clone();

        let level_name = parts
            .get(7)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid level name",
            ))?
            .clone();

        let gamemode = parts
            .get(8)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid motd gamemode",
            ))?
            .clone();

        Ok(Motd {
            motd,
            protocol: protocol.as_str().parse().unwrap(),
            version,
            player_online: player_online.parse().unwrap(),
            player_max: player_max.parse().unwrap(),
            server_guid: server_guid.parse().unwrap(),
            level_name,
            gamemode: match gamemode
                .as_str()
                .parse::<u8>()
                .expect("Gamemode is not a byte")
            {
                0 => Gamemode::Survival,
                1 => Gamemode::Creative,
                2 => Gamemode::Adventure,
                3 => Gamemode::Spectator,
                _ => Gamemode::Survival,
            },
        })
    }
}

impl Writer for Motd {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        let motd = self.write();
        let motd_len = motd.len() as u16;
        buf.write_u16(motd_len)?;
        buf.write(motd.as_bytes())?;
        Ok(())
    }
}

use statik_derive::Packet;
use uuid::Uuid;

#[derive(Debug, Packet)]
#[packet_id = 0x00]
pub struct C2SLoginStart {
    pub username: String,
    pub uuid: Option<Uuid>,
}

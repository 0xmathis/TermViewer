pub mod appn;
pub mod com;
pub mod dht;
pub mod dqt;
pub mod sof0;
pub mod sos;

#[derive(Debug, PartialEq)]
pub enum SegmentType {
    APPN,
    COM,
    DHT,
    DQT,
    EOI,
    SOF0,
    SOI,
    SOS,
}

impl SegmentType {
    pub fn get_marker(&self) -> [u8; 2] {
        match self {
            SegmentType::APPN  => [0xff, 0xe0],
            SegmentType::COM   => [0xff, 0xfe],
            SegmentType::DHT   => [0xff, 0xc4],
            SegmentType::DQT   => [0xff, 0xdb],
            SegmentType::EOI   => [0xff, 0xd9],
            SegmentType::SOF0  => [0xff, 0xc0],
            SegmentType::SOI   => [0xff, 0xd8],
            SegmentType::SOS   => [0xff, 0xda],
        }
    }

    pub fn from_marker(marker: [u8; 2]) -> Option<Self> {
        match marker {
            [0xff, 0xe0..=0xef] => Some(SegmentType::APPN),
            [0xff, 0xfe]        => Some(SegmentType::COM),
            [0xff, 0xc4]        => Some(SegmentType::DHT),
            [0xff, 0xdb]        => Some(SegmentType::DQT),
            [0xff, 0xd9]        => Some(SegmentType::EOI),
            [0xff, 0xc0]        => Some(SegmentType::SOF0),
            [0xff, 0xd8]        => Some(SegmentType::SOI),
            [0xff, 0xda]        => Some(SegmentType::SOS),
            _                   => None,
        }
    }
}

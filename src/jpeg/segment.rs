#[derive(Debug, PartialEq)]
pub enum SegmentType {
    APPN,
    COM,
    DAC,
    DHP,
    DHT,
    DNL,
    DQT,
    DRI,
    EOI,
    EXP,
    JPGN,
    RSTN,
    SOF0,
    SOI,
    SOS,
    TEM,
}

impl SegmentType {
    pub fn from_marker(marker: [u8; 2]) -> Option<Self> {
        match marker {
            [0xff, 0x01]        => Some(SegmentType::TEM),
            [0xff, 0xc0]        => Some(SegmentType::SOF0),
            [0xff, 0xc4]        => Some(SegmentType::DHT),
            [0xff, 0xcc]        => Some(SegmentType::DAC),
            [0xff, 0xd0..=0xd7] => Some(SegmentType::RSTN),
            [0xff, 0xd8]        => Some(SegmentType::SOI),
            [0xff, 0xd9]        => Some(SegmentType::EOI),
            [0xff, 0xda]        => Some(SegmentType::SOS),
            [0xff, 0xdb]        => Some(SegmentType::DQT),
            [0xff, 0xdc]        => Some(SegmentType::DNL),
            [0xff, 0xdd]        => Some(SegmentType::DRI),
            [0xff, 0xde]        => Some(SegmentType::DHP),
            [0xff, 0xdf]        => Some(SegmentType::EXP),
            [0xff, 0xe0..=0xef] => Some(SegmentType::APPN),
            [0xff, 0xf0..=0xfd] => Some(SegmentType::JPGN),
            [0xff, 0xfe]        => Some(SegmentType::COM),
            _                   => None,
        }
    }
}

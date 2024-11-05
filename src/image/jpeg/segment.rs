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
    pub fn from_marker(marker: u16) -> Option<Self> {
        match marker {
            0xFF01          => Some(SegmentType::TEM),
            0xFFC0          => Some(SegmentType::SOF0),
            0xFFC4          => Some(SegmentType::DHT),
            0xFFCC          => Some(SegmentType::DAC),
            0xFFD0..=0xFFD7 => Some(SegmentType::RSTN),
            0xFFD8          => Some(SegmentType::SOI),
            0xFFD9          => Some(SegmentType::EOI),
            0xFFDA          => Some(SegmentType::SOS),
            0xFFDB          => Some(SegmentType::DQT),
            0xFFDC          => Some(SegmentType::DNL),
            0xFFDD          => Some(SegmentType::DRI),
            0xFFDE          => Some(SegmentType::DHP),
            0xFFDF          => Some(SegmentType::EXP),
            0xFFE0..=0xFFEF => Some(SegmentType::APPN),
            0xFFF0..=0xFFFD => Some(SegmentType::JPGN),
            0xFFFE          => Some(SegmentType::COM),
            _               => None,
        }
    }
}

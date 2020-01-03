use crate::types::square::Square;

#[derive(Clone)]
pub struct Magic {
    pub mask: u64,
    pub factor: u64,
    pub offset: u64,
}

impl Magic {
    pub const SIZE: usize = 88772;

    pub const BISHOP_SHIFT: usize = 9;
    pub const ROOK_SHIFT: usize = 12;

    pub const ROOK: [Magic; Square::NUM_SQUARES] = [
        Magic {
            mask: 0x0001_0101_0101_017e,
            factor: 0x0028_0077_ffeb_fffe,
            offset: 26304,
        },
        Magic {
            mask: 0x0002_0202_0202_027c,
            factor: 0x2004_0102_0109_7fff,
            offset: 35520,
        },
        Magic {
            mask: 0x0004_0404_0404_047a,
            factor: 0x0010_0200_1005_3fff,
            offset: 38592,
        },
        Magic {
            mask: 0x0008_0808_0808_0876,
            factor: 0x0040_0400_0800_4002,
            offset: 8026,
        },
        Magic {
            mask: 0x0010_1010_1010_106e,
            factor: 0x7fd0_0441_ffff_d003,
            offset: 22196,
        },
        Magic {
            mask: 0x0020_2020_2020_205e,
            factor: 0x4020_0088_87df_fffe,
            offset: 80870,
        },
        Magic {
            mask: 0x0040_4040_4040_403e,
            factor: 0x0040_0088_8847_ffff,
            offset: 76747,
        },
        Magic {
            mask: 0x0080_8080_8080_807e,
            factor: 0x0068_00fb_ff75_fffd,
            offset: 30400,
        },
        Magic {
            mask: 0x0001_0101_0101_7e00,
            factor: 0x0000_2801_0113_ffff,
            offset: 11115,
        },
        Magic {
            mask: 0x0002_0202_0202_7c00,
            factor: 0x0020_0402_01fc_ffff,
            offset: 18205,
        },
        Magic {
            mask: 0x0004040404047a00,
            factor: 0x007fe80042ffffe8,
            offset: 53577,
        },
        Magic {
            mask: 0x0008080808087600,
            factor: 0x00001800217fffe8,
            offset: 62724,
        },
        Magic {
            mask: 0x0010101010106e00,
            factor: 0x00001800073fffe8,
            offset: 34282,
        },
        Magic {
            mask: 0x0020202020205e00,
            factor: 0x00001800e05fffe8,
            offset: 29196,
        },
        Magic {
            mask: 0x0040404040403e00,
            factor: 0x00001800602fffe8,
            offset: 23806,
        },
        Magic {
            mask: 0x0080808080807e00,
            factor: 0x000030002fffffa0,
            offset: 49481,
        },
        Magic {
            mask: 0x00010101017e0100,
            factor: 0x00300018010bffff,
            offset: 2410,
        },
        Magic {
            mask: 0x00020202027c0200,
            factor: 0x0003000c0085fffb,
            offset: 36498,
        },
        Magic {
            mask: 0x00040404047a0400,
            factor: 0x0004000802010008,
            offset: 24478,
        },
        Magic {
            mask: 0x0008080808760800,
            factor: 0x0004002020020004,
            offset: 10074,
        },
        Magic {
            mask: 0x00101010106e1000,
            factor: 0x0001002002002001,
            offset: 79315,
        },
        Magic {
            mask: 0x00202020205e2000,
            factor: 0x0001001000801040,
            offset: 51779,
        },
        Magic {
            mask: 0x00404040403e4000,
            factor: 0x0000004040008001,
            offset: 13586,
        },
        Magic {
            mask: 0x00808080807e8000,
            factor: 0x0000006800cdfff4,
            offset: 19323,
        },
        Magic {
            mask: 0x000101017e010100,
            factor: 0x0040200010080010,
            offset: 70612,
        },
        Magic {
            mask: 0x000202027c020200,
            factor: 0x0000080010040010,
            offset: 83652,
        },
        Magic {
            mask: 0x000404047a040400,
            factor: 0x0004010008020008,
            offset: 63110,
        },
        Magic {
            mask: 0x0008080876080800,
            factor: 0x0000040020200200,
            offset: 34496,
        },
        Magic {
            mask: 0x001010106e101000,
            factor: 0x0002008010100100,
            offset: 84966,
        },
        Magic {
            mask: 0x002020205e202000,
            factor: 0x0000008020010020,
            offset: 54341,
        },
        Magic {
            mask: 0x004040403e404000,
            factor: 0x0000008020200040,
            offset: 60421,
        },
        Magic {
            mask: 0x008080807e808000,
            factor: 0x0000820020004020,
            offset: 86402,
        },
        Magic {
            mask: 0x0001017e01010100,
            factor: 0x00fffd1800300030,
            offset: 50245,
        },
        Magic {
            mask: 0x0002027c02020200,
            factor: 0x007fff7fbfd40020,
            offset: 76622,
        },
        Magic {
            mask: 0x0004047a04040400,
            factor: 0x003fffbd00180018,
            offset: 84676,
        },
        Magic {
            mask: 0x0008087608080800,
            factor: 0x001fffde80180018,
            offset: 78757,
        },
        Magic {
            mask: 0x0010106e10101000,
            factor: 0x000fffe0bfe80018,
            offset: 37346,
        },
        Magic {
            mask: 0x0020205e20202000,
            factor: 0x0001000080202001,
            offset: 370,
        },
        Magic {
            mask: 0x0040403e40404000,
            factor: 0x0003fffbff980180,
            offset: 42182,
        },
        Magic {
            mask: 0x0080807e80808000,
            factor: 0x0001fffdff9000e0,
            offset: 45385,
        },
        Magic {
            mask: 0x00017e0101010100,
            factor: 0x00fffefeebffd800,
            offset: 61659,
        },
        Magic {
            mask: 0x00027c0202020200,
            factor: 0x007ffff7ffc01400,
            offset: 12790,
        },
        Magic {
            mask: 0x00047a0404040400,
            factor: 0x003fffbfe4ffe800,
            offset: 16762,
        },
        Magic {
            mask: 0x0008760808080800,
            factor: 0x001ffff01fc03000,
            offset: 0,
        },
        Magic {
            mask: 0x00106e1010101000,
            factor: 0x000fffe7f8bfe800,
            offset: 38380,
        },
        Magic {
            mask: 0x00205e2020202000,
            factor: 0x0007ffdfdf3ff808,
            offset: 11098,
        },
        Magic {
            mask: 0x00403e4040404000,
            factor: 0x0003fff85fffa804,
            offset: 21803,
        },
        Magic {
            mask: 0x00807e8080808000,
            factor: 0x0001fffd75ffa802,
            offset: 39189,
        },
        Magic {
            mask: 0x007e010101010100,
            factor: 0x00ffffd7ffebffd8,
            offset: 58628,
        },
        Magic {
            mask: 0x007c020202020200,
            factor: 0x007fff75ff7fbfd8,
            offset: 44116,
        },
        Magic {
            mask: 0x007a040404040400,
            factor: 0x003fff863fbf7fd8,
            offset: 78357,
        },
        Magic {
            mask: 0x0076080808080800,
            factor: 0x001fffbfdfd7ffd8,
            offset: 44481,
        },
        Magic {
            mask: 0x006e101010101000,
            factor: 0x000ffff810280028,
            offset: 64134,
        },
        Magic {
            mask: 0x005e202020202000,
            factor: 0x0007ffd7f7feffd8,
            offset: 41759,
        },
        Magic {
            mask: 0x003e404040404000,
            factor: 0x0003fffc0c480048,
            offset: 1394,
        },
        Magic {
            mask: 0x007e808080808000,
            factor: 0x0001ffffafd7ffd8,
            offset: 40910,
        },
        Magic {
            mask: 0x7e01010101010100,
            factor: 0x00ffffe4ffdfa3ba,
            offset: 66516,
        },
        Magic {
            mask: 0x7c02020202020200,
            factor: 0x007fffef7ff3d3da,
            offset: 3897,
        },
        Magic {
            mask: 0x7a04040404040400,
            factor: 0x003fffbfdfeff7fa,
            offset: 3930,
        },
        Magic {
            mask: 0x7608080808080800,
            factor: 0x001fffeff7fbfc22,
            offset: 72934,
        },
        Magic {
            mask: 0x6e10101010101000,
            factor: 0x0000020408001001,
            offset: 72662,
        },
        Magic {
            mask: 0x5e20202020202000,
            factor: 0x0007fffeffff77fd,
            offset: 56325,
        },
        Magic {
            mask: 0x3e40404040404000,
            factor: 0x0003ffffbf7dfeec,
            offset: 66501,
        },
        Magic {
            mask: 0x7e80808080808000,
            factor: 0x0001ffff9dffa333,
            offset: 14826,
        },
    ];

    pub const BISHOP: [Magic; Square::NUM_SQUARES] = [
        Magic {
            mask: 0x0040201008040200,
            factor: 0x007fbfbfbfbfbfff,
            offset: 5378,
        },
        Magic {
            mask: 0x0000402010080400,
            factor: 0x0000a060401007fc,
            offset: 4093,
        },
        Magic {
            mask: 0x0000004020100a00,
            factor: 0x0001004008020000,
            offset: 4314,
        },
        Magic {
            mask: 0x0000000040221400,
            factor: 0x0000806004000000,
            offset: 6587,
        },
        Magic {
            mask: 0x0000000002442800,
            factor: 0x0000100400000000,
            offset: 6491,
        },
        Magic {
            mask: 0x0000000204085000,
            factor: 0x000021c100b20000,
            offset: 6330,
        },
        Magic {
            mask: 0x0000020408102000,
            factor: 0x0000040041008000,
            offset: 5609,
        },
        Magic {
            mask: 0x0002040810204000,
            factor: 0x00000fb0203fff80,
            offset: 22236,
        },
        Magic {
            mask: 0x0020100804020000,
            factor: 0x0000040100401004,
            offset: 6106,
        },
        Magic {
            mask: 0x0040201008040000,
            factor: 0x0000020080200802,
            offset: 5625,
        },
        Magic {
            mask: 0x00004020100a0000,
            factor: 0x0000004010202000,
            offset: 16785,
        },
        Magic {
            mask: 0x0000004022140000,
            factor: 0x0000008060040000,
            offset: 16817,
        },
        Magic {
            mask: 0x0000000244280000,
            factor: 0x0000004402000000,
            offset: 6842,
        },
        Magic {
            mask: 0x0000020408500000,
            factor: 0x0000000801008000,
            offset: 7003,
        },
        Magic {
            mask: 0x0002040810200000,
            factor: 0x000007efe0bfff80,
            offset: 4197,
        },
        Magic {
            mask: 0x0004081020400000,
            factor: 0x0000000820820020,
            offset: 7356,
        },
        Magic {
            mask: 0x0010080402000200,
            factor: 0x0000400080808080,
            offset: 4602,
        },
        Magic {
            mask: 0x0020100804000400,
            factor: 0x00021f0100400808,
            offset: 4538,
        },
        Magic {
            mask: 0x004020100a000a00,
            factor: 0x00018000c06f3fff,
            offset: 29531,
        },
        Magic {
            mask: 0x0000402214001400,
            factor: 0x0000258200801000,
            offset: 45393,
        },
        Magic {
            mask: 0x0000024428002800,
            factor: 0x0000240080840000,
            offset: 12420,
        },
        Magic {
            mask: 0x0002040850005000,
            factor: 0x000018000c03fff8,
            offset: 15763,
        },
        Magic {
            mask: 0x0004081020002000,
            factor: 0x00000a5840208020,
            offset: 5050,
        },
        Magic {
            mask: 0x0008102040004000,
            factor: 0x0000020008208020,
            offset: 4346,
        },
        Magic {
            mask: 0x0008040200020400,
            factor: 0x0000804000810100,
            offset: 6074,
        },
        Magic {
            mask: 0x0010080400040800,
            factor: 0x0001011900802008,
            offset: 7866,
        },
        Magic {
            mask: 0x0020100a000a1000,
            factor: 0x0000804000810100,
            offset: 32139,
        },
        Magic {
            mask: 0x0040221400142200,
            factor: 0x000100403c0403ff,
            offset: 57673,
        },
        Magic {
            mask: 0x0002442800284400,
            factor: 0x00078402a8802000,
            offset: 55365,
        },
        Magic {
            mask: 0x0004085000500800,
            factor: 0x0000101000804400,
            offset: 15818,
        },
        Magic {
            mask: 0x0008102000201000,
            factor: 0x0000080800104100,
            offset: 5562,
        },
        Magic {
            mask: 0x0010204000402000,
            factor: 0x00004004c0082008,
            offset: 6390,
        },
        Magic {
            mask: 0x0004020002040800,
            factor: 0x0001010120008020,
            offset: 7930,
        },
        Magic {
            mask: 0x0008040004081000,
            factor: 0x000080809a004010,
            offset: 13329,
        },
        Magic {
            mask: 0x00100a000a102000,
            factor: 0x0007fefe08810010,
            offset: 7170,
        },
        Magic {
            mask: 0x0022140014224000,
            factor: 0x0003ff0f833fc080,
            offset: 27267,
        },
        Magic {
            mask: 0x0044280028440200,
            factor: 0x007fe08019003042,
            offset: 53787,
        },
        Magic {
            mask: 0x0008500050080400,
            factor: 0x003fffefea003000,
            offset: 5097,
        },
        Magic {
            mask: 0x0010200020100800,
            factor: 0x0000101010002080,
            offset: 6643,
        },
        Magic {
            mask: 0x0020400040201000,
            factor: 0x0000802005080804,
            offset: 6138,
        },
        Magic {
            mask: 0x0002000204081000,
            factor: 0x0000808080a80040,
            offset: 7418,
        },
        Magic {
            mask: 0x0004000408102000,
            factor: 0x0000104100200040,
            offset: 7898,
        },
        Magic {
            mask: 0x000a000a10204000,
            factor: 0x0003ffdf7f833fc0,
            offset: 42012,
        },
        Magic {
            mask: 0x0014001422400000,
            factor: 0x0000008840450020,
            offset: 57350,
        },
        Magic {
            mask: 0x0028002844020000,
            factor: 0x00007ffc80180030,
            offset: 22813,
        },
        Magic {
            mask: 0x0050005008040200,
            factor: 0x007fffdd80140028,
            offset: 56693,
        },
        Magic {
            mask: 0x0020002010080400,
            factor: 0x00020080200a0004,
            offset: 5818,
        },
        Magic {
            mask: 0x0040004020100800,
            factor: 0x0000101010100020,
            offset: 7098,
        },
        Magic {
            mask: 0x0000020408102000,
            factor: 0x0007ffdfc1805000,
            offset: 4451,
        },
        Magic {
            mask: 0x0000040810204000,
            factor: 0x0003ffefe0c02200,
            offset: 4709,
        },
        Magic {
            mask: 0x00000a1020400000,
            factor: 0x0000000820806000,
            offset: 4794,
        },
        Magic {
            mask: 0x0000142240000000,
            factor: 0x0000000008403000,
            offset: 13364,
        },
        Magic {
            mask: 0x0000284402000000,
            factor: 0x0000000100202000,
            offset: 4570,
        },
        Magic {
            mask: 0x0000500804020000,
            factor: 0x0000004040802000,
            offset: 4282,
        },
        Magic {
            mask: 0x0000201008040200,
            factor: 0x0004010040100400,
            offset: 14964,
        },
        Magic {
            mask: 0x0000402010080400,
            factor: 0x00006020601803f4,
            offset: 4026,
        },
        Magic {
            mask: 0x0002040810204000,
            factor: 0x0003ffdfdfc28048,
            offset: 4826,
        },
        Magic {
            mask: 0x0004081020400000,
            factor: 0x0000000820820020,
            offset: 7354,
        },
        Magic {
            mask: 0x000a102040000000,
            factor: 0x0000000008208060,
            offset: 4848,
        },
        Magic {
            mask: 0x0014224000000000,
            factor: 0x0000000000808020,
            offset: 15946,
        },
        Magic {
            mask: 0x0028440200000000,
            factor: 0x0000000001002020,
            offset: 14932,
        },
        Magic {
            mask: 0x0050080402000000,
            factor: 0x0000000401002008,
            offset: 16588,
        },
        Magic {
            mask: 0x0020100804020000,
            factor: 0x0000004040404040,
            offset: 6905,
        },
        Magic {
            mask: 0x0040201008040200,
            factor: 0x007fff9fdf7ff813,
            offset: 16076,
        },
    ];
}

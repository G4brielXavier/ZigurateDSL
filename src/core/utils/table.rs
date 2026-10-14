pub struct Table {

    pub kwd_functions: [&'static [u8]; 6],
    pub kwd_modificators: [&'static [u8]; 1],
    pub kwd_variable:  [&'static [u8]; 1],
    pub kwd_simple:    [&'static [u8]; 2],
    pub kwd_special:    [&'static [u8]; 2],
    pub symb:           &'static [u8; 10],
    pub kwd_lunig:      &'static [u8],
    pub kwd_anki:       &'static [u8],
    pub kwd_maru:       &'static [u8],
    pub kwd_gish:       &'static [u8],

}

impl Table {
    pub fn get() -> Self {
        Self {
            kwd_functions: [
                b"silim",
                b"kud",
                b"gaz",
                b"push",
                b"pop",
                b"clear"
            ],
            kwd_modificators: [
                b"nisa"
            ],
            kwd_variable: [
                b"uda",
            ],
            kwd_simple: [
                b"as",
                b"ak"
            ],
            kwd_special: [
                b"true",
                b"false"
            ],
            kwd_lunig: b"lunig",
            kwd_anki: b"anki",
            kwd_maru: b"maru",
            kwd_gish: b"gish",
            symb: b"@()[]=.:,-"
        }
    }
}
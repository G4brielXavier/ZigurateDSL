pub struct Table {

    pub kwd_functions: [&'static [u8]; 7],
    pub kwd_modificators: [&'static [u8]; 1],
    pub kwd_variable:  [&'static [u8]; 1],
    pub kwd_simple:    [&'static [u8]; 5],
    pub kwd_special:    [&'static [u8]; 2],
    pub kwd_comparation_operators:    [&'static [u8]; 8],
    pub symb:           &'static [u8; 12],
    pub kwd_lunig:      &'static [u8],
    pub kwd_anki:       &'static [u8],
    pub kwd_maru:       &'static [u8],
    pub kwd_suma:       &'static [u8],
    pub kwd_gish:       &'static [u8],

}

impl Table {
    pub fn get() -> Self {
        Self {
            kwd_functions: [
                b"silim",
                b"kud",
                b"gaz",
                b"dumu",
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
                b"ak",
                b"in",
                b"out",
                b"li"
            ],
            kwd_special: [
                b"true",
                b"false"
            ],
            kwd_comparation_operators: [
                b"and",
                b"or",
                b"is",
                b"is_not",
                b"is_more",
                b"is_less",
                b"is_more_eq",
                b"is_less_eq",
            ],
            kwd_lunig: b"lunig",
            kwd_suma: b"suma",
            kwd_anki: b"anki",
            kwd_maru: b"maru",
            kwd_gish: b"gish",
            symb: b"@()[]{}=.:,-"
        }
    }
}
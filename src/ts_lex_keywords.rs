unsafe extern "C" fn ts_lex_keywords(mut lexer: *mut TSLexer, mut state: TSStateId) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut skip: bool = 0 as libc::c_int != 0;
    let mut eof: bool = 0 as libc::c_int != 0;
    let mut lookahead: int32_t = 0;
    loop {
        skip = 0 as libc::c_int != 0;
        lookahead = (*lexer).lookahead;
        eof = (*lexer).eof.expect("non-null function pointer")(lexer);
        match state as libc::c_int {
            0 => {
                if lookahead == '_' as i32 {
                    state = 1 as libc::c_int as TSStateId
                } else if lookahead == 'a' as i32 {
                    state = 2 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 3 as libc::c_int as TSStateId
                } else if lookahead == 'c' as i32 {
                    state = 4 as libc::c_int as TSStateId
                } else if lookahead == 'd' as i32 {
                    state = 5 as libc::c_int as TSStateId
                } else if lookahead == 'e' as i32 {
                    state = 6 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 7 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 8 as libc::c_int as TSStateId
                } else if lookahead == 'l' as i32 {
                    state = 9 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 10 as libc::c_int as TSStateId
                } else if lookahead == 'p' as i32 {
                    state = 11 as libc::c_int as TSStateId
                } else if lookahead == 'r' as i32 {
                    state = 12 as libc::c_int as TSStateId
                } else if lookahead == 's' as i32 {
                    state = 13 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 14 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 15 as libc::c_int as TSStateId
                } else if lookahead == 'v' as i32 {
                    state = 16 as libc::c_int as TSStateId
                } else if lookahead == 'w' as i32 {
                    state = 17 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 0 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            1 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym__ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            2 => {
                if lookahead == 's' as i32 {
                    state = 18 as libc::c_int as TSStateId
                } else if lookahead == 'w' as i32 {
                    state = 19 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            3 => {
                if lookahead == 'l' as i32 {
                    state = 20 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 21 as libc::c_int as TSStateId
                } else if lookahead == 'r' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            4 => {
                if lookahead == 'h' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'r' as i32 {
                    state = 25 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            5 => {
                if lookahead == 'e' as i32 {
                    state = 26 as libc::c_int as TSStateId
                } else if lookahead == 'y' as i32 {
                    state = 27 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            6 => {
                if lookahead == 'l' as i32 {
                    state = 28 as libc::c_int as TSStateId
                } else if lookahead == 'n' as i32 {
                    state = 29 as libc::c_int as TSStateId
                } else if lookahead == 'x' as i32 {
                    state = 30 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            7 => {
                if lookahead == '3' as i32 {
                    state = 31 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 32 as libc::c_int as TSStateId
                } else if lookahead == 'a' as i32 {
                    state = 33 as libc::c_int as TSStateId
                } else if lookahead == 'n' as i32 {
                    state = 34 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 35 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            8 => {
                if lookahead == '1' as i32 {
                    state = 36 as libc::c_int as TSStateId
                } else if lookahead == '3' as i32 {
                    state = 37 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 38 as libc::c_int as TSStateId
                } else if lookahead == '8' as i32 {
                    state = 39 as libc::c_int as TSStateId
                } else if lookahead == 'd' as i32 {
                    state = 40 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 41 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 42 as libc::c_int as TSStateId
                } else if lookahead == 'n' as i32 {
                    state = 43 as libc::c_int as TSStateId
                } else if lookahead == 's' as i32 {
                    state = 44 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 45 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            9 => {
                if lookahead == 'e' as i32 {
                    state = 46 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 47 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 48 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            10 => {
                if lookahead == 'a' as i32 {
                    state = 49 as libc::c_int as TSStateId
                } else if lookahead == 'e' as i32 {
                    state = 50 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 51 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 52 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            11 => {
                if lookahead == 'a' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 54 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            12 => {
                if lookahead == 'e' as i32 {
                    state = 55 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            13 => {
                if lookahead == 'e' as i32 {
                    state = 56 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 57 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            14 => {
                if lookahead == 'r' as i32 {
                    state = 59 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == 'y' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            15 => {
                if lookahead == '1' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '3' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 64 as libc::c_int as TSStateId
                } else if lookahead == '8' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == 'n' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == 's' as i32 {
                    state = 67 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            16 => {
                if lookahead == 'i' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            17 => {
                if lookahead == 'h' as i32 {
                    state = 69 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            18 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_as as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'y' as i32 {
                    state = 70 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            19 => {
                if lookahead == 'a' as i32 {
                    state = 71 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            20 => {
                if lookahead == 'o' as i32 {
                    state = 72 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            21 => {
                if lookahead == 'o' as i32 {
                    state = 73 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            22 => {
                if lookahead == 'e' as i32 {
                    state = 74 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            23 => {
                if lookahead == 'a' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            24 => {
                if lookahead == 'n' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            25 => {
                if lookahead == 'a' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            26 => {
                if lookahead == 'f' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            27 => {
                if lookahead == 'n' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            28 => {
                if lookahead == 's' as i32 {
                    state = 80 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            29 => {
                if lookahead == 'u' as i32 {
                    state = 81 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            30 => {
                if lookahead == 'p' as i32 {
                    state = 82 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 83 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            31 => {
                if lookahead == '2' as i32 {
                    state = 84 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            32 => {
                if lookahead == '4' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            33 => {
                if lookahead == 'l' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            34 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_fn as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            35 => {
                if lookahead == 'r' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            36 => {
                if lookahead == '2' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 89 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            37 => {
                if lookahead == '2' as i32 {
                    state = 90 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            38 => {
                if lookahead == '4' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            39 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_i8 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            40 => {
                if lookahead == 'e' as i32 {
                    state = 92 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            41 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_if as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            42 => {
                if lookahead == 'p' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            43 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_in as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            44 => {
                if lookahead == 'i' as i32 {
                    state = 94 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            45 => {
                if lookahead == 'e' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            46 => {
                if lookahead == 't' as i32 {
                    state = 96 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            47 => {
                if lookahead == 'f' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            48 => {
                if lookahead == 'o' as i32 {
                    state = 99 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            49 => {
                if lookahead == 't' as i32 {
                    state = 100 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            50 => {
                if lookahead == 't' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            51 => {
                if lookahead == 'd' as i32 {
                    state = 102 as libc::c_int as TSStateId
                } else if lookahead == 'v' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            52 => {
                if lookahead == 't' as i32 {
                    state = 104 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            53 => {
                if lookahead == 't' as i32 {
                    state = 105 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            54 => {
                if lookahead == 'b' as i32 {
                    state = 106 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            55 => {
                if lookahead == 'f' as i32 {
                    state = 107 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 108 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            56 => {
                if lookahead == 'l' as i32 {
                    state = 109 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            57 => {
                if lookahead == 'a' as i32 {
                    state = 110 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 111 as libc::c_int as TSStateId
                } else if lookahead == 'r' as i32 {
                    state = 112 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            58 => {
                if lookahead == 'p' as i32 {
                    state = 113 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            59 => {
                if lookahead == 'a' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            60 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_tt as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            61 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_ty as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'p' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            62 => {
                if lookahead == '2' as i32 {
                    state = 117 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 118 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            63 => {
                if lookahead == '2' as i32 {
                    state = 119 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            64 => {
                if lookahead == '4' as i32 {
                    state = 120 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            65 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_u8 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            66 => {
                if lookahead == 'i' as i32 {
                    state = 121 as libc::c_int as TSStateId
                } else if lookahead == 's' as i32 {
                    state = 122 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            67 => {
                if lookahead == 'e' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            68 => {
                if lookahead == 's' as i32 {
                    state = 125 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            69 => {
                if lookahead == 'e' as i32 {
                    state = 126 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 127 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            70 => {
                if lookahead == 'n' as i32 {
                    state = 128 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            71 => {
                if lookahead == 'i' as i32 {
                    state = 129 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            72 => {
                if lookahead == 'c' as i32 {
                    state = 130 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            73 => {
                if lookahead == 'l' as i32 {
                    state = 131 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            74 => {
                if lookahead == 'a' as i32 {
                    state = 132 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            75 => {
                if lookahead == 'r' as i32 {
                    state = 133 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            76 => {
                if lookahead == 's' as i32 {
                    state = 134 as libc::c_int as TSStateId
                } else if lookahead == 't' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            77 => {
                if lookahead == 't' as i32 {
                    state = 136 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            78 => {
                if lookahead == 'a' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            79 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_dyn as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            80 => {
                if lookahead == 'e' as i32 {
                    state = 138 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            81 => {
                if lookahead == 'm' as i32 {
                    state = 139 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            82 => {
                if lookahead == 'r' as i32 {
                    state = 140 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            83 => {
                if lookahead == 'e' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            84 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_f32 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            85 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_f64 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            86 => {
                if lookahead == 's' as i32 {
                    state = 142 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            87 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_for as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            88 => {
                if lookahead == '8' as i32 {
                    state = 143 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            89 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_i16 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            90 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_i32 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            91 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_i64 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            92 => {
                if lookahead == 'n' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            93 => {
                if lookahead == 'l' as i32 {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            94 => {
                if lookahead == 'z' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            95 => {
                if lookahead == 'm' as i32 {
                    state = 147 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            96 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_let as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            97 => {
                if lookahead == 'e' as i32 {
                    state = 148 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            98 => {
                if lookahead == 'e' as i32 {
                    state = 149 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            99 => {
                if lookahead == 'p' as i32 {
                    state = 150 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            100 => {
                if lookahead == 'c' as i32 {
                    state = 151 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            101 => {
                if lookahead == 'a' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            102 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_mod as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            103 => {
                if lookahead == 'e' as i32 {
                    state = 153 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            104 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_mutable_specifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            105 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_pat as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'h' as i32 {
                    state = 154 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            106 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_pub as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            107 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_ref as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            108 => {
                if lookahead == 'u' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            109 => {
                if lookahead == 'f' as i32 {
                    state = 156 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            110 => {
                if lookahead == 't' as i32 {
                    state = 157 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            111 => {
                if lookahead == 't' as i32 {
                    state = 158 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            112 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_str as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'u' as i32 {
                    state = 159 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            113 => {
                if lookahead == 'e' as i32 {
                    state = 160 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            114 => {
                if lookahead == 'i' as i32 {
                    state = 161 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            115 => {
                if lookahead == 'e' as i32 {
                    state = 162 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            116 => {
                if lookahead == 'e' as i32 {
                    state = 163 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            117 => {
                if lookahead == '8' as i32 {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            118 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_u16 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            119 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_u32 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            120 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_u64 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            121 => {
                if lookahead == 'o' as i32 {
                    state = 165 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            122 => {
                if lookahead == 'a' as i32 {
                    state = 166 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            123 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_use as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            124 => {
                if lookahead == 'z' as i32 {
                    state = 167 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            125 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_vis as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            126 => {
                if lookahead == 'r' as i32 {
                    state = 168 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            127 => {
                if lookahead == 'l' as i32 {
                    state = 169 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            128 => {
                if lookahead == 'c' as i32 {
                    state = 170 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            129 => {
                if lookahead == 't' as i32 {
                    state = 171 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            130 => {
                if lookahead == 'k' as i32 {
                    state = 172 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            131 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_bool as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            132 => {
                if lookahead == 'k' as i32 {
                    state = 173 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            133 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_char as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            134 => {
                if lookahead == 't' as i32 {
                    state = 174 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            135 => {
                if lookahead == 'i' as i32 {
                    state = 175 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            136 => {
                if lookahead == 'e' as i32 {
                    state = 176 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            137 => {
                if lookahead == 'u' as i32 {
                    state = 177 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            138 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_else as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            139 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_enum as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            140 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_expr as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            141 => {
                if lookahead == 'r' as i32 {
                    state = 178 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            142 => {
                if lookahead == 'e' as i32 {
                    state = 179 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            143 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_i128 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            144 => {
                if lookahead == 't' as i32 {
                    state = 180 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            145 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_impl as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            146 => {
                if lookahead == 'e' as i32 {
                    state = 181 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            147 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_item as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            148 => {
                if lookahead == 't' as i32 {
                    state = 182 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            149 => {
                if lookahead == 'r' as i32 {
                    state = 183 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            150 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_loop as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            151 => {
                if lookahead == 'h' as i32 {
                    state = 184 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            152 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_meta as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            153 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_move as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            154 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_path as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            155 => {
                if lookahead == 'r' as i32 {
                    state = 185 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            156 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_self as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            157 => {
                if lookahead == 'i' as i32 {
                    state = 186 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            158 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_stmt as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            159 => {
                if lookahead == 'c' as i32 {
                    state = 187 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            160 => {
                if lookahead == 'r' as i32 {
                    state = 188 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            161 => {
                if lookahead == 't' as i32 {
                    state = 189 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            162 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_true as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            163 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_type as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            164 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_u128 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            165 => {
                if lookahead == 'n' as i32 {
                    state = 190 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            166 => {
                if lookahead == 'f' as i32 {
                    state = 191 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            167 => {
                if lookahead == 'e' as i32 {
                    state = 192 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            168 => {
                if lookahead == 'e' as i32 {
                    state = 193 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            169 => {
                if lookahead == 'e' as i32 {
                    state = 194 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            170 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_async as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            171 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_await as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            172 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_block as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            173 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_break as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            174 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_const as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            175 => {
                if lookahead == 'n' as i32 {
                    state = 195 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            176 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_crate as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            177 => {
                if lookahead == 'l' as i32 {
                    state = 196 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            178 => {
                if lookahead == 'n' as i32 {
                    state = 197 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            179 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_false as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            180 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_ident as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            181 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_isize as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            182 => {
                if lookahead == 'i' as i32 {
                    state = 198 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            183 => {
                if lookahead == 'a' as i32 {
                    state = 199 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            184 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_match as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            185 => {
                if lookahead == 'n' as i32 {
                    state = 200 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            186 => {
                if lookahead == 'c' as i32 {
                    state = 201 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            187 => {
                if lookahead == 't' as i32 {
                    state = 202 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            188 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_super as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            189 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_trait as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            190 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_union as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            191 => {
                if lookahead == 'e' as i32 {
                    state = 203 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            192 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_usize as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            193 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_where as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            194 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_while as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            195 => {
                if lookahead == 'u' as i32 {
                    state = 204 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            196 => {
                if lookahead == 't' as i32 {
                    state = 205 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            197 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_extern as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            198 => {
                if lookahead == 'm' as i32 {
                    state = 206 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            199 => {
                if lookahead == 'l' as i32 {
                    state = 207 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            200 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_return as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            201 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_static as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            202 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_struct as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            203 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_unsafe as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            204 => {
                if lookahead == 'e' as i32 {
                    state = 208 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            205 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_default as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            206 => {
                if lookahead == 'e' as i32 {
                    state = 209 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            207 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            208 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_continue as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            209 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_lifetime as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            _ => return 0 as libc::c_int != 0,
        }
        (*lexer).advance.expect("non-null function pointer")(lexer, skip);
    }
}


unsafe extern "C" fn ts_lex(mut lexer: *mut TSLexer, mut state: TSStateId) -> bool {
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
                if eof {
                    state = 57 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 147 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 70 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 110 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '@' as i32 {
                    state = 136 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == '\\' as i32 {
                    state = 35 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 54 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            1 => {
                if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 109 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 96 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 1 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            2 => {
                if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 84 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 110 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 96 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 2 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            3 => {
                if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 109 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '@' as i32 {
                    state = 136 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 3 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            4 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 147 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 84 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 89 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == '\\' as i32 {
                    state = 35 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 8 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            5 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 100 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 111 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 20 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 30 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 5 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            6 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 100 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 108 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 21 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 30 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 6 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            7 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 100 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 84 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 32 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 92 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 7 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            8 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 84 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 89 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 8 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            9 => {
                if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 92 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == '@' as i32 {
                    state = 136 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 9 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            10 => {
                if lookahead == '!' as i32 {
                    state = 31 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 109 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 67 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 96 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 10 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            11 => {
                if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 70 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 80 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 69 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '_' as i32 {
                    state = 81 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 11 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if '!' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || '|' as i32 <= lookahead && lookahead <= '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            12 => {
                if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 70 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 80 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '_' as i32 {
                    state = 81 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 12 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if '!' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || '|' as i32 <= lookahead && lookahead <= '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            13 => {
                if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 67 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 89 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 153 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 13 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            14 => {
                if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 20 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 67 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 14 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            15 => {
                if lookahead == '\'' as i32 {
                    state = 148 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            16 => {
                if lookahead == '\'' as i32 {
                    state = 148 as libc::c_int as TSStateId
                } else if lookahead == '\\' as i32 {
                    state = 36 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 15 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            17 => {
                if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 32 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 22 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 67 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 92 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 17 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            18 => {
                if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 73 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    state = 72 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 74 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            19 => {
                if lookahead == '.' as i32 {
                    state = 102 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 107 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            20 => {
                if lookahead == '.' as i32 {
                    state = 104 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            21 => {
                if lookahead == '.' as i32 {
                    state = 105 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            22 => {
                if lookahead == '.' as i32 {
                    state = 19 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            23 => {
                if lookahead == '/' as i32 {
                    state = 150 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            24 => {
                if lookahead == '1' as i32 {
                    state = 26 as libc::c_int as TSStateId
                } else if lookahead == '3' as i32 {
                    state = 25 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 28 as libc::c_int as TSStateId
                } else if lookahead == '8' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else if lookahead == 's' as i32 {
                    state = 34 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            25 => {
                if lookahead == '2' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            26 => {
                if lookahead == '2' as i32 {
                    state = 29 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            27 => {
                if lookahead == '3' as i32 {
                    state = 25 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 28 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            28 => {
                if lookahead == '4' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            29 => {
                if lookahead == '8' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            30 => {
                if lookahead == ':' as i32 {
                    state = 99 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            31 => {
                if lookahead == '=' as i32 {
                    state = 118 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            32 => {
                if lookahead == '>' as i32 {
                    state = 94 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            33 => {
                if lookahead == 'e' as i32 {
                    state = 137 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            34 => {
                if lookahead == 'i' as i32 {
                    state = 37 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            35 => {
                if lookahead == 'u' as i32 {
                    state = 38 as libc::c_int as TSStateId
                } else if lookahead == 'x' as i32 {
                    state = 49 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 149 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            36 => {
                if lookahead == 'u' as i32 {
                    state = 39 as libc::c_int as TSStateId
                } else if lookahead == 'x' as i32 {
                    state = 50 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 15 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            37 => {
                if lookahead == 'z' as i32 {
                    state = 33 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            38 => {
                if lookahead == '{' as i32 {
                    state = 47 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 45 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            39 => {
                if lookahead == '{' as i32 {
                    state = 48 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 51 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            40 => {
                if lookahead == '}' as i32 {
                    state = 15 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 40 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            41 => {
                if lookahead == '}' as i32 {
                    state = 149 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 41 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            42 => {
                if lookahead == '0' as i32 || lookahead == '1' as i32 || lookahead == '_' as i32 {
                    state = 142 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            43 => {
                if '0' as i32 <= lookahead && lookahead <= '7' as i32 || lookahead == '_' as i32 {
                    state = 143 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            44 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 15 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            45 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 49 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            46 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 149 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            47 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 41 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            48 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 40 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            49 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 46 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            50 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 44 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            51 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 50 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            52 => {
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'f' as i32
                {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            53 => {
                if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                {
                    state = 165 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            54 => {
                if eof {
                    state = 57 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 70 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 110 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 68 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 103 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 91 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '@' as i32 {
                    state = 136 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 54 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            55 => {
                if eof {
                    state = 57 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32 {
                    state = 88 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '%' as i32 {
                    state = 124 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 101 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 78 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 76 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 109 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 135 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 123 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 30 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 96 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 90 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 98 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == '^' as i32 {
                    state = 116 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 115 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 55 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            56 => {
                if eof {
                    state = 57 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32 {
                    state = 87 as libc::c_int as TSStateId
                } else if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '#' as i32 {
                    state = 86 as libc::c_int as TSStateId
                } else if lookahead == '$' as i32 {
                    state = 53 as libc::c_int as TSStateId
                } else if lookahead == '&' as i32 {
                    state = 100 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 85 as libc::c_int as TSStateId
                } else if lookahead == '(' as i32 {
                    state = 60 as libc::c_int as TSStateId
                } else if lookahead == ')' as i32 {
                    state = 61 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32 {
                    state = 77 as libc::c_int as TSStateId
                } else if lookahead == '+' as i32 {
                    state = 75 as libc::c_int as TSStateId
                } else if lookahead == ',' as i32 {
                    state = 93 as libc::c_int as TSStateId
                } else if lookahead == '-' as i32 {
                    state = 111 as libc::c_int as TSStateId
                } else if lookahead == '.' as i32 {
                    state = 20 as libc::c_int as TSStateId
                } else if lookahead == '/' as i32 {
                    state = 23 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32 {
                    state = 141 as libc::c_int as TSStateId
                } else if lookahead == ':' as i32 {
                    state = 30 as libc::c_int as TSStateId
                } else if lookahead == ';' as i32 {
                    state = 58 as libc::c_int as TSStateId
                } else if lookahead == '<' as i32 {
                    state = 95 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 89 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 97 as libc::c_int as TSStateId
                } else if lookahead == '?' as i32 {
                    state = 79 as libc::c_int as TSStateId
                } else if lookahead == '[' as i32 {
                    state = 65 as libc::c_int as TSStateId
                } else if lookahead == ']' as i32 {
                    state = 66 as libc::c_int as TSStateId
                } else if lookahead == 'b' as i32 {
                    state = 152 as libc::c_int as TSStateId
                } else if lookahead == 'm' as i32 {
                    state = 155 as libc::c_int as TSStateId
                } else if lookahead == '{' as i32 {
                    state = 62 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 114 as libc::c_int as TSStateId
                } else if lookahead == '}' as i32 {
                    state = 63 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    skip = 1 as libc::c_int != 0;
                    state = 56 as libc::c_int as TSStateId
                } else if '1' as i32 <= lookahead && lookahead <= '9' as i32 {
                    state = 144 as libc::c_int as TSStateId
                } else if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            57 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = 0 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            58 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_SEMI as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            59 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_macro_rules_BANG as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            60 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LPAREN as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            61 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_RPAREN as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            62 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LBRACE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            63 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_RBRACE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            64 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ_GT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            65 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LBRACK as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            66 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_RBRACK as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            67 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_COLON as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            68 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_COLON as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == ':' as i32 {
                    state = 99 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            69 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_COLON as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '!' as i32
                    || lookahead == '#' as i32
                    || lookahead == '%' as i32
                    || lookahead == '&' as i32
                    || '*' as i32 <= lookahead && lookahead <= '/' as i32
                    || ':' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || lookahead == '_' as i32
                    || lookahead == '|' as i32
                    || lookahead == '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            70 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOLLAR as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                {
                    state = 165 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            71 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym_token_repetition_pattern_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '\n' as i32 {
                    state = 74 as libc::c_int as TSStateId
                } else if lookahead == '*' as i32
                    || lookahead == '+' as i32
                    || lookahead == '?' as i32
                {
                    state = 150 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 71 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            72 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym_token_repetition_pattern_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '/' as i32 {
                    state = 73 as libc::c_int as TSStateId
                } else if lookahead == '\t' as i32
                    || lookahead == '\n' as i32
                    || lookahead == '\r' as i32
                    || lookahead == ' ' as i32
                {
                    state = 72 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int
                    && lookahead != '*' as i32
                    && lookahead != '+' as i32
                    && lookahead != '?' as i32
                {
                    state = 74 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            73 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym_token_repetition_pattern_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '/' as i32 {
                    state = 71 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int
                    && lookahead != '*' as i32
                    && lookahead != '+' as i32
                    && lookahead != '?' as i32
                {
                    state = 74 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            74 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym_token_repetition_pattern_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead != 0 as libc::c_int
                    && lookahead != '*' as i32
                    && lookahead != '+' as i32
                    && lookahead != '?' as i32
                {
                    state = 74 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            75 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PLUS as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            76 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PLUS as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 125 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            77 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_STAR as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            78 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_STAR as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 127 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            79 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_QMARK as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            80 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym__non_special_token_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '/' as i32 {
                    state = 83 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32
                    || lookahead == '#' as i32
                    || lookahead == '%' as i32
                    || lookahead == '&' as i32
                    || '*' as i32 <= lookahead && lookahead <= '.' as i32
                    || ':' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || lookahead == '_' as i32
                    || lookahead == '|' as i32
                    || lookahead == '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            81 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym__non_special_token_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '_' as i32 {
                    state = 81 as libc::c_int as TSStateId
                } else if lookahead == '!' as i32
                    || lookahead == '#' as i32
                    || lookahead == '%' as i32
                    || lookahead == '&' as i32
                    || '*' as i32 <= lookahead && lookahead <= '/' as i32
                    || ':' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || lookahead == '|' as i32
                    || lookahead == '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            82 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym__non_special_token_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '!' as i32
                    || lookahead == '#' as i32
                    || lookahead == '%' as i32
                    || lookahead == '&' as i32
                    || '*' as i32 <= lookahead && lookahead <= '/' as i32
                    || ':' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || lookahead == '_' as i32
                    || lookahead == '|' as i32
                    || lookahead == '~' as i32
                {
                    state = 82 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            83 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol =
                    aux_sym__non_special_token_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '!' as i32
                    || lookahead == '#' as i32
                    || lookahead == '%' as i32
                    || lookahead == '&' as i32
                    || '*' as i32 <= lookahead && lookahead <= '/' as i32
                    || ':' as i32 <= lookahead && lookahead <= '@' as i32
                    || lookahead == '^' as i32
                    || lookahead == '_' as i32
                    || lookahead == '|' as i32
                    || lookahead == '~' as i32
                {
                    state = 83 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int && lookahead != '\n' as i32 {
                    state = 150 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            84 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_SQUOTE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            85 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_SQUOTE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '\'' as i32 {
                    state = 148 as libc::c_int as TSStateId
                } else if lookahead == '\\' as i32 {
                    state = 36 as libc::c_int as TSStateId
                } else if lookahead != 0 as libc::c_int {
                    state = 15 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            86 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_POUND as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            87 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_BANG as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            88 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_BANG as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 118 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            89 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            90 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 117 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            91 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 117 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 64 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            92 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '>' as i32 {
                    state = 64 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            93 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_COMMA as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            94 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH_GT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            95 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            96 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '<' as i32 {
                    state = 121 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 119 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            97 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_GT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            98 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_GT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 120 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 122 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            99 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_COLON_COLON as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            100 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_AMP as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            101 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_AMP as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '&' as i32 {
                    state = 112 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 130 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            102 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT_DOT_DOT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            103 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT2 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            104 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT_DOT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            105 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT_DOT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '.' as i32 {
                    state = 102 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            106 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT_DOT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '.' as i32 {
                    state = 102 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 107 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            107 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT_DOT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            108 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            109 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 126 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            110 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 126 as libc::c_int as TSStateId
                } else if lookahead == '>' as i32 {
                    state = 94 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            111 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '>' as i32 {
                    state = 94 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            112 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_AMP_AMP as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            113 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PIPE_PIPE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            114 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PIPE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            115 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PIPE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 131 as libc::c_int as TSStateId
                } else if lookahead == '|' as i32 {
                    state = 113 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            116 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_CARET as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 132 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            117 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_EQ_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            118 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_BANG_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            119 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            120 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_GT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            121 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT_LT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 133 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            122 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_GT_GT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 134 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            123 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_SLASH as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '/' as i32 {
                    state = 150 as libc::c_int as TSStateId
                } else if lookahead == '=' as i32 {
                    state = 128 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            124 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PERCENT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '=' as i32 {
                    state = 129 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            125 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PLUS_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            126 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DASH_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            127 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_STAR_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            128 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_SLASH_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            129 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PERCENT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            130 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_AMP_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            131 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_PIPE_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            132 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_CARET_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            133 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_LT_LT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            134 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_GT_GT_EQ as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            135 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DOT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '.' as i32 {
                    state = 106 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            136 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_AT as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            137 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            138 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '2' as i32 {
                    state = 145 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 139 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'e' as i32
                {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            139 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '3' as i32 {
                    state = 138 as libc::c_int as TSStateId
                } else if lookahead == '6' as i32 {
                    state = 140 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 139 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'e' as i32
                {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            140 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '4' as i32 {
                    state = 145 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 139 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'e' as i32
                {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            141 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'b' as i32 {
                    state = 42 as libc::c_int as TSStateId
                } else if lookahead == 'f' as i32 {
                    state = 27 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'o' as i32 {
                    state = 43 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'x' as i32 {
                    state = 52 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || lookahead == '_' as i32
                {
                    state = 144 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            142 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'f' as i32 {
                    state = 27 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == '0' as i32
                    || lookahead == '1' as i32
                    || lookahead == '_' as i32
                {
                    state = 142 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            143 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'f' as i32 {
                    state = 27 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '7' as i32
                    || lookahead == '_' as i32
                {
                    state = 143 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            144 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'f' as i32 {
                    state = 27 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || lookahead == '_' as i32
                {
                    state = 144 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            145 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_integer_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'f' as i32 {
                    state = 139 as libc::c_int as TSStateId
                } else if lookahead == 'i' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if lookahead == 'u' as i32 {
                    state = 24 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'F' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'e' as i32
                {
                    state = 145 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            146 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = aux_sym_string_literal_token1 as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            147 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = anon_sym_DQUOTE as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            148 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_char_literal as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            149 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_escape_sequence as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                return result;
            }
            150 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_line_comment as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead != 0 as libc::c_int && lookahead != '\n' as i32 {
                    state = 150 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            151 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '!' as i32 {
                    state = 59 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            152 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if lookahead == '\'' as i32 {
                    state = 16 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            153 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '\"' as i32 {
                    state = 146 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            154 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == '_' as i32 {
                    state = 161 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            155 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'a' as i32 {
                    state = 156 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'b' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            156 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'c' as i32 {
                    state = 160 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            157 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'e' as i32 {
                    state = 162 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            158 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'l' as i32 {
                    state = 157 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            159 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'o' as i32 {
                    state = 154 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            160 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'r' as i32 {
                    state = 159 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            161 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'r' as i32 {
                    state = 163 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            162 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 's' as i32 {
                    state = 151 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            163 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if lookahead == 'u' as i32 {
                    state = 158 as libc::c_int as TSStateId
                } else if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            164 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_identifier as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                    || lookahead == 181 as libc::c_int
                    || 913 as libc::c_int <= lookahead && lookahead <= 937 as libc::c_int
                    || 945 as libc::c_int <= lookahead && lookahead <= 969 as libc::c_int
                {
                    state = 164 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            165 => {
                result = 1 as libc::c_int != 0;
                (*lexer).result_symbol = sym_metavariable as libc::c_int as TSSymbol;
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
                if '0' as i32 <= lookahead && lookahead <= '9' as i32
                    || 'A' as i32 <= lookahead && lookahead <= 'Z' as i32
                    || lookahead == '_' as i32
                    || 'a' as i32 <= lookahead && lookahead <= 'z' as i32
                {
                    state = 165 as libc::c_int as TSStateId
                } else {
                    return result;
                }
            }
            _ => return 0 as libc::c_int != 0,
        }
        (*lexer).advance.expect("non-null function pointer")(lexer, skip);
    }
}

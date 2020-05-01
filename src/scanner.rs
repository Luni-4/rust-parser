use crate::parser_api::*;
use crate::utils::*;

pub const BLOCK_COMMENT: u32 = 3;
pub const FLOAT_LITERAL: u32 = 2;
pub const RAW_STRING_LITERAL: u32 = 1;
pub const STRING_CONTENT: u32 = 0;

pub struct RustScanner {}

#[inline(always)]
fn advance(lexer: &mut TSLexer) {
    lexer.advance.unwrap()(lexer, false);
}

#[inline(always)]
fn is_num_char(c: u32) -> bool {
    c == '_' as u32 || iswdigit(c) != 0
}

impl Scanner for RustScanner {
    fn tree_sitter_external_scanner_create() -> Self {
        RustScanner {}
    }
    //pub fn tree_sitter_external_scanner_destroy(&mut self) {}
    fn tree_sitter_external_scanner_reset(&mut self) {}
    fn tree_sitter_external_scanner_serialize(&mut self, _buffer: &mut [u8]) -> u32 {
        0
    }
    fn tree_sitter_external_scanner_deserialize(&mut self, _b: &mut [u8], _n: u32) {}

    fn tree_sitter_external_scanner_scan(
        &mut self,
        lexer: &mut TSLexer,
        valid_symbols: &[bool],
    ) -> bool {
        if valid_symbols[STRING_CONTENT as usize] && !valid_symbols[FLOAT_LITERAL as usize] {
            let mut has_content = false;
            while !(lexer.lookahead == '\"' as i32 || lexer.lookahead == '\\' as i32) {
                if lexer.lookahead == 0 {
                    return false;
                }
                has_content = true;
                advance(lexer);
            }
            lexer.result_symbol = STRING_CONTENT as u16;
            return has_content;
        }

        while iswspace(lexer.lookahead as u32) != 0 {
            lexer.advance.unwrap()(lexer, true);
        }

        if valid_symbols[RAW_STRING_LITERAL as usize]
            && (lexer.lookahead == 'r' as i32 || lexer.lookahead == 'b' as i32)
        {
            lexer.result_symbol = RAW_STRING_LITERAL as u16;

            if lexer.lookahead == 'b' as i32 {
                advance(lexer);
            }

            if lexer.lookahead != 'r' as i32 {
                return false;
            }
            advance(lexer);

            let mut opening_hash_count = 0;
            while lexer.lookahead == '#' as i32 {
                advance(lexer);
                opening_hash_count += 1;
            }

            if lexer.lookahead != '\"' as i32 {
                return false;
            }
            advance(lexer);

            loop {
                if lexer.lookahead == 0 {
                    return false;
                } else if lexer.lookahead == '\"' as i32 {
                    advance(lexer);
                    let mut hash_count = 0;
                    while lexer.lookahead == '#' as i32 && hash_count < opening_hash_count {
                        advance(lexer);
                        hash_count += 1;
                    }
                    if hash_count == opening_hash_count {
                        return true;
                    }
                } else {
                    advance(lexer);
                }
            }
        }

        if valid_symbols[FLOAT_LITERAL as usize] && iswdigit(lexer.lookahead as u32) != 0 {
            lexer.result_symbol = FLOAT_LITERAL as u16;
            advance(lexer);
            while is_num_char(lexer.lookahead as u32) {
                advance(lexer);
            }

            let mut has_fraction = false;
            let mut has_exponent = false;
            if lexer.lookahead == '.' as i32 {
                has_fraction = true;
                advance(lexer);
                if iswalpha(lexer.lookahead as u32) != 0 {
                    // The dot is followed by a letter: 1.max(2) => not a float but an integer
                    return false;
                }
                if lexer.lookahead == '.' as i32 {
                    return false;
                }
                while is_num_char(lexer.lookahead as u32) {
                    advance(lexer);
                }
            }

            lexer.mark_end.unwrap()(lexer);
            if lexer.lookahead == 'e' as i32 || lexer.lookahead == 'E' as i32 {
                has_exponent = true;
                advance(lexer);
                if lexer.lookahead == '+' as i32 || lexer.lookahead == '-' as i32 {
                    advance(lexer);
                }
                if !is_num_char(lexer.lookahead as u32) {
                    return true;
                }
                advance(lexer);
                while is_num_char(lexer.lookahead as u32) {
                    advance(lexer);
                }
                lexer.mark_end.unwrap()(lexer);
            }

            if !has_exponent && !has_fraction {
                return false;
            }

            if lexer.lookahead != 'u' as i32
                && lexer.lookahead != 'i' as i32
                && lexer.lookahead != 'f' as i32
            {
                return true;
            }
            advance(lexer);
            if iswdigit(lexer.lookahead as u32) == 0 {
                return true;
            }
            while iswdigit(lexer.lookahead as u32) != 0 {
                advance(lexer);
            }
            lexer.mark_end.unwrap()(lexer);
            return true;
        }

        if lexer.lookahead == '/' as i32 {
            advance(lexer);
            if lexer.lookahead != '*' as i32 {
                return false;
            }

            advance(lexer);
            let mut after_star = false;
            let mut nesting_depth = 1;

            loop {
                match lexer.lookahead {
                    0 => return false,
                    42 => {
                        advance(lexer);
                        after_star = true;
                    }
                    47 => {
                        if after_star {
                            advance(lexer);
                            after_star = false;
                            nesting_depth -= 1;
                            if nesting_depth == 0 {
                                lexer.result_symbol = BLOCK_COMMENT as u16;
                                return true;
                            }
                        } else {
                            advance(lexer);
                            after_star = false;
                            if lexer.lookahead == '*' as i32 {
                                nesting_depth += 1;
                                advance(lexer);
                            }
                        }
                    }
                    _ => {
                        advance(lexer);
                        after_star = false
                    }
                }
            }
        }
        false
    }
}

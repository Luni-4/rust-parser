#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::parser_api::*;
use crate::utils::*;

pub const BLOCK_COMMENT: u32 = 3;
pub const FLOAT_LITERAL: u32 = 2;
pub const RAW_STRING_LITERAL: u32 = 1;
pub const STRING_CONTENT: u32 = 0;

pub struct RustScanner {}

impl Scanner for RustScanner {
    pub fn tree_sitter_external_scanner_create() -> Self {
        RustScanner {}
    }
    //pub fn tree_sitter_external_scanner_destroy(&mut self) {}
    pub fn tree_sitter_external_scanner_reset(&mut self) {}
    pub fn tree_sitter_external_scanner_serialize(&mut self, buffer: &mut [u8]) -> u32 {
        0
    }
    pub fn tree_sitter_external_scanner_deserialize(&mut self, b: &mut [u8], n: u32) {}

    fn advance(lexer: &mut TSLexer) {
        lexer.advance(false);
    }
    fn is_num_char(c: u32) -> bool {
        c == '_' as u32 || iswdigit(c) != 0
    }

    pub fn tree_sitter_external_scanner_scan(
        &mut self,
        lexer: &mut TSLexer,
        valid_symbols: &[bool],
    ) -> bool {
        if *valid_symbols.offset(STRING_CONTENT as libc::c_int as isize) as libc::c_int != 0
            && !*valid_symbols.offset(FLOAT_LITERAL as libc::c_int as isize)
        {
            let mut has_content: bool = 0 as libc::c_int != 0;
            while !((*lexer).lookahead == '\"' as i32 || (*lexer).lookahead == '\\' as i32) {
                if (*lexer).lookahead == 0 as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
                has_content = 1 as libc::c_int != 0;
                advance(lexer);
            }
            (*lexer).result_symbol = STRING_CONTENT as libc::c_int as TSSymbol;
            return has_content;
        }
        while iswspace((*lexer).lookahead as u32) != 0 {
            (*lexer).advance.expect("non-null function pointer")(lexer, 1 as libc::c_int != 0);
        }
        if *valid_symbols.offset(RAW_STRING_LITERAL as libc::c_int as isize) as libc::c_int != 0
            && ((*lexer).lookahead == 'r' as i32 || (*lexer).lookahead == 'b' as i32)
        {
            (*lexer).result_symbol = RAW_STRING_LITERAL as libc::c_int as TSSymbol;
            if (*lexer).lookahead == 'b' as i32 {
                advance(lexer);
            }
            if (*lexer).lookahead != 'r' as i32 {
                return 0 as libc::c_int != 0;
            }
            advance(lexer);
            let mut opening_hash_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (*lexer).lookahead == '#' as i32 {
                advance(lexer);
                opening_hash_count = opening_hash_count.wrapping_add(1)
            }
            if (*lexer).lookahead != '\"' as i32 {
                return 0 as libc::c_int != 0;
            }
            advance(lexer);
            loop {
                if (*lexer).lookahead == 0 as libc::c_int {
                    return 0 as libc::c_int != 0;
                } else {
                    if (*lexer).lookahead == '\"' as i32 {
                        advance(lexer);
                        let mut hash_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                        while (*lexer).lookahead == '#' as i32 && hash_count < opening_hash_count {
                            advance(lexer);
                            hash_count = hash_count.wrapping_add(1)
                        }
                        if hash_count == opening_hash_count {
                            return 1 as libc::c_int != 0;
                        }
                    } else {
                        advance(lexer);
                    }
                }
            }
        }
        if *valid_symbols.offset(FLOAT_LITERAL as libc::c_int as isize) as libc::c_int != 0
            && iswdigit((*lexer).lookahead as u32) != 0
        {
            (*lexer).result_symbol = FLOAT_LITERAL as libc::c_int as TSSymbol;
            advance(lexer);
            while is_num_char((*lexer).lookahead) {
                advance(lexer);
            }
            let mut has_fraction: bool = 0 as libc::c_int != 0;
            let mut has_exponent: bool = 0 as libc::c_int != 0;
            if (*lexer).lookahead == '.' as i32 {
                has_fraction = 1 as libc::c_int != 0;
                advance(lexer);
                if iswalpha((*lexer).lookahead as u32) != 0 {
                    // The dot is followed by a letter: 1.max(2) => not a float but an integer
                    return 0 as libc::c_int != 0;
                }
                if (*lexer).lookahead == '.' as i32 {
                    return 0 as libc::c_int != 0;
                }
                while is_num_char((*lexer).lookahead) {
                    advance(lexer);
                }
            }
            (*lexer).mark_end.expect("non-null function pointer")(lexer);
            if (*lexer).lookahead == 'e' as i32 || (*lexer).lookahead == 'E' as i32 {
                has_exponent = 1 as libc::c_int != 0;
                advance(lexer);
                if (*lexer).lookahead == '+' as i32 || (*lexer).lookahead == '-' as i32 {
                    advance(lexer);
                }
                if !is_num_char((*lexer).lookahead) {
                    return 1 as libc::c_int != 0;
                }
                advance(lexer);
                while is_num_char((*lexer).lookahead) {
                    advance(lexer);
                }
                (*lexer).mark_end.expect("non-null function pointer")(lexer);
            }
            if !has_exponent && !has_fraction {
                return 0 as libc::c_int != 0;
            }
            if (*lexer).lookahead != 'u' as i32
                && (*lexer).lookahead != 'i' as i32
                && (*lexer).lookahead != 'f' as i32
            {
                return 1 as libc::c_int != 0;
            }
            advance(lexer);
            if iswdigit((*lexer).lookahead as u32) == 0 {
                return 1 as libc::c_int != 0;
            }
            while iswdigit((*lexer).lookahead as u32) != 0 {
                advance(lexer);
            }
            (*lexer).mark_end.expect("non-null function pointer")(lexer);
            return 1 as libc::c_int != 0;
        }
        if (*lexer).lookahead == '/' as i32 {
            advance(lexer);
            if (*lexer).lookahead != '*' as i32 {
                return 0 as libc::c_int != 0;
            }
            advance(lexer);
            let mut after_star: bool = 0 as libc::c_int != 0;
            let mut nesting_depth: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            loop {
                match (*lexer).lookahead {
                    0 => return 0 as libc::c_int != 0,
                    42 => {
                        advance(lexer);
                        after_star = 1 as libc::c_int != 0
                    }
                    47 => {
                        if after_star {
                            advance(lexer);
                            after_star = 0 as libc::c_int != 0;
                            nesting_depth = nesting_depth.wrapping_sub(1);
                            if nesting_depth == 0 as libc::c_int as libc::c_uint {
                                (*lexer).result_symbol = BLOCK_COMMENT as libc::c_int as TSSymbol;
                                return 1 as libc::c_int != 0;
                            }
                        } else {
                            advance(lexer);
                            after_star = 0 as libc::c_int != 0;
                            if (*lexer).lookahead == '*' as i32 {
                                nesting_depth = nesting_depth.wrapping_add(1);
                                advance(lexer);
                            }
                        }
                    }
                    _ => {
                        advance(lexer);
                        after_star = 0 as libc::c_int != 0
                    }
                }
            }
        }
        false
    }
}

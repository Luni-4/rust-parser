#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::ffi;

use crate::definition::*;
use crate::field_map_entries::*;
use crate::field_map_slices::*;
use crate::parser_api::*;
use crate::scanner::*;
use crate::ts_symbol_metadata::*;

pub const ts_external_scanner_symbol_map: [TSSymbol; 4] = [
    sym__string_content,
    sym_raw_string_literal,
    sym_float_literal,
    sym_block_comment,
];

pub const ts_external_scanner_states: [[bool; 4]; 6] = [
    [false; 4],
    [true, true, true, true],
    [false, true, true, true],
    [false, false, false, true],
    [true, false, false, true],
    [false, false, true, true],
];

// Initialized in run_static_initializers
static mut ts_parse_actions: [TSParseActionEntry; 5597] = [TSParseActionEntry {
    action: TSParseAction {
        params: TSParseActionParams {
            field_0: TSParseActionParamsState {
                state: 0,
                extra: false,
                repetition: false,
            },
        },
        type_0: TSParseActionTypeShift,
    },
}; 5597];

/*pub unsafe extern "C" fn tree_sitter_rust<S: Scanner>() -> TSLanguage<S> {
    let language = TSLanguage {
        version: 11,
        symbol_count: 312,
        alias_count: 4,
        token_count: 138,
        external_token_count: 4,
        symbol_names: ts_symbol_names.as_ptr() as *mut _,
        symbol_metadata: ts_symbol_metadata.as_ptr(),
        parse_table: ts_parse_table.as_ptr() as *mut _ as *const u16,
        parse_actions: ts_parse_actions.as_ptr() as *mut _,
        lex_modes: ts_lex_modes.as_ptr() as *mut _,
        alias_sequences: ts_alias_sequences.as_ptr() as *mut _ as *const TSSymbol,
        max_alias_sequence_length: 10,
        lex_fn: Some(ts_lex as unsafe extern "C" fn(_: *mut TSLexer, _: TSStateId) -> bool),
        keyword_lex_fn: Some(
            ts_lex_keywords as unsafe extern "C" fn(_: *mut TSLexer, _: TSStateId) -> bool,
        ),
        keyword_capture_token: sym_identifier,
        external_scanner: TSLanguageExternalScanner {
            states: ts_external_scanner_states.as_ptr() as *mut _ as *const bool,
            symbol_map: ts_external_scanner_symbol_map.as_ptr() as *mut _,
            scanner: RustScanner,
        },
        field_count: 28,
        field_map_slices: ts_field_map_slices.as_ptr(),
        field_map_entries: ts_field_map_entries.as_ptr(),
        field_names: ts_field_names.as_ptr() as *mut _,
        large_state_count: 538,
        small_parse_table: ts_small_parse_table.as_ptr() as *mut _ as *const u16,
        small_parse_table_map: ts_small_parse_table_map.as_ptr() as *mut _ as *const u32,
        public_symbol_map: ts_symbol_map.as_ptr() as *mut _,
    };
    language
}*/

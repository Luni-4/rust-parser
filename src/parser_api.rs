use std::{ffi, os};

pub type TSSymbol = u16;
pub type TSFieldId = u16;
pub type TSStateId = u16;

pub type TSParseActionType = u32;
pub const TSParseActionTypeRecover: TSParseActionType = 3;
pub const TSParseActionTypeAccept: TSParseActionType = 2;
pub const TSParseActionTypeReduce: TSParseActionType = 1;
pub const TSParseActionTypeShift: TSParseActionType = 0;

pub trait Scanner {
    fn tree_sitter_external_scanner_create() -> Self;
    //fn tree_sitter_external_scanner_destroy(&mut self);
    fn tree_sitter_external_scanner_reset(&mut self);
    fn tree_sitter_external_scanner_scan(
        &mut self,
        lexer: &mut TSLexer,
        valid_symbols: &[bool],
    ) -> bool;
    fn tree_sitter_external_scanner_serialize(&mut self, buffer: &mut [u8]) -> u32;
    fn tree_sitter_external_scanner_deserialize(&mut self, buffer: &mut [u8], n: u32);
}

pub struct TSFieldMapEntry {
    pub field_id: TSFieldId,
    pub child_index: u8,
    pub inherited: bool,
}

pub struct TSFieldMapSlice {
    pub index: u16,
    pub length: u16,
}

pub struct TSSymbolMetadata {
    pub visible: bool,
    pub named: bool,
}

pub struct TSLexer {
    pub lookahead: i32,
    pub result_symbol: TSSymbol,
    pub advance: Option<fn(_: &mut TSLexer, _: bool) -> ()>,
    pub mark_end: Option<fn(_: &mut TSLexer) -> ()>,
    pub get_column: Option<fn(_: &mut TSLexer) -> u32>,
    pub is_at_included_range_start: Option<fn(_: &TSLexer) -> bool>,
    pub eof: Option<fn(_: &TSLexer) -> bool>,
}

#[derive(Copy, Clone)]
pub struct TSParseAction {
    pub params: TSParseActionParams,
    pub type_0: TSParseActionType,
}

#[derive(Copy, Clone)]
pub union TSParseActionParams {
    pub field_0: TSParseActionParamsState,
    pub field_1: TSParseActionParamsSymbol,
}

#[derive(Copy, Clone)]
pub struct TSParseActionParamsSymbol {
    pub symbol: TSSymbol,
    pub dynamic_precedence: i16,
    pub child_count: u8,
    pub production_id: u8,
}

#[derive(Copy, Clone)]
pub struct TSParseActionParamsState {
    pub state: TSStateId,
    pub extra: bool,
    pub repetition: bool,
}

pub struct TSLexMode {
    pub lex_state: u16,
    pub external_lex_state: u16,
}

#[derive(Copy, Clone)]
pub union TSParseActionEntry {
    pub action: TSParseAction,
    pub field_0: TSParseActionEntryContent,
}

#[derive(Copy, Clone)]
pub struct TSParseActionEntryContent {
    pub count: u8,
    pub reusable: bool,
}

pub struct TSLanguage {
    pub version: u32,
    pub symbol_count: u32,
    pub alias_count: u32,
    pub token_count: u32,
    pub external_token_count: u32,
    pub symbol_names: *mut *const os::raw::c_char,
    pub symbol_metadata: *const TSSymbolMetadata,
    pub parse_table: *const u16,
    pub parse_actions: *const TSParseActionEntry,
    pub lex_modes: *const TSLexMode,
    pub alias_sequences: *const TSSymbol,
    pub max_alias_sequence_length: u16,
    pub lex_fn: Option<fn(_: &mut TSLexer, _: TSStateId) -> bool>,
    pub keyword_lex_fn: Option<fn(_: &mut TSLexer, _: TSStateId) -> bool>,
    pub keyword_capture_token: TSSymbol,
    pub external_scanner: TSLanguageExternalScanner,
    pub field_count: u32,
    pub field_map_slices: *const TSFieldMapSlice,
    pub field_map_entries: *const TSFieldMapEntry,
    pub field_names: *mut *const os::raw::c_char,
    pub large_state_count: u32,
    pub small_parse_table: *const u16,
    pub small_parse_table_map: *const u32,
    pub public_symbol_map: *const TSSymbol,
}

pub struct TSLanguageExternalScanner {
    pub states: *const bool,
    pub symbol_map: *const TSSymbol,
    pub scanner: Scanner,
}

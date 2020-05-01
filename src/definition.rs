#![allow(non_upper_case_globals)]

use crate::parser_api::*;

// Sym identifier
pub const alias_sym_type_identifier: u16 = 315;
pub const alias_sym_shorthand_field_identifier: u16 = 314;
pub const alias_sym_primitive_type: u16 = 313;
pub const alias_sym_field_identifier: u16 = 312;
pub const aux_sym_string_literal_repeat1: u16 = 311;
pub const aux_sym_struct_pattern_repeat1: u16 = 310;
pub const aux_sym_tuple_pattern_repeat1: u16 = 309;
pub const aux_sym_closure_parameters_repeat1: u16 = 308;
pub const aux_sym_match_pattern_repeat1: u16 = 307;
pub const aux_sym_match_block_repeat1: u16 = 306;
pub const aux_sym_field_initializer_list_repeat1: u16 = 305;
pub const aux_sym_tuple_expression_repeat1: u16 = 304;
pub const aux_sym_array_expression_repeat1: u16 = 303;
pub const aux_sym_arguments_repeat1: u16 = 302;
pub const aux_sym_type_arguments_repeat1: u16 = 301;
pub const aux_sym_tuple_type_repeat1: u16 = 300;
pub const aux_sym_for_lifetimes_repeat1: u16 = 299;
pub const aux_sym_parameters_repeat1: u16 = 298;
pub const aux_sym_use_list_repeat1: u16 = 297;
pub const aux_sym_type_parameters_repeat1: u16 = 296;
pub const aux_sym_trait_bounds_repeat1: u16 = 295;
pub const aux_sym_where_clause_repeat1: u16 = 294;
pub const aux_sym_function_modifiers_repeat1: u16 = 293;
pub const aux_sym_ordered_field_declaration_list_repeat1: u16 = 292;
pub const aux_sym_field_declaration_list_repeat1: u16 = 291;
pub const aux_sym_enum_variant_list_repeat2: u16 = 290;
pub const aux_sym_enum_variant_list_repeat1: u16 = 289;
pub const aux_sym_declaration_list_repeat1: u16 = 288;
pub const aux_sym_meta_arguments_repeat1: u16 = 287;
pub const aux_sym_token_tree_repeat1: u16 = 286;
pub const aux_sym_token_tree_pattern_repeat1: u16 = 285;
pub const aux_sym_macro_definition_repeat1: u16 = 284;
pub const aux_sym_source_file_repeat1: u16 = 283;
pub const sym_boolean_literal: u16 = 282;
pub const sym_string_literal: u16 = 281;
pub const sym_negative_literal: u16 = 280;
pub const sym__literal_pattern: u16 = 279;
pub const sym__literal: u16 = 278;
pub const sym_reference_pattern: u16 = 277;
pub const sym_captured_pattern: u16 = 276;
pub const sym_ref_pattern: u16 = 275;
pub const sym_range_pattern: u16 = 274;
pub const sym_mut_pattern: u16 = 273;
pub const sym_remaining_field_pattern: u16 = 272;
pub const sym_field_pattern: u16 = 271;
pub const sym_struct_pattern: u16 = 270;
pub const sym_tuple_struct_pattern: u16 = 269;
pub const sym_slice_pattern: u16 = 268;
pub const sym_tuple_pattern: u16 = 267;
pub const sym__pattern: u16 = 266;
pub const sym_block: u16 = 265;
pub const sym_async_block: u16 = 264;
pub const sym_unsafe_block: u16 = 263;
pub const sym_field_expression: u16 = 262;
pub const sym_await_expression: u16 = 261;
pub const sym_index_expression: u16 = 260;
pub const sym_continue_expression: u16 = 259;
pub const sym_break_expression: u16 = 258;
pub const sym_loop_label: u16 = 257;
pub const sym_closure_parameters: u16 = 256;
pub const sym_closure_expression: u16 = 255;
pub const sym_for_expression: u16 = 254;
pub const sym_loop_expression: u16 = 253;
pub const sym_while_let_expression: u16 = 252;
pub const sym_while_expression: u16 = 251;
pub const sym_match_pattern: u16 = 250;
pub const sym_last_match_arm: u16 = 249;
pub const sym_match_arm: u16 = 248;
pub const sym_match_block: u16 = 247;
pub const sym_match_expression: u16 = 246;
pub const sym__else_tail: u16 = 245;
pub const sym_if_let_expression: u16 = 244;
pub const sym_if_expression: u16 = 243;
pub const sym_base_field_initializer: u16 = 242;
pub const sym_field_initializer: u16 = 241;
pub const sym_shorthand_field_initializer: u16 = 240;
pub const sym_field_initializer_list: u16 = 239;
pub const sym_struct_expression: u16 = 238;
pub const sym_unit_expression: u16 = 237;
pub const sym_tuple_expression: u16 = 236;
pub const sym_parenthesized_expression: u16 = 235;
pub const sym_array_expression: u16 = 234;
pub const sym_arguments: u16 = 233;
pub const sym_call_expression: u16 = 232;
pub const sym_return_expression: u16 = 231;
pub const sym_type_cast_expression: u16 = 230;
pub const sym_compound_assignment_expr: u16 = 229;
pub const sym_assignment_expression: u16 = 228;
pub const sym_binary_expression: u16 = 227;
pub const sym_reference_expression: u16 = 226;
pub const sym_try_expression: u16 = 225;
pub const sym_unary_expression: u16 = 224;
pub const sym_range_expression: u16 = 223;
pub const sym_scoped_type_identifier: u16 = 222;
pub const sym_scoped_type_identifier_in_expression_position: u16 = 221;
pub const sym_scoped_identifier: u16 = 220;
pub const sym_macro_invocation: u16 = 219;
pub const sym__expression: u16 = 218;
pub const sym_dynamic_type: u16 = 217;
pub const sym_abstract_type: u16 = 216;
pub const sym_empty_type: u16 = 215;
pub const sym_pointer_type: u16 = 214;
pub const sym_reference_type: u16 = 213;
pub const sym_type_binding: u16 = 212;
pub const sym_type_arguments: u16 = 211;
pub const sym_bounded_type: u16 = 210;
pub const sym_generic_type_with_turbofish: u16 = 209;
pub const sym_generic_type: u16 = 208;
pub const sym_generic_function: u16 = 207;
pub const sym_unit_type: u16 = 206;
pub const sym_tuple_type: u16 = 205;
pub const sym_function_type: u16 = 204;
pub const sym_for_lifetimes: u16 = 203;
pub const sym_array_type: u16 = 202;
pub const sym_lifetime: u16 = 201;
pub const sym_qualified_type: u16 = 200;
pub const sym_bracketed_type: u16 = 199;
pub const sym__type: u16 = 198;
pub const sym_visibility_modifier: u16 = 197;
pub const sym_extern_modifier: u16 = 196;
pub const sym_parameter: u16 = 195;
pub const sym_variadic_parameter: u16 = 194;
pub const sym_self_parameter: u16 = 193;
pub const sym_parameters: u16 = 192;
pub const sym_use_wildcard: u16 = 191;
pub const sym_use_as_clause: u16 = 190;
pub const sym_use_list: u16 = 189;
pub const sym_scoped_use_list: u16 = 188;
pub const sym__use_clause: u16 = 187;
pub const sym_use_declaration: u16 = 186;
pub const sym_let_declaration: u16 = 185;
pub const sym_optional_type_parameter: u16 = 184;
pub const sym_constrained_type_parameter: u16 = 183;
pub const sym_const_parameter: u16 = 182;
pub const sym_type_parameters: u16 = 181;
pub const sym_removed_trait_bound: u16 = 180;
pub const sym_higher_ranked_trait_bound: u16 = 179;
pub const sym_trait_bounds: u16 = 178;
pub const sym_associated_type: u16 = 177;
pub const sym_trait_item: u16 = 176;
pub const sym_impl_item: u16 = 175;
pub const sym_where_predicate: u16 = 174;
pub const sym_where_clause: u16 = 173;
pub const sym_function_modifiers: u16 = 172;
pub const sym_function_signature_item: u16 = 171;
pub const sym_function_item: u16 = 170;
pub const sym_type_item: u16 = 169;
pub const sym_static_item: u16 = 168;
pub const sym_const_item: u16 = 167;
pub const sym_extern_crate_declaration: u16 = 166;
pub const sym_ordered_field_declaration_list: u16 = 165;
pub const sym_field_declaration: u16 = 164;
pub const sym_field_declaration_list: u16 = 163;
pub const sym_enum_variant: u16 = 162;
pub const sym_enum_variant_list: u16 = 161;
pub const sym_enum_item: u16 = 160;
pub const sym_union_item: u16 = 159;
pub const sym_struct_item: u16 = 158;
pub const sym_declaration_list: u16 = 157;
pub const sym_foreign_mod_item: u16 = 156;
pub const sym_mod_item: u16 = 155;
pub const sym_meta_arguments: u16 = 154;
pub const sym_meta_item: u16 = 153;
pub const sym_inner_attribute_item: u16 = 152;
pub const sym_attribute_item: u16 = 151;
pub const sym_token_repetition: u16 = 150;
pub const sym_token_tree: u16 = 149;
pub const sym_fragment_specifier: u16 = 148;
pub const sym_token_repetition_pattern: u16 = 147;
pub const sym_token_binding_pattern: u16 = 146;
pub const sym_token_tree_pattern: u16 = 145;
pub const sym__token_pattern: u16 = 144;
pub const sym_macro_rule: u16 = 143;
pub const sym_macro_definition: u16 = 142;
pub const sym__expression_statement: u16 = 141;
pub const sym_empty_statement: u16 = 140;
pub const sym__statement: u16 = 139;
pub const sym_source_file: u16 = 138;
pub const sym_block_comment: u16 = 137;
pub const sym_float_literal: u16 = 136;
pub const sym_raw_string_literal: u16 = 135;
pub const sym__string_content: u16 = 134;
pub const sym_metavariable: u16 = 133;
pub const sym_crate: u16 = 132;
pub const sym_super: u16 = 131;
pub const sym_self: u16 = 130;
pub const sym_line_comment: u16 = 129;
pub const anon_sym_false: u16 = 128;
pub const anon_sym_true: u16 = 127;
pub const sym_escape_sequence: u16 = 126;
pub const sym_char_literal: u16 = 125;
pub const anon_sym_DQUOTE: u16 = 124;
pub const aux_sym_string_literal_token1: u16 = 123;
pub const sym_integer_literal: u16 = 122;
pub const anon_sym_AT: u16 = 121;
pub const anon_sym_DOT: u16 = 120;
pub const anon_sym_move: u16 = 119;
pub const anon_sym_else: u16 = 118;
pub const anon_sym_GT_GT_EQ: u16 = 117;
pub const anon_sym_LT_LT_EQ: u16 = 116;
pub const anon_sym_CARET_EQ: u16 = 115;
pub const anon_sym_PIPE_EQ: u16 = 114;
pub const anon_sym_AMP_EQ: u16 = 113;
pub const anon_sym_PERCENT_EQ: u16 = 112;
pub const anon_sym_SLASH_EQ: u16 = 111;
pub const anon_sym_STAR_EQ: u16 = 110;
pub const anon_sym_DASH_EQ: u16 = 109;
pub const anon_sym_PLUS_EQ: u16 = 108;
pub const anon_sym_PERCENT: u16 = 107;
pub const anon_sym_SLASH: u16 = 106;
pub const anon_sym_GT_GT: u16 = 105;
pub const anon_sym_LT_LT: u16 = 104;
pub const anon_sym_GT_EQ: u16 = 103;
pub const anon_sym_LT_EQ: u16 = 102;
pub const anon_sym_BANG_EQ: u16 = 101;
pub const anon_sym_EQ_EQ: u16 = 100;
pub const anon_sym_CARET: u16 = 99;
pub const anon_sym_PIPE: u16 = 98;
pub const anon_sym_PIPE_PIPE: u16 = 97;
pub const anon_sym_AMP_AMP: u16 = 96;
pub const anon_sym_DASH: u16 = 95;
pub const anon_sym_DOT_DOT_EQ: u16 = 94;
pub const anon_sym_DOT_DOT: u16 = 93;
pub const sym_mutable_specifier: u16 = 92;
pub const anon_sym_dyn: u16 = 91;
pub const anon_sym_LT2: u16 = 90;
pub const anon_sym_in: u16 = 89;
pub const anon_sym_DOT_DOT_DOT: u16 = 88;
pub const anon_sym_AMP: u16 = 87;
pub const anon_sym__: u16 = 86;
pub const anon_sym_COLON_COLON: u16 = 85;
pub const anon_sym_GT: u16 = 84;
pub const anon_sym_LT: u16 = 83;
pub const anon_sym_DASH_GT: u16 = 82;
pub const anon_sym_ref: u16 = 81;
pub const anon_sym_extern: u16 = 80;
pub const anon_sym_COMMA: u16 = 79;
pub const anon_sym_EQ: u16 = 78;
pub const anon_sym_BANG: u16 = 77;
pub const anon_sym_POUND: u16 = 76;
pub const anon_sym_while: u16 = 75;
pub const anon_sym_where: u16 = 74;
pub const anon_sym_use: u16 = 73;
pub const anon_sym_unsafe: u16 = 72;
pub const anon_sym_union: u16 = 71;
pub const anon_sym_type: u16 = 70;
pub const anon_sym_trait: u16 = 69;
pub const anon_sym_struct: u16 = 68;
pub const anon_sym_static: u16 = 67;
pub const anon_sym_return: u16 = 66;
pub const anon_sym_pub: u16 = 65;
pub const anon_sym_mod: u16 = 64;
pub const anon_sym_match: u16 = 63;
pub const anon_sym_loop: u16 = 62;
pub const anon_sym_let: u16 = 61;
pub const anon_sym_impl: u16 = 60;
pub const anon_sym_if: u16 = 59;
pub const anon_sym_for: u16 = 58;
pub const anon_sym_fn: u16 = 57;
pub const anon_sym_enum: u16 = 56;
pub const anon_sym_default: u16 = 55;
pub const anon_sym_continue: u16 = 54;
pub const anon_sym_const: u16 = 53;
pub const anon_sym_break: u16 = 52;
pub const anon_sym_await: u16 = 51;
pub const anon_sym_async: u16 = 50;
pub const anon_sym_as: u16 = 49;
pub const anon_sym_SQUOTE: u16 = 48;
pub const aux_sym__non_special_token_token1: u16 = 47;
pub const anon_sym_char: u16 = 46;
pub const anon_sym_str: u16 = 45;
pub const anon_sym_bool: u16 = 44;
pub const anon_sym_f64: u16 = 43;
pub const anon_sym_f32: u16 = 42;
pub const anon_sym_usize: u16 = 41;
pub const anon_sym_isize: u16 = 40;
pub const anon_sym_i128: u16 = 39;
pub const anon_sym_u128: u16 = 38;
pub const anon_sym_i64: u16 = 37;
pub const anon_sym_u64: u16 = 36;
pub const anon_sym_i32: u16 = 35;
pub const anon_sym_u32: u16 = 34;
pub const anon_sym_i16: u16 = 33;
pub const anon_sym_u16: u16 = 32;
pub const anon_sym_i8: u16 = 31;
pub const anon_sym_u8: u16 = 30;
pub const anon_sym_vis: u16 = 29;
pub const anon_sym_ty: u16 = 28;
pub const anon_sym_tt: u16 = 27;
pub const anon_sym_stmt: u16 = 26;
pub const anon_sym_path: u16 = 25;
pub const anon_sym_pat: u16 = 24;
pub const anon_sym_meta: u16 = 23;
pub const anon_sym_literal: u16 = 22;
pub const anon_sym_lifetime: u16 = 21;
pub const anon_sym_item: u16 = 20;
pub const anon_sym_ident: u16 = 19;
pub const anon_sym_expr: u16 = 18;
pub const anon_sym_block: u16 = 17;
pub const anon_sym_QMARK: u16 = 16;
pub const anon_sym_STAR: u16 = 15;
pub const anon_sym_PLUS: u16 = 14;
pub const aux_sym_token_repetition_pattern_token1: u16 = 13;
pub const anon_sym_DOLLAR: u16 = 12;
pub const anon_sym_COLON: u16 = 11;
pub const anon_sym_RBRACK: u16 = 10;
pub const anon_sym_LBRACK: u16 = 9;
pub const anon_sym_EQ_GT: u16 = 8;
pub const anon_sym_RBRACE: u16 = 7;
pub const anon_sym_LBRACE: u16 = 6;
pub const anon_sym_RPAREN: u16 = 5;
pub const anon_sym_LPAREN: u16 = 4;
pub const anon_sym_macro_rules_BANG: u16 = 3;
pub const anon_sym_SEMI: u16 = 2;
pub const sym_identifier: u16 = 1;

// Field values
pub const field_value: u32 = 28;
pub const field_type_parameters: u32 = 27;
pub const field_type_arguments: u32 = 26;
pub const field_type: u32 = 25;
pub const field_trait: u32 = 24;
pub const field_right: u32 = 23;
pub const field_return_type: u32 = 22;
pub const field_pattern: u32 = 21;
pub const field_path: u32 = 20;
pub const field_parameters: u32 = 19;
pub const field_operator: u32 = 18;
pub const field_name: u32 = 17;
pub const field_macro: u32 = 16;
pub const field_list: u32 = 15;
pub const field_length: u32 = 14;
pub const field_left: u32 = 13;
pub const field_function: u32 = 12;
pub const field_field: u32 = 11;
pub const field_element: u32 = 10;
pub const field_default_type: u32 = 9;
pub const field_consequence: u32 = 8;
pub const field_condition: u32 = 7;
pub const field_bounds: u32 = 6;
pub const field_body: u32 = 5;
pub const field_arguments: u32 = 4;
pub const field_argument: u32 = 3;
pub const field_alternative: u32 = 2;
pub const field_alias: u32 = 1;

pub const ts_symbol_names: [&'static [u8]; 316] = [
    b"end\x00",
    b"identifier\x00",
    b";\x00",
    b"macro_rules!\x00",
    b"(\x00",
    b")\x00",
    b"{\x00",
    b"}\x00",
    b"=>\x00",
    b"[\x00",
    b"]\x00",
    b":\x00",
    b"$\x00",
    b"token_repetition_pattern_token1\x00",
    b"+\x00",
    b"*\x00",
    b"?\x00",
    b"block\x00",
    b"expr\x00",
    b"ident\x00",
    b"item\x00",
    b"lifetime\x00",
    b"literal\x00",
    b"meta\x00",
    b"pat\x00",
    b"path\x00",
    b"stmt\x00",
    b"tt\x00",
    b"ty\x00",
    b"vis\x00",
    b"u8\x00",
    b"i8\x00",
    b"u16\x00",
    b"i16\x00",
    b"u32\x00",
    b"i32\x00",
    b"u64\x00",
    b"i64\x00",
    b"u128\x00",
    b"i128\x00",
    b"isize\x00",
    b"usize\x00",
    b"f32\x00",
    b"f64\x00",
    b"bool\x00",
    b"str\x00",
    b"char\x00",
    b"_non_special_token_token1\x00",
    b"\'\x00",
    b"as\x00",
    b"async\x00",
    b"await\x00",
    b"break\x00",
    b"const\x00",
    b"continue\x00",
    b"default\x00",
    b"enum\x00",
    b"fn\x00",
    b"for\x00",
    b"if\x00",
    b"impl\x00",
    b"let\x00",
    b"loop\x00",
    b"match\x00",
    b"mod\x00",
    b"pub\x00",
    b"return\x00",
    b"static\x00",
    b"struct\x00",
    b"trait\x00",
    b"type\x00",
    b"union\x00",
    b"unsafe\x00",
    b"use\x00",
    b"where\x00",
    b"while\x00",
    b"#\x00",
    b"!\x00",
    b"=\x00",
    b",\x00",
    b"extern\x00",
    b"ref\x00",
    b"->\x00",
    b"<\x00",
    b">\x00",
    b"::\x00",
    b"_\x00",
    b"&\x00",
    b"...\x00",
    b"in\x00",
    b"<\x00",
    b"dyn\x00",
    b"mutable_specifier\x00",
    b"..\x00",
    b"..=\x00",
    b"-\x00",
    b"&&\x00",
    b"||\x00",
    b"|\x00",
    b"^\x00",
    b"==\x00",
    b"!=\x00",
    b"<=\x00",
    b">=\x00",
    b"<<\x00",
    b">>\x00",
    b"/\x00",
    b"%\x00",
    b"+=\x00",
    b"-=\x00",
    b"*=\x00",
    b"/=\x00",
    b"%=\x00",
    b"&=\x00",
    b"|=\x00",
    b"^=\x00",
    b"<<=\x00",
    b">>=\x00",
    b"else\x00",
    b"move\x00",
    b".\x00",
    b"@\x00",
    b"integer_literal\x00",
    b"\"\x00",
    b"\"\x00",
    b"char_literal\x00",
    b"escape_sequence\x00",
    b"true\x00",
    b"false\x00",
    b"line_comment\x00",
    b"self\x00",
    b"super\x00",
    b"crate\x00",
    b"metavariable\x00",
    b"_string_content\x00",
    b"raw_string_literal\x00",
    b"float_literal\x00",
    b"block_comment\x00",
    b"source_file\x00",
    b"_statement\x00",
    b"empty_statement\x00",
    b"_expression_statement\x00",
    b"macro_definition\x00",
    b"macro_rule\x00",
    b"_token_pattern\x00",
    b"token_tree_pattern\x00",
    b"token_binding_pattern\x00",
    b"token_repetition_pattern\x00",
    b"fragment_specifier\x00",
    b"token_tree\x00",
    b"token_repetition\x00",
    b"attribute_item\x00",
    b"inner_attribute_item\x00",
    b"meta_item\x00",
    b"meta_arguments\x00",
    b"mod_item\x00",
    b"foreign_mod_item\x00",
    b"declaration_list\x00",
    b"struct_item\x00",
    b"union_item\x00",
    b"enum_item\x00",
    b"enum_variant_list\x00",
    b"enum_variant\x00",
    b"field_declaration_list\x00",
    b"field_declaration\x00",
    b"ordered_field_declaration_list\x00",
    b"extern_crate_declaration\x00",
    b"const_item\x00",
    b"static_item\x00",
    b"type_item\x00",
    b"function_item\x00",
    b"function_signature_item\x00",
    b"function_modifiers\x00",
    b"where_clause\x00",
    b"where_predicate\x00",
    b"impl_item\x00",
    b"trait_item\x00",
    b"associated_type\x00",
    b"trait_bounds\x00",
    b"higher_ranked_trait_bound\x00",
    b"removed_trait_bound\x00",
    b"type_parameters\x00",
    b"const_parameter\x00",
    b"constrained_type_parameter\x00",
    b"optional_type_parameter\x00",
    b"let_declaration\x00",
    b"use_declaration\x00",
    b"_use_clause\x00",
    b"scoped_use_list\x00",
    b"use_list\x00",
    b"use_as_clause\x00",
    b"use_wildcard\x00",
    b"parameters\x00",
    b"self_parameter\x00",
    b"variadic_parameter\x00",
    b"parameter\x00",
    b"extern_modifier\x00",
    b"visibility_modifier\x00",
    b"_type\x00",
    b"bracketed_type\x00",
    b"qualified_type\x00",
    b"lifetime\x00",
    b"array_type\x00",
    b"for_lifetimes\x00",
    b"function_type\x00",
    b"tuple_type\x00",
    b"unit_type\x00",
    b"generic_function\x00",
    b"generic_type\x00",
    b"generic_type_with_turbofish\x00",
    b"bounded_type\x00",
    b"type_arguments\x00",
    b"type_binding\x00",
    b"reference_type\x00",
    b"pointer_type\x00",
    b"empty_type\x00",
    b"abstract_type\x00",
    b"dynamic_type\x00",
    b"_expression\x00",
    b"macro_invocation\x00",
    b"scoped_identifier\x00",
    b"scoped_type_identifier\x00",
    b"scoped_type_identifier\x00",
    b"range_expression\x00",
    b"unary_expression\x00",
    b"try_expression\x00",
    b"reference_expression\x00",
    b"binary_expression\x00",
    b"assignment_expression\x00",
    b"compound_assignment_expr\x00",
    b"type_cast_expression\x00",
    b"return_expression\x00",
    b"call_expression\x00",
    b"arguments\x00",
    b"array_expression\x00",
    b"parenthesized_expression\x00",
    b"tuple_expression\x00",
    b"unit_expression\x00",
    b"struct_expression\x00",
    b"field_initializer_list\x00",
    b"shorthand_field_initializer\x00",
    b"field_initializer\x00",
    b"base_field_initializer\x00",
    b"if_expression\x00",
    b"if_let_expression\x00",
    b"_else_tail\x00",
    b"match_expression\x00",
    b"match_block\x00",
    b"match_arm\x00",
    b"match_arm\x00",
    b"match_pattern\x00",
    b"while_expression\x00",
    b"while_let_expression\x00",
    b"loop_expression\x00",
    b"for_expression\x00",
    b"closure_expression\x00",
    b"closure_parameters\x00",
    b"loop_label\x00",
    b"break_expression\x00",
    b"continue_expression\x00",
    b"index_expression\x00",
    b"await_expression\x00",
    b"field_expression\x00",
    b"unsafe_block\x00",
    b"async_block\x00",
    b"block\x00",
    b"_pattern\x00",
    b"tuple_pattern\x00",
    b"slice_pattern\x00",
    b"tuple_struct_pattern\x00",
    b"struct_pattern\x00",
    b"field_pattern\x00",
    b"remaining_field_pattern\x00",
    b"mut_pattern\x00",
    b"range_pattern\x00",
    b"ref_pattern\x00",
    b"captured_pattern\x00",
    b"reference_pattern\x00",
    b"_literal\x00",
    b"_literal_pattern\x00",
    b"negative_literal\x00",
    b"string_literal\x00",
    b"boolean_literal\x00",
    b"source_file_repeat1\x00",
    b"macro_definition_repeat1\x00",
    b"token_tree_pattern_repeat1\x00",
    b"token_tree_repeat1\x00",
    b"meta_arguments_repeat1\x00",
    b"declaration_list_repeat1\x00",
    b"enum_variant_list_repeat1\x00",
    b"enum_variant_list_repeat2\x00",
    b"field_declaration_list_repeat1\x00",
    b"ordered_field_declaration_list_repeat1\x00",
    b"function_modifiers_repeat1\x00",
    b"where_clause_repeat1\x00",
    b"trait_bounds_repeat1\x00",
    b"type_parameters_repeat1\x00",
    b"use_list_repeat1\x00",
    b"parameters_repeat1\x00",
    b"for_lifetimes_repeat1\x00",
    b"tuple_type_repeat1\x00",
    b"type_arguments_repeat1\x00",
    b"arguments_repeat1\x00",
    b"array_expression_repeat1\x00",
    b"tuple_expression_repeat1\x00",
    b"field_initializer_list_repeat1\x00",
    b"match_block_repeat1\x00",
    b"match_pattern_repeat1\x00",
    b"closure_parameters_repeat1\x00",
    b"tuple_pattern_repeat1\x00",
    b"struct_pattern_repeat1\x00",
    b"string_literal_repeat1\x00",
    b"field_identifier\x00",
    b"primitive_type\x00",
    b"shorthand_field_identifier\x00",
    b"type_identifier\x00",
];

pub const ts_symbol_map: [TSSymbol; 316] = [
    0,
    sym_identifier,
    anon_sym_SEMI,
    anon_sym_macro_rules_BANG,
    anon_sym_LPAREN,
    anon_sym_RPAREN,
    anon_sym_LBRACE,
    anon_sym_RBRACE,
    anon_sym_EQ_GT,
    anon_sym_LBRACK,
    anon_sym_RBRACK,
    anon_sym_COLON,
    anon_sym_DOLLAR,
    aux_sym_token_repetition_pattern_token1,
    anon_sym_PLUS,
    anon_sym_STAR,
    anon_sym_QMARK,
    anon_sym_block,
    anon_sym_expr,
    anon_sym_ident,
    anon_sym_item,
    anon_sym_lifetime,
    anon_sym_literal,
    anon_sym_meta,
    anon_sym_pat,
    anon_sym_path,
    anon_sym_stmt,
    anon_sym_tt,
    anon_sym_ty,
    anon_sym_vis,
    anon_sym_u8,
    anon_sym_i8,
    anon_sym_u16,
    anon_sym_i16,
    anon_sym_u32,
    anon_sym_i32,
    anon_sym_u64,
    anon_sym_i64,
    anon_sym_u128,
    anon_sym_i128,
    anon_sym_isize,
    anon_sym_usize,
    anon_sym_f32,
    anon_sym_f64,
    anon_sym_bool,
    anon_sym_str,
    anon_sym_char,
    aux_sym__non_special_token_token1,
    anon_sym_SQUOTE,
    anon_sym_as,
    anon_sym_async,
    anon_sym_await,
    anon_sym_break,
    anon_sym_const,
    anon_sym_continue,
    anon_sym_default,
    anon_sym_enum,
    anon_sym_fn,
    anon_sym_for,
    anon_sym_if,
    anon_sym_impl,
    anon_sym_let,
    anon_sym_loop,
    anon_sym_match,
    anon_sym_mod,
    anon_sym_pub,
    anon_sym_return,
    anon_sym_static,
    anon_sym_struct,
    anon_sym_trait,
    anon_sym_type,
    anon_sym_union,
    anon_sym_unsafe,
    anon_sym_use,
    anon_sym_where,
    anon_sym_while,
    anon_sym_POUND,
    anon_sym_BANG,
    anon_sym_EQ,
    anon_sym_COMMA,
    anon_sym_extern,
    anon_sym_ref,
    anon_sym_DASH_GT,
    anon_sym_LT,
    anon_sym_GT,
    anon_sym_COLON_COLON,
    anon_sym__,
    anon_sym_AMP,
    anon_sym_DOT_DOT_DOT,
    anon_sym_in,
    anon_sym_LT2,
    anon_sym_dyn,
    sym_mutable_specifier,
    anon_sym_DOT_DOT,
    anon_sym_DOT_DOT_EQ,
    anon_sym_DASH,
    anon_sym_AMP_AMP,
    anon_sym_PIPE_PIPE,
    anon_sym_PIPE,
    anon_sym_CARET,
    anon_sym_EQ_EQ,
    anon_sym_BANG_EQ,
    anon_sym_LT_EQ,
    anon_sym_GT_EQ,
    anon_sym_LT_LT,
    anon_sym_GT_GT,
    anon_sym_SLASH,
    anon_sym_PERCENT,
    anon_sym_PLUS_EQ,
    anon_sym_DASH_EQ,
    anon_sym_STAR_EQ,
    anon_sym_SLASH_EQ,
    anon_sym_PERCENT_EQ,
    anon_sym_AMP_EQ,
    anon_sym_PIPE_EQ,
    anon_sym_CARET_EQ,
    anon_sym_LT_LT_EQ,
    anon_sym_GT_GT_EQ,
    anon_sym_else,
    anon_sym_move,
    anon_sym_DOT,
    anon_sym_AT,
    sym_integer_literal,
    anon_sym_DQUOTE,
    anon_sym_DQUOTE,
    sym_char_literal,
    sym_escape_sequence,
    anon_sym_true,
    anon_sym_false,
    sym_line_comment,
    sym_self,
    sym_super,
    sym_crate,
    sym_metavariable,
    sym__string_content,
    sym_raw_string_literal,
    sym_float_literal,
    sym_block_comment,
    sym_source_file,
    sym__statement,
    sym_empty_statement,
    sym__expression_statement,
    sym_macro_definition,
    sym_macro_rule,
    sym__token_pattern,
    sym_token_tree_pattern,
    sym_token_binding_pattern,
    sym_token_repetition_pattern,
    sym_fragment_specifier,
    sym_token_tree,
    sym_token_repetition,
    sym_attribute_item,
    sym_inner_attribute_item,
    sym_meta_item,
    sym_meta_arguments,
    sym_mod_item,
    sym_foreign_mod_item,
    sym_declaration_list,
    sym_struct_item,
    sym_union_item,
    sym_enum_item,
    sym_enum_variant_list,
    sym_enum_variant,
    sym_field_declaration_list,
    sym_field_declaration,
    sym_ordered_field_declaration_list,
    sym_extern_crate_declaration,
    sym_const_item,
    sym_static_item,
    sym_type_item,
    sym_function_item,
    sym_function_signature_item,
    sym_function_modifiers,
    sym_where_clause,
    sym_where_predicate,
    sym_impl_item,
    sym_trait_item,
    sym_associated_type,
    sym_trait_bounds,
    sym_higher_ranked_trait_bound,
    sym_removed_trait_bound,
    sym_type_parameters,
    sym_const_parameter,
    sym_constrained_type_parameter,
    sym_optional_type_parameter,
    sym_let_declaration,
    sym_use_declaration,
    sym__use_clause,
    sym_scoped_use_list,
    sym_use_list,
    sym_use_as_clause,
    sym_use_wildcard,
    sym_parameters,
    sym_self_parameter,
    sym_variadic_parameter,
    sym_parameter,
    sym_extern_modifier,
    sym_visibility_modifier,
    sym__type,
    sym_bracketed_type,
    sym_qualified_type,
    sym_lifetime,
    sym_array_type,
    sym_for_lifetimes,
    sym_function_type,
    sym_tuple_type,
    sym_unit_type,
    sym_generic_function,
    sym_generic_type,
    sym_generic_type_with_turbofish,
    sym_bounded_type,
    sym_type_arguments,
    sym_type_binding,
    sym_reference_type,
    sym_pointer_type,
    sym_empty_type,
    sym_abstract_type,
    sym_dynamic_type,
    sym__expression,
    sym_macro_invocation,
    sym_scoped_identifier,
    sym_scoped_type_identifier,
    sym_scoped_type_identifier,
    sym_range_expression,
    sym_unary_expression,
    sym_try_expression,
    sym_reference_expression,
    sym_binary_expression,
    sym_assignment_expression,
    sym_compound_assignment_expr,
    sym_type_cast_expression,
    sym_return_expression,
    sym_call_expression,
    sym_arguments,
    sym_array_expression,
    sym_parenthesized_expression,
    sym_tuple_expression,
    sym_unit_expression,
    sym_struct_expression,
    sym_field_initializer_list,
    sym_shorthand_field_initializer,
    sym_field_initializer,
    sym_base_field_initializer,
    sym_if_expression,
    sym_if_let_expression,
    sym__else_tail,
    sym_match_expression,
    sym_match_block,
    sym_match_arm,
    sym_match_arm,
    sym_match_pattern,
    sym_while_expression,
    sym_while_let_expression,
    sym_loop_expression,
    sym_for_expression,
    sym_closure_expression,
    sym_closure_parameters,
    sym_loop_label,
    sym_break_expression,
    sym_continue_expression,
    sym_index_expression,
    sym_await_expression,
    sym_field_expression,
    sym_unsafe_block,
    sym_async_block,
    sym_block,
    sym__pattern,
    sym_tuple_pattern,
    sym_slice_pattern,
    sym_tuple_struct_pattern,
    sym_struct_pattern,
    sym_field_pattern,
    sym_remaining_field_pattern,
    sym_mut_pattern,
    sym_range_pattern,
    sym_ref_pattern,
    sym_captured_pattern,
    sym_reference_pattern,
    sym__literal,
    sym__literal_pattern,
    sym_negative_literal,
    sym_string_literal,
    sym_boolean_literal,
    aux_sym_source_file_repeat1,
    aux_sym_macro_definition_repeat1,
    aux_sym_token_tree_pattern_repeat1,
    aux_sym_token_tree_repeat1,
    aux_sym_meta_arguments_repeat1,
    aux_sym_declaration_list_repeat1,
    aux_sym_enum_variant_list_repeat1,
    aux_sym_enum_variant_list_repeat2,
    aux_sym_field_declaration_list_repeat1,
    aux_sym_ordered_field_declaration_list_repeat1,
    aux_sym_function_modifiers_repeat1,
    aux_sym_where_clause_repeat1,
    aux_sym_trait_bounds_repeat1,
    aux_sym_type_parameters_repeat1,
    aux_sym_use_list_repeat1,
    aux_sym_parameters_repeat1,
    aux_sym_for_lifetimes_repeat1,
    aux_sym_tuple_type_repeat1,
    aux_sym_type_arguments_repeat1,
    aux_sym_arguments_repeat1,
    aux_sym_array_expression_repeat1,
    aux_sym_tuple_expression_repeat1,
    aux_sym_field_initializer_list_repeat1,
    aux_sym_match_block_repeat1,
    aux_sym_match_pattern_repeat1,
    aux_sym_closure_parameters_repeat1,
    aux_sym_tuple_pattern_repeat1,
    aux_sym_struct_pattern_repeat1,
    aux_sym_string_literal_repeat1,
    alias_sym_field_identifier,
    alias_sym_primitive_type,
    alias_sym_shorthand_field_identifier,
    alias_sym_type_identifier,
];

// Initialized in run_static_initializers
static mut ts_symbol_metadata: [TSSymbolMetadata; 316] = [TSSymbolMetadata {
    visible: false,
    named: false,
}; 316];

pub const ts_field_names: [&'static [u8]; 29] = [
    b"",
    b"alias\x00",
    b"alternative\x00",
    b"argument\x00",
    b"arguments\x00",
    b"body\x00",
    b"bounds\x00",
    b"condition\x00",
    b"consequence\x00",
    b"default_type\x00",
    b"element\x00",
    b"field\x00",
    b"function\x00",
    b"left\x00",
    b"length\x00",
    b"list\x00",
    b"macro\x00",
    b"name\x00",
    b"operator\x00",
    b"parameters\x00",
    b"path\x00",
    b"pattern\x00",
    b"return_type\x00",
    b"right\x00",
    b"trait\x00",
    b"type\x00",
    b"type_arguments\x00",
    b"type_parameters\x00",
    b"value\x00",
];

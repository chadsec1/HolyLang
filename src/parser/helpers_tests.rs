use super::*;
use crate::consts;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR_NO_INFER
};

#[cfg(test)]
mod tests {
    use super::*;

    // find_constructor_bracket
    mod find_constructor_bracket {
        use super::*;
        // --- basic happy paths ---

        #[test]
        fn empty_string_returns_none() {
            assert_eq!(helpers::find_constructor_bracket(""), None);
        }

        #[test]
        fn no_brackets_returns_none() {
            assert_eq!(helpers::find_constructor_bracket("hello"), None);
        }

        #[test]
        fn single_empty_pair_is_suffix_not_constructor() {
            // "[]" alone is a type suffix, not a constructor bracket
            assert_eq!(helpers::find_constructor_bracket("[]"), None);
        }

        #[test]
        fn open_bracket_with_content_is_constructor() {
            // "[1, 2, 3]" — the '[' at 0 is NOT followed by ']'
            assert_eq!(helpers::find_constructor_bracket("[1, 2, 3]"), Some(0));
        }

        #[test]
        fn suffix_then_constructor() {
            // "int[][1, 2]" — first pair is suffix, second '[' is constructor
            let s = "int[][1, 2]";
            assert_eq!(helpers::find_constructor_bracket(s), Some(5));
        }

        #[test]
        fn multiple_suffix_pairs_before_constructor() {
            // "int[][][5]" — three '[', first two are suffix pairs, third is constructor
            let s = "int[][][5]";
            assert_eq!(helpers::find_constructor_bracket(s), Some(7));
        }

        #[test]
        fn only_suffix_pairs_returns_none() {
            // "int[][]" — both are suffix pairs
            assert_eq!(helpers::find_constructor_bracket("int[][]"), None);
        }

        #[test]
        fn constructor_at_start() {
            assert_eq!(helpers::find_constructor_bracket("["), Some(0));
        }

        #[test]
        fn unclosed_bracket_is_constructor() {
            // "[abc" — '[' at 0 has no matching ']'
            assert_eq!(helpers::find_constructor_bracket("[abc"), Some(0));
        }

        #[test]
        fn suffix_pair_then_unclosed() {
            // "[][" — first is suffix, second is constructor (no following ']')
            assert_eq!(helpers::find_constructor_bracket("[]["), Some(2));
        }

        #[test]
        fn open_bracket_at_very_end_is_constructor() {
            // "abc[" — '[' at end has no following char
            assert_eq!(helpers::find_constructor_bracket("abc["), Some(3));
        }

        #[test]
        fn interleaved_non_bracket_chars() {
            // "x[]y[z]" — '[]' at 1 is suffix, '[' at 4 has 'z]' after it
            let s = "x[]y[z]";
            assert_eq!(helpers::find_constructor_bracket(s), Some(4));
        }

        #[test]
        fn trailing_open_immediately_after_suffix() {
            // "[][" last '[' is not followed by ']'
            assert_eq!(helpers::find_constructor_bracket("[]["), Some(2));
        }

        // --- does NOT care about nested brackets; it's purely sequential ---
        #[test]
        fn nested_brackets_inner_is_found_first() {
            // "[[]]" — outer '[' at 0 is followed by '[', not ']', so it IS a constructor
            assert_eq!(helpers::find_constructor_bracket("[[]]"), Some(0));
        }
    }

    // find_top_level_op_any
    //
    mod find_top_level_op_any {
        use super::*;
        #[test]
        fn empty_string_returns_none() {
            assert_eq!(helpers::find_top_level_op_any("", &['+', '-']), None);
        }

        #[test]
        fn no_ops_in_string_returns_none() {
            assert_eq!(helpers::find_top_level_op_any("hello world", &['+', '-']), None);
        }

        #[test]
        fn single_addition() {
            let result = helpers::find_top_level_op_any("a+b", &['+', '-', '*', '/']);
            assert_eq!(result, Some((1, '+')));
        }

        #[test]
        fn single_subtraction() {
            let result = helpers::find_top_level_op_any("a-b", &['+', '-', '*', '/']);
            assert_eq!(result, Some((1, '-')));
        }

        #[test]
        fn single_multiplication() {
            let result = helpers::find_top_level_op_any("a*b", &['+', '-', '*', '/']);
            assert_eq!(result, Some((1, '*')));
        }

        #[test]
        fn op_inside_parens_is_skipped() {
            // "(a+b)" — '+' is depth-1, should be ignored
            let result = helpers::find_top_level_op_any("(a+b)", &['+', '-']);
            assert_eq!(result, None);
        }

        #[test]
        fn op_at_top_level_with_inner_parens() {
            // "(a+b)+(c+d)" — only the middle '+' is top-level
            let result = helpers::find_top_level_op_any("(a+b)+(c+d)", &['+']);
            assert_eq!(result, Some((5, '+')));
        }

        #[test]
        fn left_associativity_picks_rightmost_lowest_precedence() {
            // "a+b+c" — both '+' have the same precedence; should get the RIGHTMOST one
            let result = helpers::find_top_level_op_any("a+b+c", &['+', '-', '*', '/']);
            assert_eq!(result, Some((3, '+')));
        }

        #[test]
        fn lower_precedence_wins_over_higher() {
            // "a*b+c" — '+' (prec 2) wins over '*' (prec 3)
            let result = helpers::find_top_level_op_any("a*b+c", &['+', '-', '*', '/']);
            assert_eq!(result, Some((3, '+')));
        }

        #[test]
        fn multiply_wins_when_no_addition() {
            let result = helpers::find_top_level_op_any("a*b*c", &['+', '-', '*', '/']);
            // rightmost '*' at index 3
            assert_eq!(result, Some((3, '*')));
        }

        #[test]
        fn nested_parens_all_ops_hidden() {
            // "((a+b)*(c-d))" — all ops are nested
            let result = helpers::find_top_level_op_any("((a+b)*(c-d))", &['+', '-', '*', '/']);
            assert_eq!(result, None);
        }

        #[test]
        fn comparison_operators_have_lowest_precedence() {
            // "a+b>c*d" — '>' has prec 1, '+' has prec 2, so '>' wins
            let result = helpers::find_top_level_op_any("a+b>c*d", &['+', '-', '*', '/', '>']);
            assert_eq!(result, Some((3, '>')));
        }

        #[test]
        fn only_ops_in_search_list_are_found() {
            // '+' exists but we only search for '*'
            let result = helpers::find_top_level_op_any("a+b", &['*']);
            assert_eq!(result, None);
        }

        #[test]
        fn deeply_nested_op_is_hidden() {
            let result = helpers::find_top_level_op_any("(((a+b)))", &['+']);
            assert_eq!(result, None);
        }

        #[test]
        fn unbalanced_close_paren_depth_saturates_at_zero() {
            // "a)+b" — after ')' depth would go negative but is clamped; '+' at 2 is top-level
            let result = helpers::find_top_level_op_any("a)+b", &['+']);
            assert_eq!(result, Some((2, '+')));
        }

        #[test]
        fn op_immediately_at_start() {
            let result = helpers::find_top_level_op_any("+b", &['+']);
            assert_eq!(result, Some((0, '+')));
        }

        #[test]
        fn op_immediately_at_end() {
            let result = helpers::find_top_level_op_any("a+", &['+']);
            assert_eq!(result, Some((1, '+')));
        }
    }

    // split_comma_top_level
    mod split_comma_top_level {
        use super::*;
        // --- happy paths ---

        #[test]
        fn empty_string_gives_one_empty_part() {
            let result = helpers::split_comma_top_level("").unwrap();
            assert_eq!(result, vec![""]);
        }

        #[test]
        fn single_arg_no_comma() {
            let result = helpers::split_comma_top_level("hello").unwrap();
            assert_eq!(result, vec!["hello"]);
        }

        #[test]
        fn two_simple_args() {
            let result = helpers::split_comma_top_level("a, b").unwrap();
            assert_eq!(result, vec!["a", "b"]);
        }

        #[test]
        fn three_args_with_whitespace() {
            let result = helpers::split_comma_top_level("  x ,  y ,  z  ").unwrap();
            assert_eq!(result, vec!["x", "y", "z"]);
        }

        #[test]
        fn nested_parens_hide_comma() {
            let result = helpers::split_comma_top_level("f(a, b), c").unwrap();
            assert_eq!(result, vec!["f(a, b)", "c"]);
        }

        #[test]
        fn nested_square_brackets_hide_comma() {
            let result = helpers::split_comma_top_level("[1, 2], [3, 4]").unwrap();
            assert_eq!(result, vec!["[1, 2]", "[3, 4]"]);
        }

        #[test]
        fn nested_curly_braces_hide_comma() {
            let result = helpers::split_comma_top_level("{a: 1, b: 2}, c").unwrap();
            assert_eq!(result, vec!["{a: 1, b: 2}", "c"]);
        }

        #[test]
        fn double_quoted_string_hides_comma() {
            let result = helpers::split_comma_top_level(r#""hello, world", x"#).unwrap();
            assert_eq!(result, vec![r#""hello, world""#, "x"]);
        }

        #[test]
        fn single_quoted_string_hides_comma() {
            let result = helpers::split_comma_top_level("'a, b', c").unwrap();
            assert_eq!(result, vec!["'a, b'", "c"]);
        }

        #[test]
        fn backslash_escaped_quote_inside_string() {
            // The comma after the escaped quote should NOT split
            let result = helpers::split_comma_top_level(r#""he said \"hi, there\"", x"#).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[1], "x");
        }

        #[test]
        fn backslash_at_end_of_string_is_error() {
            // Trailing backslash outside string is invalid
            let result = helpers::split_comma_top_level(r#""abc\"#);
            assert!(result.is_err(), "expected error for unclosed string");
        }

        #[test]
        fn unclosed_double_quote_is_error() {
            let result = helpers::split_comma_top_level(r#""unclosed"#);
            assert!(result.is_err());
        }

        #[test]
        fn adjacent_string_literals_are_error() {
            // Two string literals back-to-back with no separator
            let result = helpers::split_comma_top_level(r#""a""b""#);
            assert!(result.is_err(), "adjacent string literals should be an error");
        }

        #[test]
        fn comma_only_gives_two_empty_parts() {
            let result = helpers::split_comma_top_level(",").unwrap();
            assert_eq!(result, vec!["", ""]);
        }

        #[test]
        fn trailing_comma_gives_empty_last_part() {
            let result = helpers::split_comma_top_level("a,").unwrap();
            assert_eq!(result, vec!["a", ""]);
        }

        #[test]
        fn leading_comma_gives_empty_first_part() {
            let result = helpers::split_comma_top_level(",a").unwrap();
            assert_eq!(result, vec!["", "a"]);
        }

        #[test]
        fn deeply_nested_structure() {
            let result = helpers::split_comma_top_level("f(g(h(1, 2), 3), 4), 5").unwrap();
            assert_eq!(result, vec!["f(g(h(1, 2), 3), 4)", "5"]);
        }

        #[test]
        fn mixed_bracket_types_nested() {
            let result = helpers::split_comma_top_level("([{a, b}]), c").unwrap();
            assert_eq!(result, vec!["([{a, b}])", "c"]);
        }

        #[test]
        fn string_with_escaped_backslash() {
            // "\\" is a single backslash — the quote after it should close the string
            let s = r#""back\\slash", next"#;
            let result = helpers::split_comma_top_level(s).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[1], "next");
        }

        #[test]
        fn empty_string_literal_as_arg() {
            let result = helpers::split_comma_top_level(r#""", x"#).unwrap();
            assert_eq!(result.len(), 2);
        }

        #[test]
        fn whitespace_only_parts_are_trimmed() {
            let result = helpers::split_comma_top_level("  ,  ").unwrap();
            assert_eq!(result, vec!["", ""]);
        }

        #[test]
        fn unicode_in_args() {
            let result = helpers::split_comma_top_level("héllo, wörld").unwrap();
            assert_eq!(result, vec!["héllo", "wörld"]);
        }

        #[test]
        fn single_item() {
            let result = helpers::split_comma_top_level("a").unwrap();
            assert_eq!(result, vec!["a"]);
        }

        #[test]
        fn multiple_items() {
            let result = helpers::split_comma_top_level("a, b, c").unwrap();
            assert_eq!(result, vec!["a", "b", "c"]);
        }

        #[test]
        fn nested_parens_not_split() {
            let result = helpers::split_comma_top_level("foo(a, b), c").unwrap();
            assert_eq!(result, vec!["foo(a, b)", "c"]);
        }

        #[test]
        fn nested_brackets_not_split() {
            for t in ALL_TYPES_NO_ARR_NO_INFER {
                let f = format!("{}[1, 2], {}[3, 4]", t.clone(), t.clone());
                let result = helpers::split_comma_top_level(&f).unwrap();
                assert_eq!(result, vec![format!("{}[1, 2]", t), format!("{}[3, 4]", t)]);
            }


            for t in ALL_TYPES_NO_ARR_NO_INFER {
                let f = format!("{}[\"Hi\", \"There\"], {}[\"Lol\", \"xD\"]", t.clone(), t.clone());
                let result = helpers::split_comma_top_level(&f).unwrap();
                assert_eq!(result, vec![format!("{}[\"Hi\", \"There\"]", t), format!("{}[\"Lol\", \"xD\"]", t)]);
            }
            
            for t in ALL_TYPES_NO_ARR_NO_INFER {
                let f = format!("{}[\"Hi,!!\", \"The,re\"], {}[\"Lo, l!\", \", xD\"]", t.clone(), t.clone());
                let result = helpers::split_comma_top_level(&f).unwrap();
                assert_eq!(result, vec![format!("{}[\"Hi,!!\", \"The,re\"]", t), format!("{}[\"Lo, l!\", \", xD\"]", t)]);
            }
                

        }

        #[test]
        fn string_containing_comma() {
            let result = helpers::split_comma_top_level(r#""hello, world", b"#).unwrap();
            assert_eq!(result, vec![r#""hello, world""#, "b"]);
        }

        #[test]
        fn unclosed_string_errors() {
            assert!(helpers::split_comma_top_level(r#""unclosed, x"#).is_err());
        }
    }

    // strip_outer_quotes_and_unescape
    mod strip_outer_quotes_and_unescape {
        use super::*;

        #[test]
        fn empty_string_returns_empty() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape("").unwrap(), "");
        }

        #[test]
        fn plain_string_no_quotes_passthrough() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape("hello").unwrap(), "hello");
        }

        #[test]
        fn double_quoted_string_stripped() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""hello""#).unwrap(), "hello");
        }

        #[test]
        fn double_quote_at_start_only_errors() {
            assert!(helpers::strip_outer_quotes_and_unescape(r#""hello"#).is_err());
        }

        #[test]
        fn single_quote_at_end_only_errors() {
            assert!(helpers::strip_outer_quotes_and_unescape(r#"hello""#).is_err());
        }

        #[test]
        fn escape_newline() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\nb""#).unwrap(), "a\nb");
        }

        #[test]
        fn escape_carriage_return() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\rb""#).unwrap(), "a\rb");
        }

        #[test]
        fn escape_tab() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\tb""#).unwrap(), "a\tb");
        }

        #[test]
        fn escape_backslash() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\\b""#).unwrap(), r#"a\b"#);
        }

        #[test]
        fn escape_double_quote_inside() {
            // \" inside should produce a literal "
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\"b""#).unwrap(), r#"a"b"#);
        }

        #[test]
        fn escape_single_quote_inside() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\'b""#).unwrap(), "a'b");
        }

        #[test]
        fn escape_null_byte() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\0b""#).unwrap(), "a\0b");
        }

        #[test]
        fn unknown_escape_passes_through() {
            // \q → just 'q'
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""a\qb""#).unwrap(), "aqb");
        }

        #[test]
        fn trailing_backslash_in_unquoted_emits_backslash() {
            // Input doesn't have surrounding quotes; trailing \ emits \
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#"abc\"#).unwrap(), r#"abc\"#);
        }

        #[test]
        fn unescaped_inner_double_quote_is_error() {
            // A bare " in the middle of a quoted string is unterminated
            let result = helpers::strip_outer_quotes_and_unescape(r#""a"b""#);
            assert!(result.is_err(), "bare inner quote should be an error");
        }

        #[test]
        fn only_two_double_quotes_empty_content() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""""#).unwrap(), "");
        }

        #[test]
        fn multiple_escape_sequences_in_a_row() {
            assert_eq!(
                helpers::strip_outer_quotes_and_unescape(r#""\n\t\r\\""#).unwrap(),
                "\n\t\r\\"
            );
        }

        #[test]
        fn string_of_only_backslash_escape() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""\\""#).unwrap(), "\\");
        }

        #[test]
        fn unicode_content_preserved() {
            assert_eq!(helpers::strip_outer_quotes_and_unescape(r#""héllo""#).unwrap(), "héllo");
        }
    }

    // validate_identifier_name
    mod validate_identifier_name {
        use super::*;

        #[test]
        fn valid_simple_name() {
            assert!(helpers::validate_identifier_name("foo").is_ok());
        }

        #[test]
        fn valid_name_with_numbers() {
            assert!(helpers::validate_identifier_name("foo123").is_ok());
        }

        #[test]
        fn valid_name_with_underscore() {
            assert!(helpers::validate_identifier_name("foo_bar").is_ok());
        }

        #[test]
        fn valid_all_caps() {
            assert!(helpers::validate_identifier_name("FOO_BAR").is_ok());
        }

        #[test]
        fn valid_underscore_prefix() {
            assert!(helpers::validate_identifier_name("_private").is_ok());
        }

        #[test]
        fn valid_single_underscore() {
            assert!(helpers::validate_identifier_name("_").is_ok());
        }

        #[test]
        fn valid_single_letter() {
            assert!(helpers::validate_identifier_name("x").is_ok());
        }

        #[test]
        fn empty_name_is_error() {
            assert!(helpers::validate_identifier_name("").is_err());
        }

        #[test]
        fn starts_with_digit_is_error() {
            assert!(helpers::validate_identifier_name("1abc").is_err());
        }

        #[test]
        fn digit_only_is_error() {
            assert!(helpers::validate_identifier_name("123").is_err());
        }

        #[test]
        fn hyphen_is_invalid_char() {
            assert!(helpers::validate_identifier_name("foo-bar").is_err());
        }

        #[test]
        fn space_is_invalid_char() {
            assert!(helpers::validate_identifier_name("foo bar").is_err());
        }

        #[test]
        fn dot_is_invalid_char() {
            assert!(helpers::validate_identifier_name("foo.bar").is_err());
        }

        #[test]
        fn unicode_letter_is_invalid() {
            // Only ASCII alphanumeric + underscore are allowed
            assert!(helpers::validate_identifier_name("héllo").is_err());
        }

        #[test]
        fn reserved_keyword_own_is_error() {
            assert!(helpers::validate_identifier_name("own").is_err());
        }

        //
        
        #[test]
        fn identifier_valid() {
            let chars: Vec<char> = (b'0'..=b'9')
                .chain(b'A'..=b'Z')
                .chain(b'a'..=b'z')
                .map(|b| b as char)
                .collect();


            assert!(helpers::validate_identifier_name(&format!("_")).is_ok());

            for c in chars {
                assert!(helpers::validate_identifier_name(&format!("_{c}")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("_{c}_")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("foo{c}")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("FOO{c}")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("_{c}foo")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("f{c}oo")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("fo{c}o")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("foo_{c}")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("_{c}_foo")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("a{c}")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("a{c}3")).is_ok());
                assert!(helpers::validate_identifier_name(&format!("a2{c}")).is_ok());


            }

        }

        #[test]
        fn identifier_empty() {
            const MAX_SPACES: usize = 10000;
            let mut spaces = String::with_capacity(MAX_SPACES);
            for _ in 0..MAX_SPACES {
                assert!(helpers::validate_identifier_name(&spaces).is_err());
                spaces.push(' ');
            }
        }

        #[test]
        fn identifier_starts_with_digit() {
            for i in 0..100000 {
                assert!(helpers::validate_identifier_name(&format!("{i}foo")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{i}_foo_")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{i}foo_")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{i}_foo")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{i}_")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{i}")).is_err());
                
            }
            }

        #[test]
        fn identifier_invalid_chars() {
            // List of characters that validate_identifier_name should reject.
            //
            let failing_list: Vec<char> = (0u8..=127)
                .map(|b| b as char)
                .filter(|&c| !c.is_ascii_alphanumeric() && c != '_')
                .collect();

            for c in failing_list {
                assert!(helpers::validate_identifier_name(&format!("{c}foobar")).is_err());
                assert!(helpers::validate_identifier_name(&format!("f{c}oobar")).is_err());
                assert!(helpers::validate_identifier_name(&format!("fo{c}obar")).is_err());
                assert!(helpers::validate_identifier_name(&format!("foo{c}bar")).is_err());
                assert!(helpers::validate_identifier_name(&format!("foob{c}ar")).is_err());
                assert!(helpers::validate_identifier_name(&format!("fooba{c}r")).is_err());
                assert!(helpers::validate_identifier_name(&format!("foobar{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}{c}{c}{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}{c}{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}_{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}_")).is_err());
                assert!(helpers::validate_identifier_name(&format!("_{c}")).is_err());
                assert!(helpers::validate_identifier_name(&format!("{c}")).is_err());
            }
        }

        #[test]
        fn identifier_reserved_keywords() {
            for kw in consts::RESERVED_KEYWORDS { 
                assert!(
                    helpers::validate_identifier_name(kw).is_err(),
                    "Expected error for keyword `{}`", kw
                );


                assert!(
                    helpers::validate_identifier_name(&kw.to_uppercase()).is_err(),
                    "Expected error for keyword `{}`", &kw.to_uppercase()
                );

            }
        }
        
        #[test]
        fn name_containing_keyword_prefix_or_suffix_is_ok() {
            // key words should pass validation as long as they're not exact match.
            // i.e. "own owna" is valid.
            // "own aown" is valid, etc.

            let chars: Vec<char> = (b'A'..=b'Z')
                .chain(b'a'..=b'z')
                .map(|b| b as char)
                .collect();


            for kw in consts::RESERVED_KEYWORDS { 
                for c in &chars {
                    assert!(helpers::validate_identifier_name(&format!("{}{}", kw, c)).is_ok());

                    if !kw.to_lowercase().starts_with('u') && c.to_ascii_lowercase() == 'u' {
                        // So like an int32 doesnt become uint32, etc, which would error.
                        continue
                    }
                    assert!(helpers::validate_identifier_name(&format!("{}{}", c, kw)).is_ok());
                }
            }
        }

        #[test]
        fn single_digit_is_error() {
            assert!(helpers::validate_identifier_name("0").is_err());
        }

        #[test]
        fn at_sign_is_error() {
            assert!(helpers::validate_identifier_name("@foo").is_err());
        }

        #[test]
        fn dollar_sign_is_error() {
            assert!(helpers::validate_identifier_name("$foo").is_err());
        }

        #[test]
        fn valid_mixed_case_with_numbers_and_underscores() {
            assert!(helpers::validate_identifier_name("My_Var_2").is_ok());
        }

        #[test]
        fn name_with_only_numbers_after_underscore_is_ok() {
            assert!(helpers::validate_identifier_name("_123").is_ok());
        }
    }

    // strip_inline_comment
    mod strip_inline_comment {
        use super::*;
        #[test]
        fn no_comment_returns_original() {
            assert_eq!(helpers::strip_inline_comment("hello world"), "hello world");
        }

        #[test]
        fn empty_string_returns_empty() {
            assert_eq!(helpers::strip_inline_comment(""), "");
        }

        #[test]
        fn hash_at_start_strips_everything() {
            assert_eq!(helpers::strip_inline_comment("# this is a comment"), "");
        }

        #[test]
        fn hash_mid_line_strips_from_hash() {
            assert_eq!(helpers::strip_inline_comment("x = 1 # set x"), "x = 1");
        }

        #[test]
        fn trailing_whitespace_before_hash_is_trimmed() {
            assert_eq!(helpers::strip_inline_comment("x = 1   # set x"), "x = 1");
        }

        #[test]
        fn hash_inside_double_quoted_string_preserved() {
            let s = r##"x = "hello # world""##;
            assert_eq!(helpers::strip_inline_comment(s), s);
        }

        #[test]
        fn hash_inside_single_quoted_string_preserved() {
            let s = "x = 'hello # world'";
            assert_eq!(helpers::strip_inline_comment(s), s);
        }

        #[test]
        fn hash_after_closed_string_is_comment() {
            let s = r#"x = "hi" # comment"#;
            assert_eq!(helpers::strip_inline_comment(s), r#"x = "hi""#);
        }

        #[test]
        fn escaped_quote_inside_string_does_not_close_it() {
            // The # is still inside the string
            let s = r#"x = "say \"hi # there\"" # comment"#;
            let result = helpers::strip_inline_comment(s);
            // Everything up to the final `# comment` (outside the string) should remain
            assert!(result.ends_with(r#""say \"hi # there\"""#));
        }

        #[test]
        fn multiple_strings_then_comment() {
            let s = r#"print("a", "b") # done"#;
            assert_eq!(helpers::strip_inline_comment(s), r#"print("a", "b")"#);
        }

        #[test]
        fn only_a_hash_returns_empty() {
            assert_eq!(helpers::strip_inline_comment("#"), "");
        }

        #[test]
        fn hash_only_whitespace_before() {
            assert_eq!(helpers::strip_inline_comment("   # comment"), "");
        }

        #[test]
        fn no_trim_of_leading_whitespace() {
            // helpers::strip_inline_comment only trims the RIGHT side (trailing ws before #)
            // It should not remove leading whitespace
            assert_eq!(helpers::strip_inline_comment("   x = 1"), "   x = 1");
        }

        #[test]
        fn string_with_backslash_before_quote_escape_chain() {
            // "\\" then quote closes, then hash is a comment
            let s = r#""abc\\" # comment"#;
            let result = helpers::strip_inline_comment(s);
            assert_eq!(result, r#""abc\\""#);
        }

        #[test]
        fn no_comment_no_string() {
            assert_eq!(helpers::strip_inline_comment("x = 42"), "x = 42");
        }

        #[test]
        fn hash_in_both_string_and_outside() {
            let s = r##"x = "#hash" # real comment"##;
            // The first # is inside the string, the second is outside
            let result = helpers::strip_inline_comment(s);
            assert!(result.contains("#hash"), "in-string hash should be preserved");
            assert!(!result.contains("real comment"), "out-of-string comment should be stripped");
        }
    }

    // count_braces_outside_strings
    mod count_braces_outside_strings {
        use super::*;

        #[test]
        fn empty_line() {
            assert_eq!(helpers::count_braces_outside_strings(""), (0, 0));
        }

        #[test]
        fn no_braces() {
            assert_eq!(helpers::count_braces_outside_strings("hello world"), (0, 0));
        }

        #[test]
        fn one_open_one_close() {
            assert_eq!(helpers::count_braces_outside_strings("{}"), (1, 1));
        }

        #[test]
        fn multiple_pairs() {
            assert_eq!(helpers::count_braces_outside_strings("{}{}{}"), (3, 3));
        }

        #[test]
        fn only_opens() {
            assert_eq!(helpers::count_braces_outside_strings("{{{"), (3, 0));
        }

        #[test]
        fn only_closes() {
            assert_eq!(helpers::count_braces_outside_strings("}}}"), (0, 3));
        }

        #[test]
        fn braces_inside_double_quotes_not_counted() {
            assert_eq!(helpers::count_braces_outside_strings(r#""{{}}"#), (0, 0));
        }

        #[test]
        fn braces_inside_single_quotes_not_counted() {
            assert_eq!(helpers::count_braces_outside_strings("'{{}}'"), (0, 0));
        }

        #[test]
        fn braces_before_string_counted() {
            let s = r#"{ "{}" }"#;
            // outer '{' and '}' are outside the string; inner '{}' are inside
            assert_eq!(helpers::count_braces_outside_strings(s), (1, 1));
        }

        #[test]
        fn escaped_quote_does_not_close_string() {
            // The brace is still inside the string
            let s = r#""{\" { }""#;
            assert_eq!(helpers::count_braces_outside_strings(s), (0, 0));
        }

        #[test]
        fn backslash_backslash_then_quote_closes_string() {
            // "\\" is a literal backslash, so the next " closes the string
            // After the string, '{' is outside
            let s = r#""\\" { }"#;
            assert_eq!(helpers::count_braces_outside_strings(s), (1, 1));
        }

        #[test]
        fn mixed_outside_and_inside_braces() {
            // "{ \"inside: {}\" }" — outer braces count, inner two (in string) do not
            let s = r#"{ "inside: {}" }"#;
            assert_eq!(helpers::count_braces_outside_strings(s), (1, 1));
        }

        #[test]
        fn single_quoted_string_with_braces_then_outer_brace() {
            let s = "'{' { }";
            // '{' in single quotes is inside string; outer {} are not
            assert_eq!(helpers::count_braces_outside_strings(s), (1, 1));
        }

        #[test]
        fn interleaved_strings_and_braces() {
            // { "open{ " } "close}" {
            let s = r#"{ "open{ " } "close}" {"#;
            // Outside: first '{' (pos 0), '}' (pos 10), last '{' (pos 22)
            assert_eq!(helpers::count_braces_outside_strings(s), (2, 1));
        }

        #[test]
        fn square_brackets_and_parens_do_not_affect_count() {
            assert_eq!(helpers::count_braces_outside_strings("([])"), (0, 0));
        }
    }

    // parse_format_string
    mod parse_format_string {
        use super::*;
        #[test]
        fn empty_string() {
            let (template, exprs) = helpers::parse_format_string("").unwrap();
            assert_eq!(template, "");
            assert!(exprs.is_empty());
        }

        #[test]
        fn plain_text_no_placeholders() {
            let (template, exprs) = helpers::parse_format_string("hello world").unwrap();
            assert_eq!(template, "hello world");
            assert!(exprs.is_empty());
        }

        #[test]
        fn single_placeholder() {
            let (template, exprs) = helpers::parse_format_string("hello {name}").unwrap();
            assert_eq!(template, "hello {}");
            assert_eq!(exprs, vec!["name"]);
        }

        #[test]
        fn multiple_placeholders() {
            let (template, exprs) = helpers::parse_format_string("{a} + {b} = {c}").unwrap();
            assert_eq!(template, "{} + {} = {}");
            assert_eq!(exprs, vec!["a", "b", "c"]);
        }

        #[test]
        fn literal_double_open_brace() {
            let (template, exprs) = helpers::parse_format_string("{{").unwrap();
            assert_eq!(template, "{{");
            assert!(exprs.is_empty());
        }

        #[test]
        fn literal_double_close_brace() {
            let (template, exprs) = helpers::parse_format_string("}}").unwrap();
            assert_eq!(template, "}}");
            assert!(exprs.is_empty());
        }

        #[test]
        fn literal_braces_mixed_with_placeholder() {
            let (template, exprs) = helpers::parse_format_string("{{{x}}}").unwrap();
            // "{{" is a literal "{", "{x}" is a placeholder, "}}" is a literal "}"
            assert_eq!(template, "{{{}}}");
            assert_eq!(exprs, vec!["x"]);
        }

        #[test]
        fn unclosed_brace_is_error() {
            assert!(helpers::parse_format_string("{unclosed").is_err());
        }

        #[test]
        fn unmatched_close_brace_is_error() {
            assert!(helpers::parse_format_string("hello}world").is_err());
        }

        #[test]
        fn empty_placeholder_is_error() {
            assert!(helpers::parse_format_string("{}").is_err(), "empty {{}} placeholders should've errored.");
        }

        #[test]
        fn placeholder_with_expression() {
            let (template, exprs) = helpers::parse_format_string("{x + y}").unwrap();
            assert_eq!(template, "{}");
            assert_eq!(exprs, vec!["x + y"]);
        }

        #[test]
        fn placeholder_with_function_call() {
            let (template, exprs) = helpers::parse_format_string("result: {to_string(x)}").unwrap();
            assert_eq!(template, "result: {}");
            assert_eq!(exprs, vec!["to_string(x)"]);
        }

        #[test]
        fn adjacent_placeholders_no_space() {
            let (template, exprs) = helpers::parse_format_string("{a}{b}").unwrap();
            assert_eq!(template, "{}{}");
            assert_eq!(exprs, vec!["a", "b"]);
        }

        #[test]
        fn literal_braces_around_placeholder() {
            // "{{ {x} }}" should be "{ {} }"
            let (template, exprs) = helpers::parse_format_string("{{ {x} }}").unwrap();
            assert_eq!(template, "{{ {} }}");
            assert_eq!(exprs, vec!["x"]);
        }

        #[test]
        fn single_open_brace_at_end_is_ror() {
            assert!(helpers::parse_format_string("hello {").is_err());
        }

        #[test]
        fn single_close_brace_at_start_is_error() {
            assert!(helpers::parse_format_string("}hello").is_err());
        }

        #[test]
        fn placeholder_at_start_of_string() {
            let (template, exprs) = helpers::parse_format_string("{x} world").unwrap();
            assert_eq!(template, "{} world");
            assert_eq!(exprs, vec!["x"]);
        }

        #[test]
        fn placeholder_at_end_of_string() {
            let (template, exprs) = helpers::parse_format_string("hello {x}").unwrap();
            assert_eq!(template, "hello {}");
            assert_eq!(exprs, vec!["x"]);
        }

        #[test]
        fn many_placeholders_order_preserved() {
            let (template, exprs) = helpers::parse_format_string("{z}{y}{x}").unwrap();
            assert_eq!(template, "{}{}{}");
            assert_eq!(exprs, vec!["z", "y", "x"]);
        }

        #[test]
        fn plain_text_before_and_after_literal_braces() {
            let (template, exprs) = helpers::parse_format_string("a{{b}}c").unwrap();
            assert_eq!(template, "a{{b}}c");
            assert!(exprs.is_empty());
        }

        #[test]
        fn placeholder_containing_closing_double_brace() {
            // "{x}}}" is placeholder + literal close }}
            let (template, exprs) = helpers::parse_format_string("{x}}}").unwrap();
            assert_eq!(template, "{}}}");
            assert_eq!(exprs, vec!["x"]);
        }

        #[test]
        fn unicode_content_in_placeholder() {
            let (template, exprs) = helpers::parse_format_string("{héllo}").unwrap();
            assert_eq!(template, "{}");
            assert_eq!(exprs, vec!["héllo"]);
        }

        #[test]
        fn whitespace_only_placeholder_is_captured() {
            // Whitespace is NOT empty, so it should not error even if semantically weird
            let result = helpers::parse_format_string("{   }");
            // The implementation rejects empty inner string; "   " is not empty
            match result {
                Ok((_, exprs)) => assert_eq!(exprs, vec!["   "]),
                Err(_) => {} // also acceptable if impl trims and treats as empty
            }
        }
    }

    // is_array_type — trivial but document the contract
    mod is_array_type {
        use super::*;

        #[test]
        fn array_type_is_true() {
            let t = Type::Array(Box::new(Type::Int8)); 
            assert!(helpers::is_array_type(&t));
        }

        #[test]
        fn non_array_types_are_false() {
            assert!(!helpers::is_array_type(&Type::Int8));
            assert!(!helpers::is_array_type(&Type::Bool));
            assert!(!helpers::is_array_type(&Type::String));
        }

        #[test]
        fn nested_array_is_still_array() {
            let t = Type::Array(Box::new(Type::Array(Box::new(Type::Int8))));
            assert!(helpers::is_array_type(&t));
        }
    }

    // Cross-cutting / regression tests
    mod cross_cutting {
        use super::*;
        /// A format string that itself contains the output of split_comma_top_level args
        #[test]
        fn format_string_placeholder_is_valid_identifier() {
            let (_, exprs) = helpers::parse_format_string("{my_var}").unwrap();
            assert_eq!(exprs.len(), 1);
            assert!(helpers::validate_identifier_name(&exprs[0]).is_ok());
        }

        /// A comment stripped line fed to split_comma_top_level
        #[test]
        fn stripped_comment_then_split() {
            let raw = "a, b, c # trailing comment";
            let stripped = helpers::strip_inline_comment(raw);
            let parts = helpers::split_comma_top_level(&stripped).unwrap();
            assert_eq!(parts, vec!["a", "b", "c"]);
        }

        /// Brace counting on a format-string template
        #[test]
        fn brace_balance_of_format_template() {
            let (template, _) = helpers::parse_format_string("{x} + {y}").unwrap();
            let (opens, closes) = helpers::count_braces_outside_strings(&template);
            // "{} + {}" has 2 opens and 2 closes (all outside strings)
            assert_eq!(opens, closes);
        }

        /// Identifier used as constructor bracket argument
        #[test]
        fn constructor_bracket_arg_is_valid_identifier() {
            let input = "int[my_len]";
            let idx = helpers::find_constructor_bracket(input).unwrap();
            let inner = &input[idx + 1..input.len() - 1];
            assert!(helpers::validate_identifier_name(inner).is_ok());
        }

        /// helpers::strip_inline_comment followed by unescape should be consistent
        #[test]
        fn comment_inside_string_not_stripped_then_unescaped() {
            let raw = r#""hello # world""#;
            let stripped = helpers::strip_inline_comment(raw);
            // The # is inside the string, so nothing should be stripped
            assert_eq!(stripped, raw);
            let unescaped = helpers::strip_outer_quotes_and_unescape(&stripped).unwrap();
            assert_eq!(unescaped, "hello # world");
        }
    }
}


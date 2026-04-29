use super::*;

#[cfg(test)]
mod bin_op_tests {
    use super::*;

    #[test]
    fn binop_arth_signed_literals_only() {
        let edge_cases_numbers = [
            i8::MIN as i128, i8::MAX as i128, 
            i16::MIN as i128, i16::MAX as i128, 
            i32::MIN as i128, i32::MAX as i128, 
            i64::MIN as i128, i64::MAX as i128, 
            i128::MIN, i128::MAX, 
        ];


        let edge_cases_types = [
            Type::Int8, Type::Int8,
            Type::Int16, Type::Int16,
            Type::Int32, Type::Int32,
            Type::Int64, Type::Int64,
            Type::Int128, Type::Int128,
        ];


        for (en1, et1) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (en2, et2) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = {} {} {}", et2, en1, s, en2));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, et2.clone());
                        if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                            assert_eq!(op, b);

                            if let Expr::IntLiteral { value, .. } = **left {
                                assert_eq!(value.get_type(), et1.clone());

                                if !value.is_signed() {
                                    panic!("We are in a signed testing function, but value is unsigned: {:?}", **left);
                                }

                                assert_eq!(value.as_i128(), *en1);

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }


                            if let Expr::IntLiteral { value, .. } = **right {
                                assert_eq!(value.get_type(), et2.clone());

                                if !value.is_signed() {
                                    panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                                }

                                assert_eq!(value.as_i128(), *en2);

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }


                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }



    #[test]
    fn binop_arth_unsigned_literals_only() {
        let edge_cases_numbers = [
            u8::MIN as u128, u8::MAX as u128, 
            u16::MIN as u128, u16::MAX as u128, 
            u32::MIN as u128, u32::MAX as u128, 
            u64::MIN as u128, u64::MAX as u128, 
            u128::MIN, u128::MAX, 

            usize::MIN as u128, usize::MAX as u128
        ];


        // Because we default to signed integers literals, unless we go out of range, then we switch to
        // unsigned literals types.
        // so those expected types are correct.
        // Int8 because unsigned::MIN is always 0, which can fit into int8
        //
        // I hope this test is not too much voodo for the reader, but it is what it is. It's good,
        // it's correct, it works, and it catches most parser edge cases.
        let edge_cases_types = [
            Type::Int8, Type::Int16,
            Type::Int8, Type::Int32,
            Type::Int8, Type::Int64,
            Type::Int8, Type::Int128,
            Type::Int8, Type::Uint128,
            
            Type::Int8, Type::Int128,
        ];

        for (en1, et1) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (en2, et2) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = {} {} {}", et2, en1, s, en2));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, et2.clone());
                        if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                            assert_eq!(op, b);

                            if let Expr::IntLiteral { value, .. } = **left {
                                assert_eq!(value.get_type(), et1.clone());

                                if value.is_signed() {
                                    assert!(value.as_i128() >= 0);
                                    assert_eq!(value.as_i128() as u128, *en1);
                                } else {
                                    assert_eq!(value.as_u128(), *en1);
                                } 

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }


                            if let Expr::IntLiteral { value, .. } = **right {
                                assert_eq!(value.get_type(), et2.clone());


                                if value.is_signed() {
                                    assert!(value.as_i128() >= 0);
                                    assert_eq!(value.as_i128() as u128, *en2);
                                } else {
                                    assert_eq!(value.as_u128(), *en2);
                                } 

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }


                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }





    #[test]
    fn binop_arth_vars_only() {
        for t in ALL_TYPES_NO_ARR {
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} b", t, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));
                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    // Signed integer literals
    #[test]
    fn binop_arth_vars_and_signed_integer_literals_mixed() {
        let edge_cases_numbers = [
            i8::MIN as i128, i8::MAX as i128, 
            i16::MIN as i128, i16::MAX as i128, 
            i32::MIN as i128, i32::MAX as i128, 
            i64::MIN as i128, i64::MAX as i128, 
            i128::MIN, i128::MAX, 
        ];


        let edge_cases_types = [
            Type::Int8, Type::Int8,
            Type::Int16, Type::Int16,
            Type::Int32, Type::Int32,
            Type::Int64, Type::Int64,
            Type::Int128, Type::Int128,
        ];


        for (en, et) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} {}", et, s, en));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));
                        if let Expr::IntLiteral { value, .. } = **right {
                            assert_eq!(value.get_type(), et.clone());

                            if !value.is_signed() {
                                panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                            }

                            assert_eq!(value.as_i128(), *en);

                        } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }


            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = {} {} a", et, en, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert_eq!(value.get_type(), et.clone());
                            if !value.is_signed() {
                                panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                            }

                            assert_eq!(value.as_i128(), *en);
                        } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }

                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    // Unsigned integer literals
    #[test]
    fn binop_arth_vars_and_unsigned_integer_literals_mixed() {
        let edge_cases_numbers = [
            u8::MIN as u128, u8::MAX as u128, 
            u16::MIN as u128, u16::MAX as u128, 
            u32::MIN as u128, u32::MAX as u128, 
            u64::MIN as u128, u64::MAX as u128, 
            u128::MIN, u128::MAX, 

            usize::MIN as u128, usize::MAX as u128
        ];


        // Because we default to signed integers literals, unless we go out of range, then we switch to
        // unsigned literals types.
        // so those expected types are correct.
        // Int8 because unsigned::MIN is always 0, which can fit into int8
        //
        // I hope this test is not too much voodo for the reader, but it is what it is. It's good,
        // it's correct, it works, and it catches most parser edge cases.
        let edge_cases_types = [
            Type::Int8, Type::Int16,
            Type::Int8, Type::Int32,
            Type::Int8, Type::Int64,
            Type::Int8, Type::Int128,
            Type::Int8, Type::Uint128,
            
            Type::Int8, Type::Int128,
        ];


        for (en, et) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} {}", et, s, en));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert_eq!(value.get_type(), et.clone());

                            if value.is_signed() {
                                assert!(value.as_i128() >= 0);
                                assert_eq!(value.as_i128() as u128, *en);
                            } else {
                                assert_eq!(value.as_u128(), *en);
                            }   
                            
                        } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }


            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = {} {} a", et, en, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert_eq!(value.get_type(), et.clone());

                            if value.is_signed() {
                                assert!(value.as_i128() >= 0);
                                assert_eq!(value.as_i128() as u128, *en);
                            } else {
                                assert_eq!(value.as_u128(), *en);
                            }   

                        } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }

                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    #[test]
    fn binop_missing_right_operand_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for b in BIN_OP_KIND_SYMBOLS {
                    assert_parse_err(&wrap(&format!("own x {} = {} {}", t, l, b)));
                }
            }
        }
    }

    #[test]
    fn binop_missing_left_operand_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for b in BIN_OP_KIND_SYMBOLS {
                    // So it's not unary, like -1, which is correct and wouldn't error.
                    if b == "-" {
                        continue
                    }
                    assert_parse_err(&wrap(&format!("own x {} = {} {}", t, b, l)));
                }
            }
        }
    }

    #[test]
    fn binop_nested_via_parens() {
        // e.g. own x int32 = (1 + 1) + 4
        // .. etc
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = ({} {} {}) {} {}", t, l, s, l, s, l));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, t.clone());
                        if let Some(Expr::BinOp { op, left, .. }) = &v.value {
                            assert_eq!(op, b);
                            assert!(matches!(**left, Expr::BinOp { .. }));
                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }

}

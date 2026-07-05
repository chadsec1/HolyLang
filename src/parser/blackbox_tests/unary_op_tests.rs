use super::*;

#[cfg(test)]
mod unary_negate_tests {
    use super::*;

    #[test]
    fn int_literals_doesnt_produce_unary_negate() {
        let literals_ints_edge_cases = get_all_ints_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_ints_edge_cases {
                // Because if l is -, it would become a float
                if *l == u128::MAX.to_string() {
                    continue
                }

                let stmts = parse_body(&format!("own x {} = -{}", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
            
                    assert_eq!(v.name, "x");
                    if v.type_name != Type::Char {
                        assert_ne!(v.type_name.get_default_value(span()).unwrap(), v.value);
                    }

                    // a negative value would produce unary though because - earlier makes it do
                    // so. since theres no expressions before it. (--1 is -1 negated, etc)
                    if l.starts_with("-") {
                        if let Expr::UnaryOp { op, expr, .. } = &v.value {
                            assert_eq!(*op, UnaryOpKind::Negate);
                            if let Expr::IntLiteral { value, .. } = &**expr {
                                if value.is_signed() {
                                    let val_l = value.as_i128();
                                    assert_eq!(&val_l.to_string(), l);
                                } else {
                                    let val_l = value.as_u128();
                                    assert_eq!(&val_l.to_string(), l);
                                }   

                            } else { panic!("Expected IntLiteral"); }
                        } else { panic!("Expected Unary negate"); }
                    
                    } else {
                        if let Expr::IntLiteral { value, .. } = v.value {
                            if value.is_signed() {
                                let val_l = value.as_i128();
                                if val_l == 0 {
                                    assert_eq!(&val_l.to_string(), l);
                                } else {
                                    assert_eq!(val_l.to_string(), format!("-{}", l));
                                }
                            } else {
                                let val_l = value.as_u128();
                                assert_eq!(&val_l.to_string(), l);
                            }
                        } else { panic!("Expected IntLiteral"); }
                    }


                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn unary_negate_variable() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = -y", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());

                assert_eq!(v.name, "x");

                if v.type_name != Type::Char {
                    assert_ne!(v.type_name.get_default_value(span()).unwrap(), v.value);
                }

                if let Expr::UnaryOp { op, expr, .. } = &v.value {
                    assert_eq!(*op, UnaryOpKind::Negate);
                    assert!(matches!(**expr, Expr::Var { .. }));
                } else { panic!("Expected Unary negate"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn unary_negate_array_access() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = -y[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());

                    assert_eq!(v.name, "x");
                    if v.type_name != Type::Char {
                        assert_ne!(v.type_name.get_default_value(span()).unwrap(), v.value);
                    }

                    if let Expr::UnaryOp { op, expr, .. } = &v.value {
                        assert_eq!(*op, UnaryOpKind::Negate);
                        assert!(matches!(**expr, Expr::ArrayAccess { .. }));
                    } else { panic!("Expected Unary negate"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }



    #[test]
    fn unary_negate_dangling_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = -", t)));
        }
    }
}

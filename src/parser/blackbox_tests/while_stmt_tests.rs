use super::*;

#[cfg(test)]
mod while_stmt_in_function_tests {
    use super::*;

    #[test]
    fn while_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("while {} {{\n\n{}}}", l, l)));    
            assert_parse_err(&wrap(&format!("while range({}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("while range(, {}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("while range({}, ) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("while range({}, {}) {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("while range({}, {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("while range{}, {} {{\n\n}}", l, l)));    


            assert_parse_err(&wrap(&format!("while {} in {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("while in {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("while {} in {{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("while{} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{} while {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}while {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", l, l)));    
        }

        assert_parse_err(&wrap("while {\n\n"));    
        assert_parse_err(&wrap("while {}"));    
        assert_parse_err(&wrap("while \n\n}"));    
        assert_parse_err(&wrap("while {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if kw == &"true" || kw == &"false" {
                continue 
            }
            assert_parse_err(&wrap(&format!("while {} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("while {} {{\n\n}}", kw.to_uppercase())));
        }

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("while{} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{} while {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}while {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", kw, kw)));    
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", kw, kw)));    

            assert_parse_err(&wrap(&format!("while{} {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} while {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}while {{\n\n}}", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", kw.to_uppercase(), kw)));    
            
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", kw.to_uppercase(), kw)));    

        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("while {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("while{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{} while {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}while {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", t, t)));    
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", t, t)));    


            assert_parse_err(&wrap(&format!("while {} {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("while{} {{\n\n}}", t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} while {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}while {{\n\n}}", t.to_string().to_uppercase() )));

            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{}while{} {{\n\n}}", t, t.to_string().to_uppercase()))); 

            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} while {} {{\n\n}}", t.to_string().to_uppercase(), t)));

        }
    }

    #[test]
    fn while_statements_trailing_exprs_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            assert_parse_err(&wrap(&format!("while {} {{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {} {{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {} {{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {} {{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n", l, l)));

            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n", l, l)));
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n", l, l)));
        }
    }

    #[test]
    fn while_statements_trailing_kw_errors() {
        let literals = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for l in &literals {
                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n", l, kw)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n", l, kw)));




                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {} {} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{} {}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{}{{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{}{{\n\n", l, kw, l)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{}\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{}\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{}\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{}\n\n", l, kw, l)));
            }
        }
    }

    #[test]
    fn while_statements_trailing_types_errors() {
        let literals = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {} {{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {} {{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}{{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}{{\n\n", l, t)));    

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n}}", l, t)));

                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}} {}\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("while {}{{\n\n}}{}\n\n", l, t)));
            }
        }
    }




    #[test]
    fn while_statements_all_literals() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while {} {} {} {{\n\n}}", l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
        }
    }

    #[test]
    fn while_statements_below_var_decl_with_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own x {} = {}\nwhile {} {} {} {{\n\n}}", t, l, l, s, l));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::While(w) = &stmts[1] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else {
                        panic!("expected while statement");
                    }
                }
            }
        }
    }


    #[test]
    fn while_statements_below_var_decl_without_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own x {}\nwhile {} {} {} {{\n\n}}", t, l, s, l));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::While(w) = &stmts[1] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else {
                        panic!("expected while statement");
                    }
                }
            }
        }
    }


    #[test]
    fn while_statements_after_var_decl_with_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("while {} {} {} {{\n\n}}\nown x {} = {}", l, s, l, t, l));
                    assert_eq!(stmts.len(), 2);
                    if let Stmt::While(w) = &stmts[0] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else { panic!("expected while statement");}

                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }


                }
            }
        }
    }

    #[test]
    fn while_statements_after_var_decl_without_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("while {} {} {} {{\n\n}}\nown x {}", l, s, l, t));
                    assert_eq!(stmts.len(), 2);
                    if let Stmt::While(w) = &stmts[0] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else { panic!("expected while statement");}

                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }


                }
            }
        }
    }


    #[test]
    fn while_statements_before_expr() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("{}\nwhile {} {} {} {{\n\n}}", l, l, s, l));
                assert_eq!(stmts.len(), 2);
                assert!(matches!(stmts[0], Stmt::Expr(_)));

                if let Stmt::While(w) = &stmts[1] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else { panic!("expected while statement");}
            }
        }
    }


    #[test]
    fn while_statements_after_expr() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while {} {} {} {{\n\n}}\n{}", l, s, l, l));
                assert_eq!(stmts.len(), 2);

                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else { panic!("expected while statement");}
                
                assert!(matches!(stmts[1], Stmt::Expr(_)));
            }
        }
    }





    
    #[test]
    fn while_statements_int_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 1 {} 2 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }

    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_int_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 1 {} 2 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_int_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 1 {} 2 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    

    #[test]
    fn while_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }

    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 69 {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(69)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} 67 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(67)));
                    } else { panic!(); }
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 69 {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} 67 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while -69 {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(-69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} 67 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}", spaces)));
            spaces.push(' ');
        }
    }

}



//
//
//
//
//


#[cfg(test)]
mod while_stmt_in_global_tests {
    use super::*;

    #[test]
    fn while_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for l in &literals_edge_cases {
            assert_parse_err(&format!("while range({}) {{\n\n}}", l));    
            assert_parse_err(&format!("while range(, {}) {{\n\n}}", l));    
            assert_parse_err(&format!("while range({}, ) {{\n\n}}", l));    
            assert_parse_err(&format!("while range({}, {}) {{\n\n}}", l, l));    
            assert_parse_err(&format!("while range({}, {} {{\n\n}}", l, l));    
            assert_parse_err(&format!("while range{}, {} {{\n\n}}", l, l));    


            assert_parse_err(&format!("while {} in {} {{\n\n}}", l, l));    
            assert_parse_err(&format!("while in {} {{\n\n}}", l));    
            assert_parse_err(&format!("while {} in {{\n\n}}", l));

            assert_parse_err(&format!("while{} {{\n\n}}", l));    
            assert_parse_err(&format!("{} while {{\n\n}}", l));    
            assert_parse_err(&format!("{}while {{\n\n}}", l));    
            assert_parse_err(&format!("{}while{} {{\n\n}}", l, l));    
            assert_parse_err(&format!("{} while {} {{\n\n}}", l, l));    
        }

        assert_parse_err("while {\n\n");    
        assert_parse_err("while {}");    
        assert_parse_err("while \n\n}");    
        assert_parse_err("while {{\n\n}}");    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if kw == &"true" || kw == &"false" {
                continue 
            }
            assert_parse_err(&format!("while {} {{\n\n}}", kw));
            assert_parse_err(&format!("while {} {{\n\n}}", kw.to_uppercase()));
        }

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("while{} {{\n\n}}", kw));
            assert_parse_err(&format!("{} while {{\n\n}}", kw));    
            assert_parse_err(&format!("{}while {{\n\n}}", kw));    
            assert_parse_err(&format!("{}while{} {{\n\n}}", kw, kw));    
            assert_parse_err(&format!("{} while {} {{\n\n}}", kw, kw));    

            assert_parse_err(&format!("while{} {{\n\n}}", kw.to_uppercase()));    
            assert_parse_err(&format!("{} while {{\n\n}}", kw.to_uppercase()));    
            assert_parse_err(&format!("{}while {{\n\n}}", kw.to_uppercase()));    

            assert_parse_err(&format!("{}while{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{}while{} {{\n\n}}", kw, kw.to_uppercase()));
            assert_parse_err(&format!("{}while{} {{\n\n}}", kw.to_uppercase(), kw));    
            
            assert_parse_err(&format!("{} while {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} while {} {{\n\n}}", kw, kw.to_uppercase()));
            assert_parse_err(&format!("{} while {} {{\n\n}}", kw.to_uppercase(), kw));    

        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("while {} {{\n\n}}", t));
            assert_parse_err(&format!("while{} {{\n\n}}", t));    
            assert_parse_err(&format!("{} while {{\n\n}}", t));    
            assert_parse_err(&format!("{}while {{\n\n}}", t));    
            assert_parse_err(&format!("{}while{} {{\n\n}}", t, t));    
            assert_parse_err(&format!("{} while {} {{\n\n}}", t, t));    


            assert_parse_err(&format!("while {} {{\n\n}}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("while{} {{\n\n}}", t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} while {{\n\n}}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{}while {{\n\n}}", t.to_string().to_uppercase() ));

            assert_parse_err(&format!("{}while{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{}while{} {{\n\n}}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{}while{} {{\n\n}}", t, t.to_string().to_uppercase())); 

            assert_parse_err(&format!("{} while {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase() ));
            assert_parse_err(&format!("{} while {} {{\n\n}}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} while {} {{\n\n}}", t.to_string().to_uppercase(), t));
        }
    }



    #[test]
    fn while_statements_all_literals() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while {} {} {} {{\n\n}}", l, s, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
        }
    }

    #[test]
    fn while_statements_below_var_decl_with_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own x {} = {}\nwhile {} {} {} {{\n\n}}", t, l, l, s, l)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::While(w) = &ast.globals[1] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else {
                        panic!("expected while statement");
                    }
                }
            }
        }
    }


    #[test]
    fn while_statements_below_var_decl_without_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own x {}\nwhile {} {} {} {{\n\n}}", t, l, s, l)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::While(w) = &ast.globals[1] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else {
                        panic!("expected while statement");
                    }
                }
            }
        }
    }


    #[test]
    fn while_statements_after_var_decl_with_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("while {} {} {} {{\n\n}}\nown x {} = {}", l, s, l, t, l)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::While(w) = &ast.globals[0] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else { panic!("expected while statement");}

                    if let Stmt::VarDecl(v) = &ast.globals[1] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }


                }
            }
        }
    }

    #[test]
    fn while_statements_after_var_decl_without_value() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("while {} {} {} {{\n\n}}\nown x {}", l, s, l, t)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::While(w) = &ast.globals[0] {
                        if let Expr::BinOp { left, right, op, .. } = &w.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!("Expected BinOp"); }
                        
                        assert_eq!(w.branch.len(), 0);
                    } else { panic!("expected while statement");}

                    if let Stmt::VarDecl(v) = &ast.globals[1] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }


    #[test]
    fn while_statements_before_expr() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("{}\nwhile {} {} {} {{\n\n}}", l, l, s, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 2);

                assert!(matches!(ast.globals[0], Stmt::Expr(_)));

                if let Stmt::While(w) = &ast.globals[1] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else { panic!("expected while statement");}
            }
        }
    }


    #[test]
    fn while_statements_after_expr() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while {} {} {} {{\n\n}}\n{}", l, s, l, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 2);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else { panic!("expected while statement");}
                
                assert!(matches!(ast.globals[1], Stmt::Expr(_)));
            }
        }
    }





    
    #[test]
    fn while_statements_int_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let ast = parse(&format!("while 1 {} 2 {{\n\n}}", s)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }

    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_int_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while{} 1 {} 2 {{\n\n}}", spaces, s)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_int_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while 1 {} 2 {}{{\n\n}}", s, spaces)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    

    #[test]
    fn while_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let ast = parse(&format!("while x {} y {{\n\n}}", s)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while{} x {} y {{\n\n}}", spaces, s)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }

    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while x {} y {}{{\n\n}}", s, spaces)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let ast = parse(&format!("while 69 {} y {{\n\n}}", s)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(69)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let ast = parse(&format!("while x {} 67 {{\n\n}}", s)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(67)));
                    } else { panic!(); }
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while{} 69 {} y {{\n\n}}", spaces, s)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while{} x {} 67 {{\n\n}}", spaces, s)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while -69 {} y {}{{\n\n}}", s, spaces)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(-69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let ast = parse(&format!("while x {} 67 {}{{\n\n}}", s, spaces)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::While(w) = &ast.globals[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&format!("while {}{{\n\n}}", spaces));
            spaces.push(' ');
        }
    }

}



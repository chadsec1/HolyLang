use super::*;

#[cfg(test)]
mod var_assign_tests {
    use super::*; 
 
    #[test]
    fn var_assign() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {}\nx = {}", t, l));
                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }    


                if let Stmt::VarAssign(va) = &stmts[1] {
                    assert_eq!(va.name, "x");
                } else {
                    panic!("Expected VarAssign");
                }
            }
        }
    }


    #[test]
    fn var_assign_multi() {
        let stmts = parse_body("x, y = swap()");
        if let Stmt::VarAssignMulti(ma) = &stmts[0] {
            assert_eq!(ma.names, vec!["x", "y"]);
        } else {
            panic!("Expected VarAssignMulti");
        }
    }
}

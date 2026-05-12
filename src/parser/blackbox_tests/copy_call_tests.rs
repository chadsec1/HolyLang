/// Tests for built-in "fake" function: 
/// copy()

use super::*;

#[cfg(test)]
mod copy_call_tests {
    use super::*;

    #[test]
    fn copy_call() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                let stmts = parse_body(&format!("own z {} = copy({})", t, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "z");
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Expr::CopyCall { .. }));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn copy_wrong_arg_count_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                assert_parse_err(&wrap(&format!("own z {} = copy({}, {})", t, l, l)));
                assert_parse_err(&wrap(&format!("own z {} = copy()", t)));
            }
        }
    }



}

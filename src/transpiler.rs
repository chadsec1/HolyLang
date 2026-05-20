use crate::ast::{
    AST, GlobalStmt, Type, Expr
};

/// Takes a reference to a Abstract Syntax Tree, and returns equvilent code in Rust as a string
///
pub fn transpile(ast: &AST) -> String {
    let mut rcode: String = String::new();


    for global_stmt in &ast.globals {
        let global_stmt_rcode = transpile_global_stmt(global_stmt);

        rcode = format!("{}{}\n", rcode, global_stmt_rcode);
    }


    return rcode
}


/// Transpiles a global statement into equvilent Rust code
///
fn transpile_global_stmt(global_stmt: &GlobalStmt) -> String {
    match global_stmt {
        GlobalStmt::Const(cons) => {
            let mut const_stmt_rcode: String = "const".to_string();

            let const_type = holy_type_to_rust_type_str(&cons.type_name);
            let const_value = literal_expr_to_rust_literal_str(&cons.value);

            const_stmt_rcode = format!("{} {}", const_stmt_rcode, cons.name.clone());
            const_stmt_rcode = format!("{}: {} =", const_stmt_rcode, const_type);
            const_stmt_rcode = format!("{} {};", const_stmt_rcode, const_value);

            return const_stmt_rcode;
        }

        _ => todo!()
    }
}


/// Turns a literal expression e.g. IntLiteral, BoolLiteral, Float64Literal, StringLiteral, etc,
/// into equvilent Rust value
///
fn literal_expr_to_rust_literal_str(expr: &Expr) -> String {
    match expr {
        Expr::IntLiteral { value, .. } => {
            let value_ty = holy_type_to_rust_type_str(&value.get_type());

            if value.is_signed() {
                let value_raw: i128 = value.as_i128();
                return format!("{}{}", value_raw, value_ty);
            
            } else {
                let value_raw: u128 = value.as_u128();
                return format!("{}{}", value_raw, value_ty);
            }
        },

        _ => todo!()
    }

}


/// Turns a holylang type e.g. Int32, Int64, etc, into equvilent Rust type
///
fn holy_type_to_rust_type_str(holy_type: &Type) -> String {
    match holy_type {
        Type::Int32 => "i32".to_string(),

        _ => todo!()
    }
}

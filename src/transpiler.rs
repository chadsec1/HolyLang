use crate::ast::{
    AST, Function, Stmt, GlobalStmt, Type, Expr, BinOpKind
};

/// Takes a reference to a Abstract Syntax Tree, and returns equvilent code in Rust as a string
///
pub fn transpile(ast: &AST) -> String {
    let mut rcode: String = String::new();

    for global_stmt in &ast.globals {
        let global_stmt_rcode = transpile_global_stmt(global_stmt);

        rcode = format!("{}{}\n", rcode, global_stmt_rcode);
    }

    for func in &ast.functions {
        let func_rcode = transpile_function(func);

        rcode = format!("{}{}\n", rcode, func_rcode);
    }

    return rcode
}



/// Transpiles a function and its inner statements into equvilent Rust code
///
fn transpile_function(func: &Function) -> String {
    let mut func_rcode: String = "fn".to_string();

    func_rcode = format!("{} {}() {{\n", func_rcode, func.name.clone());
    
    for stmt in &func.body {
        let stmt_rcode = transpile_stmt(stmt);
        func_rcode = format!("{} {}\n", func_rcode, stmt_rcode);
    }
    func_rcode = format!("{}\n}}", func_rcode);
    return func_rcode
}



/// Transpiles a statement into equivlent Rust code
///
fn transpile_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::VarDecl(var) => {
            let mut var_stmt_rcode: String = "let".to_string();

            let var_type = holy_type_to_rust_type_str(&var.type_name);
            let var_value = expr_to_rust_expr(&var.value);

            var_stmt_rcode = format!("{} {}", var_stmt_rcode, var.name.clone());
            var_stmt_rcode = format!("{}: {} =", var_stmt_rcode, var_type);
            var_stmt_rcode = format!("{} {};", var_stmt_rcode, var_value);

            return var_stmt_rcode;
        },

        _ => todo!()
    }
}


/// Transpiles a global statement into equvilent Rust code
///
fn transpile_global_stmt(global_stmt: &GlobalStmt) -> String {
    match global_stmt {
        GlobalStmt::Const(cons) => {
            let mut const_stmt_rcode: String = "const".to_string();

            let const_type = holy_type_to_rust_type_str(&cons.type_name);
            let const_value = expr_to_rust_expr(&cons.value);

            const_stmt_rcode = format!("{} {}", const_stmt_rcode, cons.name.clone());
            const_stmt_rcode = format!("{}: {} =", const_stmt_rcode, const_type);
            const_stmt_rcode = format!("{} {};", const_stmt_rcode, const_value);

            return const_stmt_rcode;
        }

        _ => todo!()
    }
}


/// Turns a HolyLang expression, into equvilent Rust expression
///
fn expr_to_rust_expr(expr: &Expr) -> String {
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
        Expr::Float64Literal { value, .. } => {
            return format!("{}f64", value);
        },
        Expr::BoolLiteral { value, .. } => {
            return value.to_string();
        },
        Expr::StringLiteral { value, .. } => {
            return format!("\"{}\"", value.to_string());
        },

        Expr::BinOp { op, left, right, .. } => {
            let left_str = expr_to_rust_expr(&left);
            let right_str = expr_to_rust_expr(&right);

            match op {
                // Arithemtic
                //
                BinOpKind::Add => {
                    return format!("{}.checked_add({}).unwrap_or_else(|| panic!(\"arithemtic addition overflow\"))", left_str, right_str);
                },
                BinOpKind::Subtract => {
                    return format!("{}.checked_sub({}).unwrap_or_else(|| panic!(\"arithemtic subtraction overflow\"))", left_str, right_str);
                },
                BinOpKind::Multiply => {
                    return format!("{}.checked_mul({}).unwrap_or_else(|| panic!(\"arithemtic multiplication overflow\"))", left_str, right_str);
                },
                BinOpKind::Divide => {
                    return format!("{}.checked_div({}).unwrap_or_else(|| panic!(\"arithemtic divison overflow\"))", left_str, right_str);
                },

                // Logical
                //
                BinOpKind::Equal => {
                    return format!("({} == {})", left_str, right_str);
                },
                BinOpKind::NotEqual => {
                    return format!("({} != {})", left_str, right_str);
                },

                BinOpKind::Greater => {
                    return format!("({} > {})", left_str, right_str);
                },
                BinOpKind::GreaterEqual => {
                    return format!("({} >= {})", left_str, right_str);
                },

                BinOpKind::Less => {
                    return format!("({} > {})", left_str, right_str);
                },
                BinOpKind::LessEqual => {
                    return format!("({} <= {})", left_str, right_str);
                },

                BinOpKind::And => {
                    return format!("({} && {})", left_str, right_str);
                },
                BinOpKind::Or => {
                    return format!("({} || {})", left_str, right_str);
                }
                _ => todo!()

            }

        }


        _ => todo!()
    }

}


/// Turns a holylang type e.g. Int32, Int64, etc, into equvilent Rust type
///
fn holy_type_to_rust_type_str(holy_type: &Type) -> String {
    match holy_type {
        Type::Int8 => "i8".to_string(),
        Type::Int16 => "i16".to_string(),
        Type::Int32 => "i32".to_string(),
        Type::Int64 => "i64".to_string(),
        Type::Int128 => "i128".to_string(),

        Type::Byte => "u8".to_string(),
        Type::Uint16 => "u16".to_string(),
        Type::Uint32 => "u32".to_string(),
        Type::Uint64 => "u64".to_string(),
        Type::Uint128 => "u128".to_string(),
        Type::Usize => "usize".to_string(),
        
        Type::Float64 => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::String => "&str".to_string(),

        _ => todo!()
    }
}

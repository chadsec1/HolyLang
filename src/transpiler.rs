use crate::ast::{
    AST, Function, Stmt, GlobalStmt, Type, Expr, BinOpKind, UnaryOpKind, Constant, FixedArraySize
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
        Stmt::Const(cons) => parse_const(cons),
        Stmt::VarDecl(var) => {
            let var_type = holy_type_to_rust_type_str(&var.type_name);
            let var_value = holy_expr_to_rust_expr(&var.value);

            return format!("let mut {}: {} = {};", var.name, var_type, var_value);
        },

        Stmt::VarAssign(va) => {
            let va_value = holy_expr_to_rust_expr(&va.value);

            return format!("{} = {};", va.name, va_value)
        },



        _ => todo!()
    }
}


/// Transpiles a global statement into equvilent Rust code
///
fn transpile_global_stmt(global_stmt: &GlobalStmt) -> String {
    match global_stmt {
        GlobalStmt::Const(cons) => parse_const(cons),

        _ => todo!()
    }
}

fn parse_const(cons: &Constant) -> String {
    let mut const_stmt_rcode: String = "const".to_string();

    let const_type = holy_type_to_rust_type_str(&cons.type_name);
    let const_value = holy_expr_to_rust_expr(&cons.value);

    const_stmt_rcode = format!("{} {}", const_stmt_rcode, cons.name.clone());
    const_stmt_rcode = format!("{}: {} =", const_stmt_rcode, const_type);
    const_stmt_rcode = format!("{} {};", const_stmt_rcode, const_value);

    return const_stmt_rcode;
}

/// Turns a HolyLang expression, into equvilent Rust expression
///
fn holy_expr_to_rust_expr(expr: &Expr) -> String {
    match expr {
        Expr::IntLiteral { value, .. } => {
            let value_ty = holy_type_to_rust_type_str(&value.get_type());

            if value.is_signed() {
                let value_raw: i128 = value.as_i128();
                return format!("{}{}", value_raw, value_ty)
            
            } else {
                let value_raw: u128 = value.as_u128();
                return format!("{}{}", value_raw, value_ty)
            }
        },

        Expr::Float64Literal { value, .. } => format!("{}f64", value),

        Expr::BoolLiteral { value, .. } => value.to_string(),

        Expr::StringLiteral { value, .. } => format!("\"{}\"", value.to_string()),
        
        Expr::ArrayLiteral { elements, type_name, .. } => {
            let mut elems = String::new();

            match type_name.clone().expect("(Compiler bug) Expected type_name to be Some, instead got None. theres likely a bug in semantics layer") {
                Type::Array(_) => elems.push_str("vec!["),
                Type::FixedArray(_, _) => elems.push('['),
                other => panic!("(Compiler bug) got arrayl iteral with non array array type_name `{:?}`, indicating a potentinal bug in semantics layer", other)
            }

            for e in elements {
                let elem_expr = holy_expr_to_rust_expr(e);
                elems.push_str(&elem_expr);
                elems.push(',');
            }

            if elems.ends_with(',') {
                elems.pop();
            }

            elems.push(']');
            return elems
        },

        Expr::Var { name, .. } => name.to_string(),

        Expr::UnaryOp { op, expr, .. } => {
            let expr_str = holy_expr_to_rust_expr(&expr);

            match op {
                UnaryOpKind::Negate => format!("{}.checked_neg().unwrap_or_else(|| panic!(\"unary negate integer overflow\"))", expr_str),
                UnaryOpKind::Not => format!("!{}", expr_str),
                UnaryOpKind::BitwiseNot => format!("!{}", expr_str),
            }
        },

        Expr::BinOp { op, left, right, .. } => {
            let left_str = holy_expr_to_rust_expr(&left);
            let right_str = holy_expr_to_rust_expr(&right);

            match op {
                // Arithemtic
                //
                BinOpKind::Add      => format!("{}.checked_add({}).unwrap_or_else(|| panic!(\"arithemtic addition overflow\"))", left_str, right_str),
                BinOpKind::Subtract => format!("{}.checked_sub({}).unwrap_or_else(|| panic!(\"arithemtic subtraction overflow\"))", left_str, right_str),
                BinOpKind::Multiply => format!("{}.checked_mul({}).unwrap_or_else(|| panic!(\"arithemtic multiplication overflow\"))", left_str, right_str),
                BinOpKind::Divide   => format!("{}.checked_div({}).unwrap_or_else(|| panic!(\"arithemtic divison overflow\"))", left_str, right_str),

                // Logical
                //
                BinOpKind::Equal    => format!("({} == {})", left_str, right_str),
                BinOpKind::NotEqual => format!("({} != {})", left_str, right_str),
                BinOpKind::Greater  => format!("({} > {})", left_str, right_str),
                BinOpKind::Less     => format!("({} > {})", left_str, right_str),
                
                BinOpKind::GreaterEqual => format!("({} >= {})", left_str, right_str),
                BinOpKind::LessEqual    => format!("({} <= {})", left_str, right_str),

                BinOpKind::And => format!("({} && {})", left_str, right_str),
                BinOpKind::Or  => format!("({} || {})", left_str, right_str),

                // Bitwise
                //
                BinOpKind::BitwiseAnd => format!("({} & {})", left_str, right_str),
                BinOpKind::BitwiseOr  => format!("({} | {})", left_str, right_str),

                // NOTE to self: It's safe to cast as u32 here, becasue the semantics phase checked it.
                // but its important to keep this in mind... weird bitwise bugs may arise, and if it does,
                // this is culprit
                //
                BinOpKind::BitwiseShiftLeft => format!("({}.checked_shl({} as u32)).unwrap_or_else(|| panic!(\"bitwise shift left overflow\"))", left_str, right_str),
                BinOpKind::BitwiseShiftRight => format!("({}.checked_shr({} as u32)).unwrap_or_else(|| panic!(\"bitwise shift right overflow\"))", left_str, right_str),
            }

        }


        _ => todo!()
    }

}


/// Turns a holylang type e.g. Int32, Int64, etc, into equvilent Rust type
///
fn holy_type_to_rust_type_str(holy_type: &Type) -> String {
    match holy_type {
        Type::Int8   => "i8".to_string(),
        Type::Int16  => "i16".to_string(),
        Type::Int32  => "i32".to_string(),
        Type::Int64  => "i64".to_string(),
        Type::Int128 => "i128".to_string(),

        Type::Byte    => "u8".to_string(),
        Type::Uint16  => "u16".to_string(),
        Type::Uint32  => "u32".to_string(),
        Type::Uint64  => "u64".to_string(),
        Type::Uint128 => "u128".to_string(),
        Type::Usize   => "usize".to_string(),
        
        Type::Float64 => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::String => "&str".to_string(),
        
        Type::Array(t) => format!("Vec<{}>", holy_type_to_rust_type_str(t)),
        Type::FixedArray(t, s) => match s {
                FixedArraySize::Literal(n) => format!("[{}; {}]", holy_type_to_rust_type_str(t), n),
                FixedArraySize::Const(c) => format!("[{}; {}]", holy_type_to_rust_type_str(t), c)
            }
        }
}

use crate::ast::{
    AST, Function, Stmt, GlobalStmt, Type, Expr, BinOpKind, UnaryOpKind, Constant, FixedArraySize,

    ArraySliceRange
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
    let mut func_rcode = String::new();

    func_rcode.push_str("fn ");
    func_rcode.push_str(&func.name);
    func_rcode.push('(');

    for param in &func.params {
        let param_type = holy_type_to_rust_type_str(&param.type_name);
        func_rcode.push_str(&param.name);
        func_rcode.push_str(": ");
        func_rcode.push_str(&param_type);
        func_rcode.push_str(", ");
    }

    if func_rcode.ends_with(", ") {
        func_rcode.pop().unwrap();
        func_rcode.pop().unwrap();
    }

    func_rcode.push(')');

    if let Some(ret_types) = &func.return_type {
        func_rcode.push_str(" -> ");
        if ret_types.len() == 1 {
            func_rcode.push_str(&holy_type_to_rust_type_str(&ret_types[0]));
        } else {
            func_rcode.push('(');
            for t in ret_types {
                func_rcode.push_str(&holy_type_to_rust_type_str(&t));
                func_rcode.push_str(", ");
            }
            
            if func_rcode.ends_with(", ") {
                func_rcode.pop().unwrap();
                func_rcode.pop().unwrap();
            }

            func_rcode.push(')');
        }

    }
    
    func_rcode.push_str(" {\n");
    
    for stmt in &func.body {
        let stmt_rcode = transpile_stmt(stmt);
        func_rcode = format!("{} {}\n", func_rcode, stmt_rcode);
    }
    func_rcode.push_str("}\n");
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

        Stmt::VarDeclMulti(multi_var, expr) => {
            let mut multi_decl_stmt_str: String = "let (".to_string();
            
            for var in multi_var {
                multi_decl_stmt_str.push_str("mut ");
                multi_decl_stmt_str.push_str(&var.name);
                multi_decl_stmt_str.push_str(", ");
            }

            if multi_decl_stmt_str.ends_with(", ") {
                multi_decl_stmt_str.pop().unwrap();
                multi_decl_stmt_str.pop().unwrap();
            }
            
            multi_decl_stmt_str.push_str("): (");

            for var in multi_var {
                let var_type = holy_type_to_rust_type_str(&var.type_name);

                multi_decl_stmt_str.push_str(&var_type);
                multi_decl_stmt_str.push_str(", ");
            }

            if multi_decl_stmt_str.ends_with(", ") {
                multi_decl_stmt_str.pop().unwrap();
                multi_decl_stmt_str.pop().unwrap();
            }

            multi_decl_stmt_str.push(')');
            multi_decl_stmt_str.push_str(" = ");
            multi_decl_stmt_str.push_str(&holy_expr_to_rust_expr(&expr));
            multi_decl_stmt_str.push(';');

            return multi_decl_stmt_str
        },

        Stmt::VarAssign(va) => {
            let va_value = holy_expr_to_rust_expr(&va.value);

            return format!("{} = {};", va.name, va_value)
        },

        Stmt::VarAssignMulti(multi_assignment) => {
            let mut multi_assign_stmt_str = String::new();
            multi_assign_stmt_str.push('(');
            
            for name in &multi_assignment.names {
                multi_assign_stmt_str.push_str(&name);
                multi_assign_stmt_str.push_str(", ");
            }

            if multi_assign_stmt_str.ends_with(", ") {
                multi_assign_stmt_str.pop().unwrap();
                multi_assign_stmt_str.pop().unwrap();
            }
            
            multi_assign_stmt_str.push(')');
            multi_assign_stmt_str.push_str(" = ");
            multi_assign_stmt_str.push_str(&holy_expr_to_rust_expr(&multi_assignment.value));
            multi_assign_stmt_str.push(';');

            return multi_assign_stmt_str
        },



        Stmt::Lock(var_exprs) => {
            let mut var_lock_stmts_str = "// Lock statement started\n".to_string();

            for expr in var_exprs {
                if let Expr::Var { name, .. } = expr {
                    var_lock_stmts_str.push_str(&format!("let {} = {};", name, name));
                    var_lock_stmts_str.push('\n');
                } else {
                    panic!("(Compiler bug) Got a non-var expression in lock statement, indicating a bug in semantics analysis layer.\nvar_exprs: {:?}", var_exprs);
                }
            }

            var_lock_stmts_str.push_str("// Lock statement ended\n");
            return var_lock_stmts_str
        },

        Stmt::Unlock(var_exprs) => {
            let mut var_unlock_stmts_str = "// Unlock statement started\n".to_string();

            for expr in var_exprs {
                if let Expr::Var { name, .. } = expr {
                    var_unlock_stmts_str.push_str(&format!("let mut {} = {};", name, name));
                    var_unlock_stmts_str.push('\n');
                } else {
                    panic!("(Compiler bug) Got a non-var expression in unlock statement, indicating a bug in semantics analysis layer.\nvar_exprs: {:?}", var_exprs);
                }
            }

            var_unlock_stmts_str.push_str("// Unlock statement ended");
            return var_unlock_stmts_str
        },
        
        Stmt::Break(_) => "break;".to_string(),
        Stmt::Continue(_) => "continue;".to_string(),

        Stmt::Return(expr_vec) => {
            let mut ret_exprs_str = String::new();

            if expr_vec.len() == 1 {
                ret_exprs_str.push_str(&holy_expr_to_rust_expr(&expr_vec[0]));
            } else {
                ret_exprs_str.push('(');
                for e in expr_vec {
                    let expr_str = holy_expr_to_rust_expr(&e);
                    ret_exprs_str.push_str(&expr_str);
                    ret_exprs_str.push_str(", ");
                }

                if ret_exprs_str.ends_with(", ") {
                    ret_exprs_str.pop().unwrap();
                    ret_exprs_str.pop().unwrap();
                }
                ret_exprs_str.push(')');
            }

            return format!("return {}", ret_exprs_str)
        },


        Stmt::Infinite(inf) => {
            let mut inf_branch_stmts_str = String::new();

            for s in &inf.branch {
                inf_branch_stmts_str.push_str(&transpile_stmt(&s));
                inf_branch_stmts_str.push('\n');
            }

            return format!("loop {{\n{}}}", inf_branch_stmts_str)
        },

        Stmt::While(w) => {
            let mut w_branch_stmts_str = String::new();

            for s in &w.branch {
                w_branch_stmts_str.push_str(&transpile_stmt(&s));
                w_branch_stmts_str.push('\n');
            }

            return format!("while {} {{\n{}}}", holy_expr_to_rust_expr(&w.condition), w_branch_stmts_str)
        },

        Stmt::For(f) => {
            let mut f_branch_stmts_str = String::new();

            for s in &f.branch {
                f_branch_stmts_str.push_str(&transpile_stmt(&s));
                f_branch_stmts_str.push('\n');
            }

            return format!("for {} in {} {{\n{}}}", f.holder_name, holy_expr_to_rust_expr(&f.value), f_branch_stmts_str)
        },

        Stmt::If(i) => {
            let mut if_branch_stmts_str = String::new();

            for s in &i.if_branch {
                if_branch_stmts_str.push_str(&transpile_stmt(&s));
                if_branch_stmts_str.push('\n');
            }

            let mut if_stmt = format!("if {} {{\n{}}}", holy_expr_to_rust_expr(&i.condition), if_branch_stmts_str);

            for elif_branch in &i.elif_branches {
                let mut elif_branch_stmts_str = String::new();

                for s in &elif_branch.1 {
                    elif_branch_stmts_str.push_str(&transpile_stmt(&s));
                    elif_branch_stmts_str.push('\n');
                }

                if_stmt = format!("{} else if {} {{\n{}}}", if_stmt, holy_expr_to_rust_expr(&elif_branch.0), elif_branch_stmts_str);
            }

            if let Some(else_branch) = &i.else_branch {
                let mut else_branch_stmts_str = String::new();

                for s in else_branch {
                    else_branch_stmts_str.push_str(&transpile_stmt(&s));
                    else_branch_stmts_str.push('\n');
                }

                if_stmt = format!("{} else {{\n{}}}", if_stmt, else_branch_stmts_str);
            }

            return if_stmt
        },


        Stmt::Expr(expr) => format!("{};", holy_expr_to_rust_expr(expr))
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
    let const_type = holy_type_to_rust_type_str(&cons.type_name);
    let const_value = holy_expr_to_rust_expr(&cons.value);

    return format!("const {}: {} = {};", cons.name, const_type, const_value);
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

        Expr::StringLiteral { value, .. } => format!("\"{}\".to_string()", value.to_string()),
        
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
                elems.pop().unwrap();
            }

            elems.push(']');
            return elems
        },

        Expr::ArrayAccess { array, index, .. } => format!("{}[{}]", holy_expr_to_rust_expr(array), holy_expr_to_rust_expr(index)),

        // to_vec() safe here because HolyLang always returns a dynamic array when you slice any
        // array regardless if its fixed sized, or dynamic.
        //
        Expr::ArraySlicing { array, range, .. } => match range {
            ArraySliceRange::From(from) => format!("{}[{}..].to_vec()", holy_expr_to_rust_expr(array), holy_expr_to_rust_expr(from)),
            ArraySliceRange::To(to) => format!("{}[{}..].to_vec()", holy_expr_to_rust_expr(array), holy_expr_to_rust_expr(to)),
            ArraySliceRange::FromTo(from, to) => format!("{}[{}..{}].to_vec()", holy_expr_to_rust_expr(array), holy_expr_to_rust_expr(from), holy_expr_to_rust_expr(to)),
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

                BinOpKind::BitwiseShiftLeft => format!("({}.checked_shl({}.try_into().unwrap_or_else(|_| panic!(\"bitwise shift left count `{{}}` does not fit in u32\", {})) )).unwrap_or_else(|| panic!(\"bitwise shift left overflow\"))", left_str, right_str, right_str),

                BinOpKind::BitwiseShiftRight => format!("({}.checked_shr({}.try_into().unwrap_or_else(|_| panic!(\"bitwise shift right count `{{}}` does not fit in u32\", {})) )).unwrap_or_else(|| panic!(\"bitwise shift right overflow\"))", left_str, right_str, right_str),

            }

        },
        
        Expr::Call { name, args, .. } => {
            let mut args_str = String::new();
            
            args_str.push_str(&name);
            args_str.push('(');

            for arg_expr in args {
                args_str.push_str(&holy_expr_to_rust_expr(arg_expr));
                args_str.push(',');
            }

            if args_str.ends_with(',') {
                args_str.pop().unwrap();
            }

            args_str.push(')');
            return args_str
        },

        Expr::RangeCall { start, end, ..} => format!("{}..{}", holy_expr_to_rust_expr(start), holy_expr_to_rust_expr(end)),

        Expr::CopyCall { expr, .. } => format!("{}.clone()", holy_expr_to_rust_expr(expr)),
        Expr::FormatCall { template, expressions, .. } => {
            let mut format_expr_str = String::new();
            format_expr_str.push_str(&"format!(");
            format_expr_str.push('"');
            format_expr_str.push_str(&template);
            format_expr_str.push('"');

            for expr in expressions {
                format_expr_str.push_str(", ");
                format_expr_str.push_str(&holy_expr_to_rust_expr(expr));
            }
            
            format_expr_str.push(')');

            return format_expr_str
        }
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
        Type::String => "String".to_string(),
        
        Type::Array(t) => format!("Vec<{}>", holy_type_to_rust_type_str(t)),
        Type::FixedArray(t, s) => match s {
                FixedArraySize::Literal(n) => format!("[{}; {}]", holy_type_to_rust_type_str(t), n),
                FixedArraySize::Const(c) => format!("[{}; {}]", holy_type_to_rust_type_str(t), c)
            }
        }
}

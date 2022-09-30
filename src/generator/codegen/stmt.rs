use super::*;

pub fn gen_rs_for_stmt(generator: &mut Generator, stmt: Statement) -> String {
    let mut rs = generator.padding();

    match stmt {
        Statement::Expr(expr) => {
            rs.push_str(&gen_rs_for_expr(generator, expr));
        },
        Statement::Item(item) => {
            rs.push_str(&gen_rs_for_item(generator, item));

            // Semicolons not needed on items
            return rs;
        },
        Statement::Assign(assign) => {
            if assign.is_const {
                rs.push_str("let ");
            } else {
                rs.push_str("let mut ");
            }

            rs.push_str(&assign.name);

            if let Some(explicit_type) = assign.explicit_type {
                rs.push_str(": ");
                rs.push_str(&gen_rs_for_type(generator, explicit_type));
            }

            if let Some(expr) = assign.rhs {
                rs.push_str(" = ");
                rs.push_str(&gen_rs_for_expr(generator, expr));
            }
        },
    }

    rs.push_str(";\n");

    return rs;
}


#![allow(dead_code)]
use crate::constraint::{AoristStatement, Import};
use linked_hash_set::LinkedHashSet;
use rustpython_parser::ast::{
    BooleanOperator, Expression, ExpressionType, ImportSymbol, Keyword, Number, Operator,
    Parameter, Parameters, Statement, StatementType, StringGroup, Suite, Varargs,
};
use std::collections::BTreeSet;
pub type PythonStatementInput = (
    Vec<AoristStatement>,
    LinkedHashSet<String>,
    BTreeSet<Import>,
);

pub struct PythonProgram {}
impl PythonProgram {
    fn render_string_group(string_group: &StringGroup) -> Result<String, String> {
        match string_group {
            StringGroup::Constant { value } => match value.find('\n') {
                None => Ok(format!("'{}'", value).to_string()),
                Some(_) => Ok(format!("\"\"\"\n{}\"\"\"", value).to_string()),
            },
            StringGroup::FormattedValue {
                value,
                conversion,
                spec,
            } => match conversion {
                Some(_) => Err("Don't know how to apply conversion to string group".to_string()),
                None => match spec {
                    Some(s) => Ok(format!(
                        "{} % {}",
                        Self::render_string_group(s)?,
                        Self::render_expr(value)?,
                    )
                    .to_string()),
                    None => Err("Don't know what to do when spec missing.".to_string()),
                },
            },
            StringGroup::Joined { values } => Ok(format!(
                "({})",
                values
                    .iter()
                    .map(|x| Self::render_string_group(x).unwrap())
                    .collect::<Vec<String>>()
                    .join("\n")
            )),
        }
    }
    fn render_expr(expr: &Expression) -> Result<String, String> {
        match &expr.node {
            ExpressionType::String { value } => Self::render_string_group(value),
            ExpressionType::Subscript { a, b, .. } => {
                Ok(format!("{}[{}]", Self::render_expr(a)?, Self::render_expr(b)?).to_string())
            }
            ExpressionType::Identifier { name } => Ok(name.to_string()),
            ExpressionType::Attribute { value, name } => {
                Ok(format!("{}.{}", Self::render_expr(&value)?, name).to_string())
            }
            ExpressionType::Starred { value } => {
                Ok(format!("*{}", Self::render_expr(&value)?).to_string())
            }
            ExpressionType::Dict { elements } => Ok(format!(
                "{{{}}}",
                elements
                    .into_iter()
                    .map(|(k, v)| {
                        let key = match k {
                            None => "**".to_string(),
                            Some(expr) => Self::render_expr(expr).unwrap(),
                        };
                        let val = Self::render_expr(v).unwrap();
                        format!("{}: {}", key, val).to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
            ExpressionType::List { elements } => Ok(format!(
                "[{}]",
                elements
                    .iter()
                    .map(|x| Self::render_expr(x).unwrap())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .to_string()),
            ExpressionType::Tuple { elements } => Ok(format!(
                "({})",
                elements
                    .iter()
                    .map(|x| Self::render_expr(x).unwrap())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .to_string()),
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                let mut formatted_args: Vec<String> = Vec::new();
                for arg in args {
                    formatted_args.push(Self::render_expr(&arg)?);
                }
                for keyword in keywords {
                    formatted_args.push(Self::render_keyword(&keyword)?);
                }
                Ok(format!(
                    "{}({})",
                    call = Self::render_expr(function)?,
                    args = formatted_args.join(", "),
                )
                .to_string())
            }
            ExpressionType::True => Ok("True".to_string()),
            ExpressionType::False => Ok("False".to_string()),
            ExpressionType::None => Ok("None".to_string()),
            ExpressionType::Number { value } => match value {
                Number::Integer { value } => Ok(value.to_string()),
                Number::Float { value } => Ok(format!("{:.}", value).to_string()),
                Number::Complex { real, imag } => Ok(format!("{:.}+{:.}j", real, imag).to_string()),
            },
            ExpressionType::BoolOp { op, values } => match values.len() {
                2 => {
                    let mut it = values.into_iter();
                    let a = it.next().unwrap();
                    let b = it.next().unwrap();
                    let op_fmt = match op {
                        BooleanOperator::And => "and",
                        BooleanOperator::Or => "or",
                    };
                    Ok(format!(
                        "{} {} {}",
                        Self::render_expr(&a)?,
                        op_fmt,
                        Self::render_expr(&b)?
                    )
                    .to_string())
                }
                _ => Err(format!("Incorrect number of arguments to boolean operator").to_string()),
            },
            ExpressionType::Binop { a, op, b } => {
                let op_fmt = match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mult => "*",
                    Operator::MatMult => "@",
                    Operator::Div => "/",
                    Operator::Mod => "%",
                    Operator::Pow => "**",
                    Operator::LShift => "<<",
                    Operator::RShift => ">>",
                    Operator::BitOr => "|",
                    Operator::BitXor => "^",
                    Operator::BitAnd => "&",
                    Operator::FloorDiv => "//",
                };
                Ok(format!(
                    "{} {} {}",
                    Self::render_expr(&*a)?,
                    op_fmt,
                    Self::render_expr(&*b)?
                )
                .to_string())
            }
            _ => Err(format!("Unknown argument: {}", Expression::name(&expr))),
        }
    }
    fn render_keyword(keyword: &Keyword) -> Result<String, String> {
        match keyword.name {
            Some(ref name) => {
                Ok(format!("{}={}", name, Self::render_expr(&keyword.value)?).to_string())
            }
            None => Ok(format!("**{}", Self::render_expr(&keyword.value)?).to_string()),
        }
    }
    pub fn render_suite(suite: Suite) -> Result<String, String> {
        let mut rendered: Vec<String> = Vec::new();
        for stmt in suite {
            rendered.push(Self::render_statement(stmt)?);
        }
        Ok(rendered.join("\n"))
    }
    fn render_import_symbol(sym: ImportSymbol) -> Result<String, String> {
        match sym.alias {
            Some(alias) => Ok(format!("{} as {}", sym.symbol, alias).to_string()),
            None => Ok(sym.symbol.clone()),
        }
    }
    fn render_import_symbols(symbols: Vec<ImportSymbol>) -> Result<String, String> {
        assert!(symbols.len() > 0);
        if symbols.len() == 1 {
            return Self::render_import_symbol(symbols.into_iter().next().unwrap());
        }
        Ok(format!(
            "({})",
            symbols
                .into_iter()
                .map(|x| Self::render_import_symbol(x).unwrap())
                .collect::<Vec<String>>()
                .join(", ")
        )
        .to_string())
    }
    fn render_parameter(parameter: Parameter) -> Result<String, String> {
        if parameter.annotation.is_some() {
            return Err("Don't know what to do with annotated parameters yet".to_string());
        }
        Ok(parameter.arg.clone())
    }
    fn render_parameters(parameters: Parameters) -> Result<String, String> {
        /*if parameters.posonlyargs_count != parameters.args.len() {
            return Err(format!(
                "Incorrect number of arguments {} {}",
                parameters.posonlyargs_count,
                parameters.args.len()
            )
            .to_string());
        }*/
        if parameters.vararg != Varargs::None || parameters.kwarg != Varargs::None {
            return Err("Don't know what to do with varargs yet".to_string());
        }
        if parameters.defaults.len() > 0 || parameters.kw_defaults.len() > 0 {
            return Err("Don't know what to do with defaults yet".to_string());
        }
        Ok(parameters
            .args
            .into_iter()
            .chain(parameters.kwonlyargs.into_iter())
            .map(|x| Self::render_parameter(x).unwrap())
            .collect::<Vec<String>>()
            .join(", "))
    }
    fn render_statement(statement: Statement) -> Result<String, String> {
        match statement.node {
            StatementType::Assign { targets, value, .. } => {
                if targets.len() != 1 {
                    return Err("More than one target in task assignment".to_string());
                }
                let val = Self::render_expr(targets.iter().next().unwrap())?;
                let call = Self::render_expr(&value)?;
                Ok(format!("{val} = {call}", val = val, call = call).to_string())
            }
            StatementType::Import { names } => {
                Ok(format!("import {}", Self::render_import_symbols(names)?).to_string())
            }
            StatementType::ImportFrom { module, names, .. } => Ok(format!(
                "from {} import {}",
                match module {
                    Some(m) => m,
                    None => ".".to_string(),
                },
                Self::render_import_symbols(names)?
            )
            .to_string()),
            StatementType::Expression { expression } => Self::render_expr(&expression),
            StatementType::For {
                is_async,
                target,
                iter,
                body,
                orelse,
            } => {
                if is_async {
                    return Err("Cannot render async for statements".to_string());
                }
                if orelse.is_some() {
                    return Err("Cannot render for statements with orelse".to_string());
                }
                let body_fmt = body
                    .into_iter()
                    .map(|x| Self::render_statement(x).unwrap())
                    .map(|x| {
                        let multiline = x.replace("\n", "\n    ");
                        format!("    {}", multiline).to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(format!(
                    "for {target} in {iter}:\n{body_fmt}",
                    target = Self::render_expr(&target)?,
                    iter = Self::render_expr(&iter)?,
                    body_fmt = body_fmt
                )
                .to_string())
            }
            StatementType::FunctionDef {
                is_async,
                name,
                args,
                body,
                decorator_list,
                returns,
            } => {
                if is_async {
                    return Err("Cannot render async functions".to_string());
                }
                if decorator_list.len() > 0 {
                    return Err("Cannot render decorated functions".to_string());
                }
                if returns.is_some() {
                    return Err("Cannot render functions with return annotations".to_string());
                }
                Ok(format!(
                    "def {name}({args}):\n{body}",
                    name = name,
                    args = Self::render_parameters(*args)?,
                    body = body
                        .into_iter()
                        .map(|x| format!("    {}", Self::render_statement(x).unwrap()).to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                ))
            }
            _ => Err("Unknown statement type.".to_string()),
        }
    }
}

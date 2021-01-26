#![allow(dead_code)]
use crate::constraint::{LiteralsMap, StringLiteral};
use crate::constraint_state::ConstraintState;
use aorist_primitives::Dialect;
use rustpython_parser::ast::{
    Expression, ExpressionType, ImportSymbol, Keyword, Located, Program, Statement, StatementType,
    StringGroup, Suite, WithItem,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct PrefectProgram {
    imports: Vec<Statement>,
    preamble: Vec<Statement>,
    flow: Vec<Statement>,
}
impl PrefectProgram {
    fn empty() -> Self {
        Self {
            imports: Vec::new(),
            preamble: Vec::new(),
            flow: Vec::new(),
        }
    }
    fn program(self) -> Program {
        let mut statements = self.imports;
        for elem in self.preamble {
            statements.push(elem);
        }
        let mut loc = statements.iter().last().unwrap().location;
        loc.newline();
        let with = StatementType::With {
            is_async: false,
            items: vec![WithItem {
                context_expr: Located {
                    location: loc,
                    node: ExpressionType::NamedExpression {
                        // TODO: change this to Flow(...)
                        left: Box::new(Located {
                            location: loc,
                            node: ExpressionType::Identifier {
                                name: "flow".into(),
                            },
                        }),
                        right: Box::new(Located {
                            location: loc,
                            node: ExpressionType::Identifier {
                                name: "flow".into(),
                            },
                        }),
                    },
                },
                optional_vars: None,
            }],
            body: self.flow,
        };
        statements.push(Located {
            location: loc,
            node: with,
        });
        Program { statements }
    }
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
            _ => Err("Unknown statement type.".to_string()),
        }
    }
}

pub trait PrefectTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>>;
    fn get_constraint_name(&self) -> String;
    fn register_literals(
        &'a self,
        _literals: LiteralsMap,
        _constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
    }
    fn keywords_to_map(&self, keywords: Vec<Keyword>) -> HashMap<String, Expression> {
        assert!(keywords
            .iter()
            .filter(|x| x.name.is_none())
            .next()
            .is_none());
        keywords
            .into_iter()
            .map(|x| (x.name.unwrap(), x.value))
            .collect()
    }
    fn map_to_keywords(&self, map: HashMap<String, Expression>) -> Vec<Keyword> {
        map.into_iter()
            .map(|(k, v)| Keyword {
                name: Some(k),
                value: v,
            })
            .collect()
    }
    fn extract_hashable_value_if_string_constant(&self, expr: &Expression) -> Option<String> {
        match expr.node {
            ExpressionType::String { ref value } => match value {
                StringGroup::Constant { ref value } => Some(value.clone()),
                _ => None,
            },
            _ => None,
        }
    }
    fn render_ids(ids: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> String {
        ids.iter()
            .map(|x| format!("'{}'", x.read().unwrap().get_task_name()).to_string())
            .collect::<Vec<String>>()
            .join(",\n    ")
    }
}
pub struct PrefectPythonTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    constraint_name: String,
}
impl<'a> PrefectTaskRender<'a> for PrefectPythonTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
}
impl<'a> PrefectPythonTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>, constraint_name: String) -> Self {
        Self {
            members,
            constraint_name,
        }
    }
}
pub struct PrefectShellTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    dialect: Dialect,
    constraint_name: String,
}
impl<'a> PrefectTaskRender<'a> for PrefectShellTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn register_literals(&'a self, literals: LiteralsMap, state: Arc<RwLock<ConstraintState<'a>>>) {
        // TODO: this is super hacky, but it should do the job for now
        let read = state.read().unwrap();
        let call = match &self.dialect {
            Dialect::Presto(_) => {
                format!("presto -e '{}'", read.get_call().unwrap().clone()).to_string()
            }
            Dialect::Bash(_) => read.get_call().unwrap().clone(),
            _ => panic!("Unknown dialect encountered for PrefectShellTaskRender."),
        };
        let uuid = read.get_constraint_uuid();

        let mut write = literals.write().unwrap();
        let arc = write
            .entry(call.clone())
            .or_insert(Arc::new(RwLock::new(StringLiteral::new(call))));
        let mut arc_write = arc.write().unwrap();
        arc_write.register_object(uuid, Some("command".to_string()));
        drop(arc_write);
        drop(write);
    }
}
impl<'a> PrefectShellTaskRender<'a> {
    pub fn new(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        dialect: Dialect,
        constraint_name: String,
    ) -> Self {
        Self {
            members,
            dialect,
            constraint_name,
        }
    }
}
pub struct PrefectConstantTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    constraint_name: String,
}
impl<'a> PrefectConstantTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>, constraint_name: String) -> Self {
        Self {
            members,
            constraint_name,
        }
    }
}
impl<'a> PrefectTaskRender<'a> for PrefectConstantTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
}

pub enum PrefectRender<'a> {
    Python(PrefectPythonTaskRender<'a>),
    Shell(PrefectShellTaskRender<'a>),
    Constant(PrefectConstantTaskRender<'a>),
}
impl<'a> PrefectRender<'a> {
    pub fn register_literals(
        &'a self,
        literals: LiteralsMap,
        constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        //task_render.register_literals(literals.clone(), members.clone());
        match &self {
            PrefectRender::Python(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Shell(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Constant(x) => x.register_literals(literals, constraint_state),
        }
    }
}

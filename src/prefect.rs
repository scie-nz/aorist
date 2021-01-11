#![allow(dead_code)]
use crate::constraint::{
    AoristStatement, ArgType, Attribute, Call, Dict, List, LiteralsMap, SimpleIdentifier,
    StringLiteral, Subscript, Tuple,
};
use crate::constraint_state::{ConstraintState, PrefectSingleton};
use aorist_primitives::Dialect;
use indoc::formatdoc;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use rustpython_parser::ast::{
    Expression, ExpressionType, Keyword, Located, Location, Program, Statement, StatementType,
    StringGroup, Suite, WithItem,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct PrefectProgram {
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
    fn render_suite(suite: Suite) -> Result<String, String> {
        let mut rendered: Vec<String> = Vec::new();
        for stmt in suite {
            rendered.push(Self::render_statement(stmt)?);
        }
        Ok(rendered.join("\n"))
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
    fn get_singletons(&self, literals: LiteralsMap) -> HashMap<String, PrefectSingleton> {
        let num_constraints = self.get_constraints().len();
        for rw in self.get_constraints() {
            let mut write = rw.write().unwrap();
            let name = write.get_task_name();
            // TODO: magic number
            if num_constraints <= 2 {
                write.set_task_val(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    name,
                )));
            } else {
                let shorter_name =
                    name.replace(&format!("{}__", self.get_constraint_name()).to_string(), "");

                write.set_task_val(ArgType::Subscript(Subscript::new_wrapped(
                    ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                        format!("tasks_{}", self.get_constraint_name()).to_string(),
                    )),
                    ArgType::StringLiteral(Arc::new(RwLock::new(StringLiteral::new(shorter_name)))),
                )));
            }
        }
        let singletons = self
            .get_constraints()
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                (
                    x.get_task_name(),
                    x.get_prefect_singleton(literals.clone()).unwrap(),
                )
            })
            .collect::<HashMap<String, _>>();
        singletons
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
    fn build_for_loop_singleton(
        &self,
        collector: &ArgType,
        call: &ArgType,
        args: &Vec<ArgType>,
        kwarg_keys: Vec<String>,
        params_constraint: &ArgType,
        v: &Vec<(
            ArgType,
            std::string::String,
            std::option::Option<ArgType>,
            Vec<ArgType>,
        )>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> PrefectSingleton {
        let params = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".to_string()));

        let param_map =
            self.build_for_loop_singleton_param_map(v, &kwarg_keys, params_constraint.clone());
        let mut dict = ArgType::Dict(Dict::new_wrapped(param_map));
        dict.set_owner(params_constraint.clone());

        let ident = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".to_string()));
        let tpl = ArgType::Tuple(Tuple::new_wrapped(vec![ident.clone(), params.clone()]));
        let new_collector =
            ArgType::Subscript(Subscript::new_wrapped(collector.clone(), ident.clone()));

        self.get_for_loop_singleton(
            &new_collector,
            &call,
            &args,
            &kwarg_keys,
            preamble,
            dialect,
            (tpl.clone(), dict.clone()),
        )
    }
    fn get_for_loop_singleton(
        &self,
        new_collector: &ArgType,
        call: &ArgType,
        args: &Vec<ArgType>,
        kwarg_keys: &Vec<String>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        dict: (ArgType, ArgType),
    ) -> PrefectSingleton {
        let function = ArgType::Attribute(Attribute::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("flow".to_string())),
            "add_node".to_string(),
        ));
        let add_expr = ArgType::Call(Call::new_wrapped(
            function,
            vec![new_collector.clone()],
            LinkedHashMap::new(),
        ));

        PrefectSingleton::new_referencing_dict(
            new_collector.clone(),
            call.clone(),
            args.clone(),
            kwarg_keys,
            AoristStatement::Expression(add_expr),
            preamble,
            dialect,
            dict,
        )
    }
    fn build_for_loop_singleton_param_map(
        &self,
        v: &Vec<(ArgType, String, Option<ArgType>, Vec<ArgType>)>,
        kwarg_keys: &Vec<String>,
        params_constraint: ArgType,
    ) -> LinkedHashMap<String, ArgType> {
        v.iter()
            .map(|x| {
                (
                    x.1.clone(),
                    match x.2.clone() {
                        Some(y) => match y {
                            ArgType::List { .. } => y.clone(),
                            ArgType::SimpleIdentifier { .. } | ArgType::Subscript { .. } => {
                                ArgType::List(List::new_wrapped(vec![y.clone()]))
                            }
                            _ => panic!(formatdoc!(
                                "{}. instead I found: {}",
                                "Only SimpleIdentifiers or Lists are valid dep_lists",
                                y.name()
                            )),
                        },
                        None => ArgType::List(List::new_wrapped(Vec::new())),
                    },
                    x.3.clone(),
                )
            })
            .map(|(k, dep_list, kwargs_values)| {
                let mut local_params_map: LinkedHashMap<String, ArgType> = LinkedHashMap::new();
                local_params_map.insert("dep_list".to_string(), dep_list);
                for (i, kw) in kwarg_keys.iter().enumerate() {
                    let val = kwargs_values.get(i).unwrap().clone();
                    local_params_map.insert(kw.to_string(), val.clone());
                    // check if any formatted values are subscripts
                    // from the params map
                    // TODO: move this to function
                    if let ArgType::Formatted(x) = val.clone() {
                        for kw_val in x.read().unwrap().keywords().values() {
                            if let Some(ArgType::Subscript(s)) = kw_val.get_owner() {
                                let read = s.read().unwrap();
                                let param_dict_a = ArgType::Subscript(Subscript::new_wrapped(
                                    params_constraint.clone(),
                                    ArgType::StringLiteral(StringLiteral::new_wrapped(k.clone())),
                                ));
                                if read.a() == param_dict_a {
                                    if let ArgType::StringLiteral(l) = read.b() {
                                        let mut kw_val_remove_indirection = kw_val.clone();
                                        if let Some(owner) = read.get_owner() {
                                            kw_val_remove_indirection.set_owner(owner);
                                            local_params_map.insert(
                                                l.read().unwrap().value(),
                                                kw_val_remove_indirection,
                                            );
                                        } else {
                                            kw_val_remove_indirection.remove_owner();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                (k, ArgType::Dict(Dict::new_wrapped(local_params_map)))
            })
            .collect::<LinkedHashMap<_, _>>()
    }
    pub fn render(&'a self, location: Location, literals: LiteralsMap, constraint_name: String) {
        let mut singletons = self.get_singletons(literals);
        let singletons_deconstructed = singletons
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.deconstruct()))
            .filter(|x| x.1.is_some())
            .map(|x| (x.0, x.1.unwrap()))
            .collect::<Vec<_>>();
        let mut singletons_hash: HashMap<_, Vec<_>> = HashMap::new();
        for (
            task_key,
            (collector, task_name, call, args, kwargs, _, edge_addition, preamble, dialect),
        ) in &singletons_deconstructed
        {
            // TODO: move this to PrefectSingleton -- this is the
            // "argument-free" bit of code in the Singleton
            let key = (
                collector.clone(),
                call.clone(),
                args.clone(),
                kwargs.keys().map(|x| x.clone()).collect::<Vec<String>>(),
                preamble.clone(),
                dialect.clone(),
            );
            // TODO: assert that task_name is the same as the 2nd value in the
            // tuple (when unpacked)
            // TODO: the replacement below is a huge hack
            singletons_hash.entry(key).or_insert(Vec::new()).push((
                task_name.clone(),
                task_key.clone().replace(
                    &format!("{}__", constraint_name).to_string(),
                    &"".to_string(),
                ),
                edge_addition.clone(),
                kwargs.values().map(|x| x.clone()).collect::<Vec<ArgType>>(),
            ));
        }
        let mut for_loop_singletons: Vec<PrefectSingleton> = Vec::new();
        if singletons_deconstructed.len() > 1 {
            let params_constraint = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                format!("params_{}", constraint_name).to_string(),
            ));
            for ((collector, call, args, kwarg_keys, preamble, dialect), v) in singletons_hash {
                // TODO: magic number
                if v.len() >= 3 {
                    let new_singleton = self.build_for_loop_singleton(
                        &collector,
                        &call,
                        &args,
                        kwarg_keys,
                        &params_constraint,
                        &v,
                        preamble,
                        dialect,
                    );
                    for elem in v.iter().map(|x| x.1.clone()) {
                        singletons.remove(&format!("{}__{}", constraint_name, elem).to_string());
                    }
                    for_loop_singletons.push(new_singleton);
                }
            }
        }
        // TODO: this is very hacky, should dedup by parsing the preambles
        let python_preambles: LinkedHashSet<String> = for_loop_singletons
            .iter()
            .chain(singletons.values())
            .map(|x| {
                if let Some(Dialect::Python(_)) = x.get_dialect() {
                    x.get_preamble()
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        for new_singleton in for_loop_singletons {
            println!(
                "{}\n",
                PrefectProgram::render_suite(
                    new_singleton
                        .get_assign_statements()
                        .into_iter()
                        .map(|x| x.statement(location))
                        .collect::<Vec<_>>()
                )
                .unwrap()
            );
        }
        for singleton in singletons.into_iter() {
            print!(
                "{}\n",
                PrefectProgram::render_suite(singleton.1.as_suite(location)).unwrap()
            );
        }
    }
    pub fn get_singletons(&self, literals: LiteralsMap) -> HashMap<String, PrefectSingleton> {
        match &self {
            PrefectRender::Python(x) => x.get_singletons(literals),
            PrefectRender::Shell(x) => x.get_singletons(literals),
            PrefectRender::Constant(x) => x.get_singletons(literals),
        }
    }
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

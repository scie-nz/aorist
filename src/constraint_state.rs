use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{
    AllConstraintsSatisfiability, AoristStatement, ArgType, Constraint, LiteralsMap,
    ParameterTuple, StringLiteral,
};
use crate::object::TAoristObject;
use aorist_primitives::Dialect;
use inflector::cases::snakecase::to_snake_case;
use rustpython_parser::ast::{Expression, ExpressionType, Located, Location, Statement, Suite};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct ConstraintState<'a> {
    dialect: Option<Dialect>,
    pub key: Option<String>,
    name: String,
    pub satisfied: bool,
    pub satisfied_dependencies: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    pub unsatisfied_dependencies: HashSet<(Uuid, String)>,
    constraint: Arc<RwLock<Constraint>>,
    root: Concept<'a>,
    // these are concept ancestors
    // TODO: change this to Vec<Concept<'a>>
    ancestors: Vec<(Uuid, String, Option<String>, usize)>,
    preamble: Option<String>,
    call: Option<String>,
    params: Option<ParameterTuple>,
    task_name: Option<String>,
    task_val: Option<ArgType>,
    //task_val_fn: Option<Box<dyn Fn(Location, String) -> Expression>>,
}

pub struct PrefectSingleton {
    task_creation: Statement,
    flow_addition: Suite,
}
impl PrefectSingleton {
    pub fn new(task_creation: Statement, flow_addition: Suite) -> Self {
        Self {
            task_creation,
            flow_addition,
        }
    }
    pub fn as_suite(self) -> Suite {
        let mut stmts = vec![self.task_creation];
        for stmt in self.flow_addition {
            stmts.push(stmt);
        }
        stmts
    }
}

impl<'a> ConstraintState<'a> {
    pub fn get_prefect_singleton(
        &self,
        location: Location,
        literals: LiteralsMap,
    ) -> Result<PrefectSingleton, String> {
        Ok(PrefectSingleton {
            task_creation: self.get_task_statement(literals)?.statement(location),
            flow_addition: self
                .get_flow_addition_statements()
                .into_iter()
                .map(|x| x.statement(location))
                .collect::<Vec<_>>(),
        })
    }
    pub fn get_dep_ident(&self, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Identifier {
                name: "dep".to_string(),
            },
        }
    }
    pub fn set_task_val(&mut self, val: ArgType) {
        self.task_val = Some(val);
    }
    pub fn get_task_val(&self) -> ArgType {
        self.task_val.as_ref().unwrap().clone()
    }
    fn get_flow_add_edge_statement(&self, dep: ArgType) -> AoristStatement {
        let function = ArgType::Attribute(
            Box::new(ArgType::SimpleIdentifier("flow".to_string())),
            "add_edge".to_string(),
        );
        let add_expr = ArgType::Call(
            Box::new(function),
            vec![self.get_task_val(), dep],
            HashMap::new(),
        );
        AoristStatement::Expression(add_expr)
    }
    pub fn get_flow_addition_statements(&self) -> Vec<AoristStatement> {
        let deps = self
            .satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val()
            })
            .collect::<Vec<_>>();
        let function = ArgType::Attribute(
            Box::new(ArgType::SimpleIdentifier("flow".to_string())),
            "add_node".to_string(),
        );
        let add_expr = ArgType::Call(
            Box::new(function),
            vec![self.get_task_val()],
            HashMap::new(),
        );
        let mut statements = vec![AoristStatement::Expression(add_expr)];

        if deps.len() == 1 {
            let dep = deps.into_iter().next().unwrap();
            let add_stmt = self.get_flow_add_edge_statement(dep);
            statements.push(add_stmt);
        } else if deps.len() > 1 {
            let dep_list = ArgType::List(deps);
            let target = ArgType::SimpleIdentifier("dep".to_string());
            let for_stmt = AoristStatement::For(
                target.clone(),
                dep_list,
                vec![self.get_flow_add_edge_statement(target.clone())],
            );
            statements.push(for_stmt);
        }
        statements
    }
    fn get_task_creation_expr(&self, literals: LiteralsMap) -> Result<ArgType, String> {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::Call(
                Box::new(ArgType::SimpleIdentifier(self.get_call().unwrap())),
                match self.params {
                    Some(ref p) => p.get_args(),
                    None => Vec::new(),
                },
                match self.params {
                    Some(ref p) => p.get_kwargs(),
                    None => HashMap::new(),
                },
            )),
            Some(Dialect::Presto(_)) => {
                /*let query = self
                    .params
                    .as_ref()
                    .unwrap()
                    .get_presto_query(self.get_call().unwrap().clone())
                    .replace("'", "\\'");
                let raw_command = format!("presto -e '{}'", query);
                let formatted_str = Located {
                    location,
                    node: ExpressionType::String {
                        value: StringGroup::Constant {
                            value: raw_command.to_string(),
                        },
                    },
                };*/
                // TODO: unify this with call in register_literals
                let raw_command = format!("presto -e '{}'", self.get_call().unwrap().clone());
                let format_string = literals.read().unwrap().get(&raw_command).unwrap().clone();
                let command = match self.params {
                    Some(ref p) => ArgType::Formatted(
                        Box::new(ArgType::StringLiteral(format_string)),
                        p.get_kwargs(),
                    ),
                    None => ArgType::StringLiteral(format_string),
                };
                let mut keywords: HashMap<String, ArgType> = HashMap::new();
                keywords.insert("command".to_string(), command);
                Ok(ArgType::Call(
                    Box::new(ArgType::SimpleIdentifier("ShellTask".to_string())),
                    Vec::new(),
                    keywords,
                ))
            }
            Some(Dialect::Bash(_)) => {
                let format_string = literals
                    .read()
                    .unwrap()
                    .get(&self.get_call().unwrap())
                    .unwrap()
                    .clone();
                let command = match self.params {
                    Some(ref p) => ArgType::Formatted(
                        Box::new(ArgType::StringLiteral(format_string)),
                        p.get_kwargs(),
                    ),
                    None => ArgType::StringLiteral(format_string),
                };
                let mut keywords: HashMap<String, ArgType> = HashMap::new();
                keywords.insert("command".to_string(), command);

                Ok(ArgType::Call(
                    Box::new(ArgType::SimpleIdentifier("ShellTask".to_string())),
                    Vec::new(),
                    keywords,
                ))
            }
            None => Ok(ArgType::Call(
                Box::new(ArgType::SimpleIdentifier("ConstantTask".to_string())),
                vec![ArgType::StringLiteral(Arc::new(RwLock::new(
                    StringLiteral::new(self.constraint.read().unwrap().get_name().clone()),
                )))],
                HashMap::new(),
            )),
            _ => Err("Dialect not supported".to_string()),
        }
    }
    pub fn get_task_statement(&self, literals: LiteralsMap) -> Result<AoristStatement, String> {
        Ok(AoristStatement::Assign(
            self.get_task_val(),
            self.get_task_creation_expr(literals)?,
        ))
    }
    pub fn set_task_name(&mut self, name: String) {
        self.task_name = Some(name)
    }
    pub fn get_task_name(&self) -> String {
        self.task_name.as_ref().unwrap().clone()
    }
    pub fn get_satisfied_dependency_keys(&self) -> Vec<String> {
        self.satisfied_dependencies
            .iter()
            .map(|x| x.read().unwrap().get_task_name())
            .collect()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[allow(dead_code)]
    pub fn get_root(&self) -> Concept<'a> {
        self.root.clone()
    }
    pub fn get_constraint_uuid(&self) -> Uuid {
        self.constraint.read().unwrap().get_uuid().clone()
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> Uuid {
        self.root.get_uuid().clone()
    }
    pub fn get_root_type(&self) -> String {
        self.root.get_type()
    }
    pub fn get_ancestors(&self) -> Vec<(Uuid, String, Option<String>, usize)> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_params(&self) -> Option<ParameterTuple> {
        self.params.clone()
    }
    pub fn get_call(&self) -> Option<String> {
        self.call.clone()
    }
    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    pub fn satisfy(
        &mut self,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
        literals: Arc<RwLock<HashMap<String, Arc<RwLock<StringLiteral>>>>>,
    ) {
        let root_clone = self.root.clone();
        let mut constraint = self.constraint.write().unwrap();
        let (preamble, call, params, dialect) = constraint
            .satisfy_given_preference_ordering(root_clone, preferences, ancestry, literals)
            .unwrap();
        drop(constraint);
        self.preamble = Some(preamble);
        self.call = Some(call);
        self.params = Some(params);
        self.dialect = Some(dialect);
    }
    pub fn new(
        constraint: Arc<RwLock<Constraint>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        concept_ancestors: &HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>>,
    ) -> Self {
        let arc = constraint.clone();
        let x = arc.read().unwrap();
        let root_uuid = x.get_root_uuid();
        let guard = concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), x.root.clone()))
            .unwrap()
            .clone();
        let dependencies = x
            .get_downstream_constraints_ignore_chains()
            .iter()
            .map(|x| (x.read().unwrap().get_uuid(), x.read().unwrap().root.clone()))
            .collect::<HashSet<_>>();
        let ancestors = concept_ancestors
            .get(&(root_uuid, x.root.clone()))
            .unwrap()
            .clone();
        Self {
            dialect: None,
            key: None,
            name: x.get_name().clone(),
            satisfied: false,
            unsatisfied_dependencies: dependencies,
            satisfied_dependencies: Vec::new(),
            constraint,
            root,
            ancestors: ancestors.clone(),
            preamble: None,
            call: None,
            params: None,
            task_name: None,
            task_val: None,
        }
    }
    pub fn compute_task_name(
        &mut self,
        ancestors: &Vec<(Uuid, String, Option<String>, usize)>,
    ) -> String {
        self.key = Some(match self.root.get_tag() {
            None => {
                let mut relative_path: String = "".to_string();
                for (_, ancestor_type, tag, ix) in ancestors.iter().rev() {
                    if let Some(t) = tag {
                        relative_path = format!("{}__{}", relative_path, t);
                        break;
                    }
                    if *ix > 0 {
                        relative_path = format!(
                            "{}__{}_{}",
                            relative_path,
                            to_snake_case(&ancestor_type),
                            ix
                        );
                    }
                }
                relative_path
            }
            Some(t) => t,
        });
        self.key.as_ref().unwrap().clone()
    }
    pub fn get_fully_qualified_task_name(&self) -> String {
        format!(
            "{}__{}",
            to_snake_case(&self.get_name()),
            self.key.as_ref().unwrap()
        )
    }
}

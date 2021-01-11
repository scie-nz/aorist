use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{
    AllConstraintsSatisfiability, AoristStatement, ArgType, Attribute, Call, Constraint, Formatted,
    List, LiteralsMap, ParameterTuple, SimpleIdentifier, StringLiteral,
    Subscript,
};
use crate::object::TAoristObject;
use aorist_primitives::Dialect;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Expression, ExpressionType, Located, Location, Suite};
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PrefectSingleton {
    task_val: ArgType,
    task_call: ArgType,
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
    flow_node_addition: AoristStatement,
    dep_list: Option<ArgType>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
}
impl PrefectSingleton {
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    pub fn get_task_val(&self) -> ArgType {
        self.task_val.clone()
    }
    fn get_flow_add_edge_statement(&self, dep: ArgType) -> AoristStatement {
        let function = ArgType::Attribute(Attribute::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("flow".to_string())),
            "add_edge".to_string(),
        ));
        let add_expr = ArgType::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val(), dep],
            LinkedHashMap::new(),
        ));
        AoristStatement::Expression(add_expr)
    }
    pub fn get_edge_addition_statements(&self) -> Vec<AoristStatement> {
        match self.dep_list {
            None => Vec::new(),
            Some(ArgType::List(_)) => {
                let target =
                    ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("dep".to_string()));
                let for_stmt = AoristStatement::For(
                    target.clone(),
                    self.dep_list.as_ref().unwrap().clone(),
                    vec![self.get_flow_add_edge_statement(target.clone())],
                );
                vec![for_stmt]
            }
            _ => {
                let dep = self.dep_list.clone();
                let add_stmt = self.get_flow_add_edge_statement(dep.unwrap());
                vec![add_stmt]
            }
        }
    }
    pub fn get_flow_node_addition(&self) -> AoristStatement {
        self.flow_node_addition.clone()
    }
    pub fn deconstruct(
        &self,
    ) -> Option<(
        ArgType,
        ArgType,
        ArgType,
        Vec<ArgType>,
        LinkedHashMap<String, ArgType>,
        AoristStatement,
        Option<ArgType>,
        Option<String>,
        Option<Dialect>,
    )> {
        if let ArgType::Subscript(ref subscript) = self.task_val {
            let guard = subscript.read().unwrap();
            return Some((
                guard.a().clone(),
                guard.b().clone(),
                self.task_call.clone(),
                self.args.clone(),
                self.kwargs.clone(),
                self.get_flow_node_addition().clone(),
                self.dep_list.clone(),
                self.preamble.clone(),
                self.dialect.clone(),
            ));
        }
        None
    }
    pub fn new(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        flow_node_addition: AoristStatement,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_val,
            task_call,
            args,
            kwargs,
            flow_node_addition,
            dep_list,
            preamble,
            dialect,
        }
    }
    pub fn new_referencing_dict(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwarg_keys: &Vec<String>,
        flow_node_addition: AoristStatement,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        params: ArgType,
    ) -> Self {
        // HACK
        let kwargs = kwarg_keys
            .iter()
            .map(|x| {
                (
                    x.clone(),
                    ArgType::Subscript(Subscript::new_wrapped(
                        params.clone(),
                        ArgType::StringLiteral(StringLiteral::new_wrapped(x.to_string())),
                    )),
                )
            })
            .collect::<LinkedHashMap<_, _>>();
        let mut future_list = ArgType::List(List::new_wrapped(vec![]));
        future_list.set_owner(ArgType::Subscript(Subscript::new_wrapped(
            params.clone(),
            ArgType::StringLiteral(StringLiteral::new_wrapped("dep_list".to_string())),
        )));
        Self::new(
            task_val,
            task_call,
            args,
            kwargs,
            flow_node_addition,
            Some(future_list),
            preamble,
            dialect,
        )
    }
    pub fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = ArgType::Call(Call::new_wrapped(
            self.task_call.clone(),
            self.args.clone(),
            self.kwargs.clone(),
        ));
        let task_creation = AoristStatement::Assign(self.task_val.clone(), creation_expr);
        let mut stmts = vec![task_creation];
        stmts.push(self.flow_node_addition.clone());
        for stmt in self.get_edge_addition_statements() {
            stmts.push(stmt);
        }
        stmts
    }
    pub fn as_suite(self, location: Location) -> Suite {
        self.get_statements()
            .into_iter()
            .map(|x| x.statement(location))
            .collect::<Vec<_>>()
    }
}

impl<'a> ConstraintState<'a> {
    pub fn get_prefect_singleton(&self, literals: LiteralsMap) -> Result<PrefectSingleton, String> {
        let (flow_node_addition, dep_list) = self.get_flow_addition_statements();
        let dep;
        if dep_list.len() == 1 {
            dep = Some(dep_list.clone().into_iter().next().unwrap());
        } else if dep_list.len() > 1 {
            dep = Some(ArgType::List(Arc::new(RwLock::new(List::new(dep_list)))));
        } else {
            dep = None;
        }
        Ok(PrefectSingleton::new(
            self.get_task_val(),
            self.get_task_call()?,
            self.get_args_vec()?,
            self.get_kwargs_map(literals)?,
            flow_node_addition,
            dep,
            self.get_preamble(),
            self.get_dialect(),
        ))
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
    pub fn get_flow_addition_statements(&self) -> (AoristStatement, Vec<ArgType>) {
        let deps = self
            .satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val()
            })
            .collect::<Vec<ArgType>>();
        let function = ArgType::Attribute(Attribute::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("flow".to_string())),
            "add_node".to_string(),
        ));
        let add_expr = ArgType::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val()],
            LinkedHashMap::new(),
        ));
        let node_stmt = AoristStatement::Expression(add_expr);
        (node_stmt, deps)
    }
    fn get_task_call(&self) -> Result<ArgType, String> {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(self.get_call().unwrap()),
            )),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".to_string()),
            )),
            None => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "ConstantTask".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
    }
    fn get_args_vec(&self) -> Result<Vec<ArgType>, String> {
        match (&self.params, &self.dialect) {
            (Some(ref p), Some(Dialect::Python(_))) => Ok(p.get_args()),
            (None, Some(Dialect::Python(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Presto(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Bash(_))) => Ok(Vec::new()),
            (_, None) => Ok(vec![ArgType::StringLiteral(StringLiteral::new_wrapped(
                self.constraint.read().unwrap().get_name().clone(),
            ))]),
            _ => Err("Dialect not supported".to_string()),
        }
    }
    fn get_kwargs_map(
        &self,
        literals: LiteralsMap,
    ) -> Result<LinkedHashMap<String, ArgType>, String> {
        match &self.dialect {
            Some(Dialect::Python(_)) => match self.params {
                Some(ref p) => Ok(p.get_kwargs()),
                None => Ok(LinkedHashMap::new()),
            },
            Some(Dialect::Presto(_)) => {
                let raw_command = format!("presto -e '{}'", self.get_call().unwrap().clone());
                let format_string = literals.read().unwrap().get(&raw_command).unwrap().clone();
                let command = match self.params {
                    Some(ref p) => ArgType::Formatted(Formatted::new_wrapped(
                        ArgType::StringLiteral(format_string),
                        p.get_kwargs(),
                    )),
                    None => ArgType::StringLiteral(format_string),
                };
                let mut keywords: LinkedHashMap<String, ArgType> = LinkedHashMap::new();
                keywords.insert("command".to_string(), command);
                Ok(keywords)
            }
            Some(Dialect::Bash(_)) => {
                let format_string = literals
                    .read()
                    .unwrap()
                    .get(&self.get_call().unwrap())
                    .unwrap()
                    .clone();
                let command = match self.params {
                    Some(ref p) => ArgType::Formatted(Formatted::new_wrapped(
                        ArgType::StringLiteral(format_string),
                        p.get_kwargs(),
                    )),
                    None => ArgType::StringLiteral(format_string),
                };
                let mut keywords: LinkedHashMap<String, ArgType> = LinkedHashMap::new();
                keywords.insert("command".to_string(), command);
                Ok(keywords)
            }
            None => Ok(LinkedHashMap::new()),
            _ => Err("Dialect not supported".to_string()),
        }
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
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
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

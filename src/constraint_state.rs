use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, Constraint, ParameterTuple};
use crate::object::TAoristObject;
use aorist_primitives::Dialect;
use inflector::cases::snakecase::to_snake_case;
use rustpython_parser::ast::{
    Expression, ExpressionType, Keyword, Located, Location, Statement, StatementType, StringGroup,
    Suite,
};
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
}

impl<'a> ConstraintState<'a> {
    pub fn get_prefect_singleton(&self) -> Result<Suite, String> {
        let location = Location::new(0, 0);
        let mut stmts = vec![self.get_task_statement(location)?];
        for stmt in self.get_flow_addition_statements(location) {
            stmts.push(stmt);
        }
        Ok(stmts)
    }
    pub fn get_dep_ident(&self, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Identifier {
                name: "dep".to_string(),
            },
        }
    }
    pub fn get_task_val(&self, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Identifier {
                name: self.get_task_name(),
            },
        }
    }
    pub fn _get_task_val_dict(&self, location: Location) -> Expression {
        let outer = Located {
            location,
            node: ExpressionType::Identifier {
                name: "tasks".to_string(),
            },
        };
        let inner = Located {
            location,
            node: ExpressionType::String {
                value: StringGroup::Constant {
                    value: self.get_task_name(),
                },
            },
        };
        Located {
            location,
            node: ExpressionType::Subscript {
                a: Box::new(outer),
                b: Box::new(inner),
            },
        }
    }
    fn get_flow_add_edge_statement(&self, location: Location, dep: Expression) -> Statement {
        let add_edge_fn = Located {
            location,
            node: ExpressionType::Attribute {
                value: Box::new(Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "flow".to_string(),
                    },
                }),
                name: "add_edge".to_string(),
            },
        };
        let add_edge = Located {
            location,
            node: ExpressionType::Call {
                function: Box::new(add_edge_fn),
                args: vec![dep, self.get_task_val(location)],
                keywords: Vec::new(),
            },
        };
        Located {
            location,
            node: StatementType::Expression {
                expression: add_edge,
            },
        }
    }
    pub fn get_flow_addition_statements(&self, location: Location) -> Vec<Statement> {
        let deps = self
            .satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val(location)
            })
            .collect::<Vec<Expression>>();
        let function = Located {
            location,
            node: ExpressionType::Attribute {
                value: Box::new(Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "flow".to_string(),
                    },
                }),
                name: "add_node".to_string(),
            },
        };
        let add_expr = Located {
            location,
            node: ExpressionType::Call {
                function: Box::new(function),
                args: vec![self.get_task_val(location)],
                keywords: Vec::new(),
            },
        };
        let mut statements = vec![Located {
            location,
            node: StatementType::Expression {
                expression: add_expr,
            },
        }];

        if deps.len() == 1 {
            let dep = deps.into_iter().next().unwrap();
            let add_stmt = self.get_flow_add_edge_statement(location, dep);
            statements.push(add_stmt);
        } else if deps.len() > 1 {
            let dep_list = Located {
                location,
                node: ExpressionType::List { elements: deps },
            };
            let add_stmt_suite =
                vec![self.get_flow_add_edge_statement(location, self.get_dep_ident(location))];
            let for_stmt = Located {
                location,
                node: StatementType::For {
                    is_async: false,
                    target: Box::new(self.get_dep_ident(location)),
                    iter: Box::new(dep_list),
                    body: add_stmt_suite,
                    orelse: None,
                },
            };
            statements.push(for_stmt);
        }
        statements
    }
    fn get_task_creation_expr(&self, location: Location) -> Result<Expression, String> {
        match self.dialect {
            Some(Dialect::Python(_)) => {
                let function = Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: self.get_call().unwrap(),
                    },
                };
                Ok(self
                    .params
                    .as_ref()
                    .unwrap()
                    .populate_call(function, location))
            }
            Some(Dialect::Presto(_)) => {
                let query = self
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
                };
                let function = Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "ShellTask".to_string(),
                    },
                };
                Ok(Located {
                    location,
                    node: ExpressionType::Call {
                        function: Box::new(function),
                        args: Vec::new(),
                        keywords: vec![Keyword {
                            name: Some("command".to_string()),
                            value: formatted_str,
                        }],
                    },
                })
            }
            Some(Dialect::Bash(_)) => {
                let formatted_str = self
                    .params
                    .as_ref()
                    .unwrap()
                    .get_shell_task_command(location, self.get_call().unwrap().clone());

                let function = Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "ShellTask".to_string(),
                    },
                };
                Ok(Located {
                    location,
                    node: ExpressionType::Call {
                        function: Box::new(function),
                        args: Vec::new(),
                        keywords: vec![Keyword {
                            name: Some("command".to_string()),
                            value: formatted_str,
                        }],
                    },
                })
            }
            None => {
                let function = Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "ConstantTask".to_string(),
                    },
                };
                Ok(Located {
                    location,
                    node: ExpressionType::Call {
                        function: Box::new(function),
                        args: vec![Located {
                            location,
                            node: ExpressionType::String {
                                value: StringGroup::Constant {
                                    value: self.constraint.read().unwrap().get_name().clone(),
                                },
                            },
                        }],
                        keywords: Vec::new(),
                    },
                })
            }
            _ => Err("Dialect not supported".to_string()),
        }
    }
    pub fn get_task_statement(&self, location: Location) -> Result<Statement, String> {
        let val = self.get_task_val(location);
        let assign = StatementType::Assign {
            targets: vec![val],
            value: self.get_task_creation_expr(location)?,
        };
        Ok(Located {
            location,
            node: assign,
        })
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
    pub fn satisfy(&mut self, preferences: &Vec<Dialect>, ancestry: Arc<ConceptAncestry<'a>>) {
        let root_clone = self.root.clone();
        let constraint = self.constraint.read().unwrap();
        let (preamble, call, params, dialect) = constraint
            .satisfy_given_preference_ordering(root_clone, preferences, ancestry)
            .unwrap();
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

#![allow(dead_code)]
use crate::constraint_state::ConstraintState;
use indoc::formatdoc;
use rustpython_parser::ast::{
    Expression, ExpressionType, Located, Location, Program, Statement, StatementType, StringGroup,
    WithItem,
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
    fn render_expr(expr: &Expression) -> Result<String, String> {
        match &expr.node {
            ExpressionType::String {
                value: StringGroup::Constant { value },
            } => Ok(format!("'{}'", value).to_string()),
            ExpressionType::Subscript { a, b, .. } => {
                Ok(format!("{}[{}]", Self::render_expr(a)?, Self::render_expr(b)?).to_string())
            }
            ExpressionType::Identifier { name } => Ok(name.to_string()),
            ExpressionType::Attribute { value, name } => {
                Ok(format!("{}.{}", Self::render_expr(&value)?, name).to_string())
            }
            ExpressionType::Call { function, args, .. } => {
                let mut formatted_args: Vec<String> = Vec::new();
                for arg in args {
                    formatted_args.push(Self::render_expr(&arg)?);
                }
                Ok(format!(
                    "{}({})",
                    call = Self::render_expr(function)?,
                    args = formatted_args.join(", ")
                )
                .to_string())
            }
            _ => Err(format!("Unknown argument: {}", Expression::name(&expr))),
        }
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
            _ => Err("Unknown statement type.".to_string()),
        }
    }
}

pub trait PrefectTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>>;
    fn render_singleton(&self, constraint_name: String);
    fn render_ids(ids: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> String {
        ids.iter()
            .map(|x| format!("'{}'", x.read().unwrap().get_task_name()).to_string())
            .collect::<Vec<String>>()
            .join(",\n    ")
    }
    fn render_dependencies(&self, constraint_name: String) -> Option<String> {
        if self
            .get_constraints()
            .iter()
            .map(|x| x.read().unwrap().get_satisfied_dependency_keys())
            .flatten()
            .next()
            .is_none()
        {
            return None;
        }
        Some(formatdoc!(
            "
        dependencies_{constraint} = {{ 
            {dependencies} 
        }}
        ",
            constraint = constraint_name,
            dependencies = self
                .get_constraints()
                .iter()
                .map(|rw| {
                    let x = rw.read().unwrap();
                    formatdoc!(
                        "
               '{key}': [
                   {deps}
               ]",
                        key = x.get_task_name(),
                        deps = x
                            .get_satisfied_dependency_keys()
                            .iter()
                            .map(|x| format!("'{}'", x))
                            .collect::<Vec<_>>()
                            .join(",\n    "),
                    )
                    .to_string()
                    .replace("\n", "\n    ")
                })
                .collect::<Vec<_>>()
                .join(",\n    "),
        ))
    }
}
pub trait PrefectTaskRenderWithCalls<'a>: PrefectTaskRender<'a> {
    fn extract_call_for_rendering(rw: Arc<RwLock<ConstraintState<'a>>>) -> Option<String> {
        rw.read().unwrap().get_call()
    }
    fn render(&self, constraint_name: String) {
        if self.get_constraints().len() == 1 {
            self.render_singleton(constraint_name);
        } else {
            let mut by_call: HashMap<String, Vec<Arc<RwLock<ConstraintState<'a>>>>> =
                HashMap::new();
            for rw in self.get_constraints() {
                let maybe_call = Self::extract_call_for_rendering(rw.clone());
                if let Some(call) = maybe_call {
                    by_call.entry(call).or_insert(Vec::new()).push(rw.clone());
                }
            }
            if by_call.len() == 1 {
                self.render_single_call(
                    by_call.keys().next().unwrap().clone(),
                    constraint_name.clone(),
                )
            } else if by_call.len() > 1 {
                for (call, rws) in by_call.iter() {
                    self.render_multiple_calls(call.clone(), constraint_name.clone(), rws);
                }
            }
        }
    }
    fn render_single_call(&self, call_name: String, constraint_name: String);
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    );
}

pub struct PrefectPythonTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> PrefectTaskRender<'a> for PrefectPythonTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn render_singleton(&self, _constraint_name: String) {
        assert_eq!(self.get_constraints().len(), 1);
        let rw = self.get_constraints().get(0).unwrap();
        let constraint = rw.read().unwrap();
        let deps = constraint.get_satisfied_dependency_keys();

        let location = Location::new(0, 0);
        let val = PrefectProgram::render_expr(&constraint.get_task_val(location)).unwrap();
        let assign =
            PrefectProgram::render_statement(constraint.get_task_statement(location)).unwrap();
        let addition = constraint.get_flow_addition_statement(location);
        if deps.len() > 0 {
            let formatted_deps = deps
                .iter()
                .map(|x| format!("tasks['{}']", x))
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "{}",
                formatdoc!(
                    "
                {assign}
                {addition}
                for dep in [{dependencies}]:
                    flow.add_edge(dep, {val})
            ",
                    assign = assign,
                    dependencies = formatted_deps,
                    val = val,
                    addition = PrefectProgram::render_statement(addition).unwrap(),
                )
            )
        } else {
            println!(
                "{}",
                formatdoc!(
                    "
                {assign}
                {addition}
            ",
                    assign = assign,
                    addition = PrefectProgram::render_statement(addition).unwrap(),
                )
            )
        }
    }
}
impl<'a> PrefectTaskRenderWithCalls<'a> for PrefectPythonTaskRender<'a> {
    fn render_single_call(&self, call_name: String, constraint_name: String) {
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
            {dependencies}
            for k, v in params_{constraint}.items():
                tasks[k] = {call}(*v)
                flow.add_node(tasks[k])
                for dep in dependencies_{constraint}[k]:
                    flow.add_edge(tasks[dep], tasks[k])
            ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
            for k, v in params_{constraint}.items():
                tasks[k] = {call}(*v)
                flow.add_node(tasks[k])
            ",
                    constraint = constraint_name,
                    call = call_name,
                )
            ),
        }
    }
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    ) {
        let ids = Self::render_ids(rws.clone());
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                    {dependencies}
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = {call}(*params_{constraint}[k])
                        flow.add_node(tasks[k])
                        for dep in dependencies_{constraint}[k]:
                            flow.add_edge(tasks[dep], tasks[k])
                    ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = {call}(*params_{constraint}[k])
                        flow.add_node(tasks[k])
                    ",
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
        }
    }
}
impl<'a> PrefectPythonTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { members }
    }
}
pub struct PrefectShellTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> PrefectTaskRender<'a> for PrefectShellTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn render_singleton(&self, constraint_name: String) {
        assert_eq!(self.get_constraints().len(), 1);
        let rw = self.get_constraints().get(0).unwrap();
        let call_name = Self::extract_call_for_rendering(rw.clone()).unwrap();
        let constraint = rw.read().unwrap();
        let key = constraint.get_task_name();
        let deps = constraint.get_satisfied_dependency_keys();
        if deps.len() > 0 {
            let formatted_deps = deps
                .iter()
                .map(|x| format!("tasks['{}']", x))
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "{}",
                formatdoc!(
                    "
                flow.add_node(tasks['{k}'])
                tasks['{k}'] = ShellTask(
                    command=\"\"\"
                    {call} %s
                    \"\"\" % params_{constraint}['{k}'].join(' '),
                )
                flow.add_node(tasks['{k}'])
                for dep in [{dependencies}]:
                    flow.add_edge(dep, tasks['{k}'])
            ",
                    dependencies = formatted_deps,
                    k = key,
                    constraint = constraint_name,
                    call = call_name,
                )
            )
        } else {
            println!(
                "{}",
                formatdoc!(
                    "
                tasks['{k}'] = ShellTask(
                    command=\"\"\"
                    {call} %s
                    \"\"\" % params_{constraint}['{k}'].join(' '),
                )
                flow.add_node(tasks['{k}'])
            ",
                    k = key,
                    call = call_name,
                    constraint = constraint_name,
                )
            )
        }
    }
}
impl<'a> PrefectTaskRenderWithCalls<'a> for PrefectShellTaskRender<'a> {
    fn extract_call_for_rendering(rw: Arc<RwLock<ConstraintState<'a>>>) -> Option<String> {
        let maybe_call = rw.read().unwrap().get_call();
        match maybe_call {
            Some(call) => {
                let maybe_preamble = rw.read().unwrap().get_preamble();
                match maybe_preamble {
                    Some(preamble) => Some(format!("{}\n{}", preamble, call).to_string()),
                    None => Some(call),
                }
            }
            None => None,
        }
    }
    fn render_single_call(&self, call_name: String, constraint_name: String) {
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                        {dependencies}
                        for k, v in params_{constraint}.items():
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                {call} %s
                                \"\"\" % v.join(' '),
                            )
                            flow.add_node(tasks[k])
                            for dep in dependencies_{constraint}[k]:
                                flow.add_edge(tasks[dep], tasks[k])
                        ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name.replace("\n", "\n        "),
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                        for k, v in params_{constraint}.items():
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                {call} %s
                                \"\"\" % v.join(' '),
                            )
                            flow.add_node(tasks[k])
                        ",
                    constraint = constraint_name,
                    call = call_name.replace("\n", "\n        "),
                )
            ),
        }
    }
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    ) {
        let ids = Self::render_ids(rws.clone());
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                        {dependencies}
                        for k in [{ids}]:
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                    {call} %s
                                \"\"\" % params_{constraint}[k].join(' '),
                            )
                            flow.add_node(tasks[k])
                            for dep in dependencies_{constraint}[k]:
                                flow.add_edge(tasks[dep], tasks[k])
                        ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                        for k in [{ids}]:
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                    {call} %s
                                \"\"\" % params_{constraint}[k].join(' '),
                            )
                            flow.add_node(tasks[k])
                        ",
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
        }
    }
}
impl<'a> PrefectShellTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { members }
    }
}
pub struct PrefectConstantTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> PrefectConstantTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { members }
    }
    pub fn render(&self, constraint_name: String) {
        if self.get_constraints().len() == 1 {
            self.render_singleton(constraint_name);
        } else {
            let ids = Self::render_ids(self.get_constraints().clone());
            match self.render_dependencies(constraint_name.clone()) {
                Some(dependencies) => println!(
                    "{}",
                    formatdoc!(
                        "
                    {dependencies}
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = ConstantTask('{constraint}')
                        flow.add_node(tasks[k])
                        for dep in dependencies_{constraint}[k]:
                            flow.add_edge(tasks[dep], tasks[k])
                    ",
                        constraint = constraint_name,
                        dependencies = dependencies,
                        ids = ids,
                    )
                ),
                None => println!(
                    "{}",
                    formatdoc!(
                        "
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = ConstantTask('{constraint}')
                        flow.add_node(tasks[k])
                    ",
                        constraint = constraint_name,
                        ids = ids,
                    )
                ),
            }
        }
    }
}
impl<'a> PrefectTaskRender<'a> for PrefectConstantTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn render_singleton(&self, constraint_name: String) {
        assert_eq!(self.get_constraints().len(), 1);
        let rw = self.get_constraints().get(0).unwrap();
        let constraint = rw.read().unwrap();
        let key = constraint.get_task_name();
        let deps = constraint.get_satisfied_dependency_keys();
        if deps.len() > 0 {
            let formatted_deps = deps
                .iter()
                .map(|x| format!("tasks['{}']", x))
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "{}",
                formatdoc!(
                    "
                tasks['{k}'] = ConstantTask('{constraint}')
                flow.add_node(tasks['{k}'])
                for dep in [{dependencies}]:
                    flow.add_edge(dep, tasks['{k}'])
            ",
                    dependencies = formatted_deps,
                    k = key,
                    constraint = constraint_name,
                )
            )
        } else {
            println!(
                "{}",
                formatdoc!(
                    "
                tasks['{k}'] = ConstantTask('{constraint}')
                flow.add_node(tasks['{k}'])
            ",
                    k = key,
                    constraint = constraint_name,
                )
            )
        }
    }
}

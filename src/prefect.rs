#![allow(dead_code)]
use rustpython_parser::ast::{
    ExpressionType, Located, Program, Statement, StatementType, WithItem,
};
use indoc::formatdoc;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::constraint_state::ConstraintState;


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
}

pub trait PrefectTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>>;
    fn render_ids(ids: Vec<Arc<RwLock<ConstraintState<'a>>>>, constraint_name: String) -> String {
        ids.iter()
            .map(|x| {
                format!(
                    "'{constraint_name}_{id}'",
                    constraint_name = constraint_name,
                    id = x.read().unwrap().get_key().unwrap()
                )
                .to_string()
            })
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
        let mut by_call: HashMap<String, Vec<Arc<RwLock<ConstraintState<'a>>>>> = HashMap::new();
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
        let ids = Self::render_ids(rws.clone(), constraint_name.clone());
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
        let ids = Self::render_ids(rws.clone(), constraint_name.clone());
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
        let ids = Self::render_ids(self.get_constraints().clone(), constraint_name.clone());
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
impl<'a> PrefectTaskRender<'a> for PrefectConstantTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
}


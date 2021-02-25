use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::python::{
    Add, Assignment, Attribute, BigIntLiteral, BinOp, Call, Dict, ForLoop, Import, List,
    ParameterTuple, ParameterTupleDedupKey, SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandaloneETLTask<T>
where
    T: ETLSingleton,
{
    /// where the task creation call should be stored.
    task_val: AST,
    /// unique task identifier
    task_id: String,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: Option<String>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dependencies: Vec<AST>,
    /// Python preamble used by this task call
    preamble: Option<String>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
    singleton_type: PhantomData<T>,
}
/// tuple of:
/// - name of dict / list in which task_val is stored (must be dict or list)
/// - function call (if any)
/// - from parameters:
///   - number of args
///   - names of kwargs
/// - preamble
/// - dialect
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ETLTaskCompressionKey {
    // dict name
    dict_name: AST,
    // function call
    function_call: Option<String>,
    // dedup key from parameters
    dedup_key: Option<ParameterTupleDedupKey>,
    // preamble
    preamble: Option<String>,
    // dialect
    dialect: Option<Dialect>,
    // optional: dependencies
    pub deps: Vec<AST>,
    // optional: kwargs
    pub kwargs: LinkedHashMap<String, AST>,
}
impl ETLTaskCompressionKey {
    pub fn new(
        dict_name: AST,
        function_call: Option<String>,
        dedup_key: Option<ParameterTupleDedupKey>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            dict_name,
            function_call,
            dedup_key,
            preamble,
            dialect,
            deps: Vec::new(),
            kwargs: LinkedHashMap::new(),
        }
    }
    pub fn get_dict_name(&self) -> AST {
        self.dict_name.clone()
    }
    pub fn get_dedup_key(&self) -> Option<ParameterTupleDedupKey> {
        self.dedup_key.clone()
    }
    pub fn get_call(&self) -> Option<String> {
        self.function_call.clone()
    }
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
}
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ETLTaskUncompressiblePart<T>
where
    T: ETLSingleton,
{
    // unique task_id
    pub task_id: String,
    // dict value
    pub dict: String,
    // params
    pub params: Option<ParameterTuple>,
    // dep list
    pub deps: Vec<AST>,
    singleton_type: PhantomData<T>,
}

impl<T> ETLTaskUncompressiblePart<T>
where
    T: ETLSingleton,
{
    pub fn new(
        task_id: String,
        dict: String,
        params: Option<ParameterTuple>,
        deps: Vec<AST>,
    ) -> Self {
        Self {
            task_id,
            dict,
            params,
            deps,
            singleton_type: PhantomData,
        }
    }
    pub fn as_python_dict(&self, dependencies_as_list: bool, insert_task_name: bool) -> AST {
        let mut local_params_map: LinkedHashMap<String, AST> = LinkedHashMap::new();
        if self.deps.len() > 0 {
            let dependencies = match dependencies_as_list {
                true => AST::List(List::new_wrapped(self.deps.clone(), false)),
                false => {
                    assert_eq!(self.deps.len(), 1);
                    self.deps.get(0).unwrap().clone()
                }
            };
            local_params_map.insert("dependencies".to_string(), dependencies);
        }
        // TODO: get_type should return an enum
        if insert_task_name && T::get_type() == "airflow".to_string() {
            local_params_map.insert(
                "task_id".to_string(),
                AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            );
        }
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}

impl<T> StandaloneETLTask<T>
where
    T: ETLSingleton,
{
    /// only return true for compressible tasks, i.e. those that have a
    /// dict task val (in the future more stuff could be added here)
    pub fn is_compressible(&self) -> bool {
        match &self.task_val {
            AST::Subscript(_) => true,
            _ => false,
        }
    }
    fn get_left_of_task_val(&self) -> Result<AST, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                Ok(rw.a().clone())
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    fn get_right_of_task_val(&self) -> Result<String, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                match &rw.b() {
                    AST::StringLiteral(l) => Ok(l.read().unwrap().value().clone()),
                    _ => Err("Right of subscript must be a string
                    literal"
                        .to_string()),
                }
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    pub fn get_compression_key(&self) -> Result<ETLTaskCompressionKey, String> {
        Ok(ETLTaskCompressionKey::new(
            self.get_left_of_task_val()?,
            self.call.clone(),
            match &self.params {
                Some(p) => Some(p.get_dedup_key()),
                None => None,
            },
            self.preamble.clone(),
            self.dialect.clone(),
        ))
    }
    pub fn get_uncompressible_part(&self) -> Result<ETLTaskUncompressiblePart<T>, String> {
        Ok(ETLTaskUncompressiblePart::new(
            self.task_id.clone(),
            self.get_right_of_task_val()?,
            self.params.clone(),
            self.dependencies.clone(),
        ))
    }
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    pub fn new(
        task_id: String,
        task_val: AST,
        call: Option<String>,
        params: Option<ParameterTuple>,
        dependencies: Vec<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_id,
            task_val,
            call,
            params,
            dependencies,
            preamble,
            dialect,
            singleton_type: PhantomData,
        }
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<String>, Vec<Import>) {
        let args;
        let kwargs;
        if let Some(ref p) = self.params {
            args = p.get_args();
            kwargs = p.get_kwargs();
        } else {
            args = Vec::new();
            kwargs = LinkedHashMap::new();
        }
        let singleton = T::new(
            AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            self.get_task_val(),
            self.call.clone(),
            args,
            kwargs,
            match self.dependencies.len() {
                0 => None,
                _ => Some(AST::List(List::new_wrapped(
                    self.dependencies.clone(),
                    false,
                ))),
            },
            self.get_preamble(),
            self.get_dialect(),
            endpoints.clone(),
        );
        (
            singleton.get_statements(),
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ForLoopETLTask<T>
where
    T: ETLSingleton,
{
    params_dict_name: AST,
    key: ETLTaskCompressionKey,
    values: Vec<ETLTaskUncompressiblePart<T>>,
    singleton_type: PhantomData<T>,
    task_id: AST,
    insert_task_name: bool,
}
impl<T> ForLoopETLTask<T>
where
    T: ETLSingleton,
{
    pub fn new(
        params_dict_name: AST,
        key: ETLTaskCompressionKey,
        values: Vec<ETLTaskUncompressiblePart<T>>,
        task_id: AST,
        insert_task_name: bool,
    ) -> Self {
        Self {
            params_dict_name,
            key,
            values,
            task_id,
            insert_task_name,
            singleton_type: PhantomData,
        }
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<String>, Vec<Import>) {
        let any_dependencies = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 0)
            .next()
            .is_some();
        let dependencies_as_list = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 1)
            .next()
            .is_some();
        let dict_content = AST::Dict(Dict::new_wrapped(
            self.values
                .iter()
                .map(|x| (x.dict.clone(), x.as_python_dict(dependencies_as_list, self.insert_task_name)))
                .collect(),
        ));
        let dict_assign = AST::Assignment(Assignment::new_wrapped(
            self.params_dict_name.clone(),
            dict_content,
        ));

        let params = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".to_string()));
        let ident = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".to_string()));
        let tpl = AST::Tuple(Tuple::new_wrapped(
            vec![ident.clone(), params.clone()],
            false,
        ));
        //let (dict, call, params_dedup_key, preamble, dialect)
        let key = self.key.clone();
        let new_collector = AST::Subscript(Subscript::new_wrapped(
            key.get_dict_name(),
            ident.clone(),
            false,
        ));
        let mut kwargs;
        let args;
        if let Some((num_args, kwarg_keys)) = key.get_dedup_key() {
            kwargs = kwarg_keys
                .iter()
                .map(|x| {
                    (
                        x.clone(),
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(x.to_string(), false)),
                            false,
                        )),
                    )
                })
                .collect::<LinkedHashMap<_, _>>();

            args = (0..num_args)
                .map(|x| {
                    AST::Subscript(Subscript::new_wrapped(
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(
                                "args".to_string(),
                                false,
                            )),
                            false,
                        )),
                        AST::BigIntLiteral(BigIntLiteral::new_wrapped(x as i64)),
                        false,
                    ))
                })
                .collect::<Vec<AST>>();
        } else {
            kwargs = LinkedHashMap::new();
            args = Vec::new();
        }
        for (k, v) in &key.kwargs {
            kwargs.insert(k.clone(), v.clone());
        }
        let mut dependencies = match any_dependencies {
            true => Some(AST::Subscript(Subscript::new_wrapped(
                params.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped(
                    "dependencies".to_string(),
                    false,
                )),
                false,
            ))),
            false => None,
        };
        let compressed_dependencies = self.key.deps.clone();
        if compressed_dependencies.len() > 0 {
            let left = AST::List(List::new_wrapped(compressed_dependencies, false));
            if let Some(ref right) = dependencies {
                let op = AST::Add(Add::new_wrapped());
                dependencies = Some(AST::BinOp(BinOp::new_wrapped(left, op, right.clone())));
            } else {
                dependencies = Some(left);
            }
        }

        let singleton = T::new(
            self.task_id.clone(),
            new_collector.clone(),
            key.get_call(),
            args,
            kwargs,
            dependencies,
            key.get_preamble(),
            key.get_dialect(),
            endpoints.clone(),
        );
        let statements = singleton.get_statements();
        let items_call = AST::Call(Call::new_wrapped(
            AST::Attribute(Attribute::new_wrapped(
                self.params_dict_name.clone(),
                "items".to_string(),
                false,
            )),
            Vec::new(),
            LinkedHashMap::new(),
        ));
        let for_loop = AST::ForLoop(ForLoop::new_wrapped(
            tpl.clone(),
            items_call,
            statements.clone(),
        ));
        (
            vec![dict_assign, for_loop],
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}

pub enum ETLTask<T>
where
    T: ETLSingleton,
{
    StandaloneETLTask(StandaloneETLTask<T>),
    ForLoopETLTask(ForLoopETLTask<T>),
}
impl<T> ETLTask<T>
where
    T: ETLSingleton,
{
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<String>, Vec<Import>) {
        match &self {
            ETLTask::StandaloneETLTask(x) => x.get_statements(endpoints),
            ETLTask::ForLoopETLTask(x) => x.get_statements(endpoints),
        }
    }
}

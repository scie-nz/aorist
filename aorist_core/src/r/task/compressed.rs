use aorist_primitives::AVec;
use crate::endpoints::EndpointConfig;
use crate::flow::{CompressionKey, ETLFlow, ForLoopCompressedTask, TaskBase, UncompressiblePart};
use crate::r::preamble::RPreamble;
use crate::r::r_import::RImport;
use crate::r::task::key::RBasedTaskCompressionKey;
use crate::r::task::uncompressible::RBasedTaskUncompressiblePart;
use aorist_ast::{
    Add, Assignment, BigIntLiteral, BinOp, Dict, ForLoop, List, SimpleIdentifier, StringLiteral,
    Subscript, AST,
};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ForLoopRBasedTask<T>
where
    T: ETLFlow<ImportType = RImport>,
{
    params_dict_name: AST,
    key: RBasedTaskCompressionKey,
    values: AVec<RBasedTaskUncompressiblePart<T>>,
    singleton_type: PhantomData<T>,
    task_id: AST,
    insert_task_name: bool,
}
impl<T> ForLoopCompressedTask<T> for ForLoopRBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    type KeyType = RBasedTaskCompressionKey;
    type UncompressiblePartType = RBasedTaskUncompressiblePart<T>;
    fn new(
        params_dict_name: AST,
        key: RBasedTaskCompressionKey,
        values: AVec<RBasedTaskUncompressiblePart<T>>,
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
}
impl<T> TaskBase<T> for ForLoopRBasedTask<T> where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>
{
}

impl<T> ForLoopRBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    fn get_dict_assign(&self) -> AST {
        let dependencies_as_list = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 1)
            .next()
            .is_some();
        let dict_content = AST::Dict(Dict::new_wrapped(
            self.values
                .iter()
                .map(|x| {
                    (
                        x.dict.clone(),
                        x.as_dict(dependencies_as_list, self.insert_task_name),
                    )
                })
                .collect(),
        ));
        AST::Assignment(Assignment::new_wrapped(
            self.params_dict_name.clone(),
            dict_content,
        ))
    }
    fn get_for_loop_tuple(&self, _ident: &AST, params: &AST) -> AST {
        params.clone()
    }
    fn get_task_collector(&self, ident: &AST) -> AST {
        AST::Subscript(Subscript::new_wrapped(
            self.key.clone().get_dict_name(),
            ident.clone(),
            false,
        ))
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (AVec<AST>, AVec<RPreamble>, AVec<RImport>) {
        let any_dependencies = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 0)
            .next()
            .is_some();

        let dict_assign = self.get_dict_assign();

        let params = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".into()));
        let ident = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".into()));

        let tpl = self.get_for_loop_tuple(&ident, &params);
        let new_collector = self.get_task_collector(&ident);

        let mut kwargs;
        let args;
        if let Some((num_args, kwarg_keys)) = self.key.get_dedup_key() {
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
                                "args".into(),
                                false,
                            )),
                            false,
                        )),
                        AST::BigIntLiteral(BigIntLiteral::new_wrapped(x as i64)),
                        false,
                    ))
                })
                .collect::<AVec<AST>>();
        } else {
            kwargs = LinkedHashMap::new();
            args = AVec::new();
        }
        for (k, v) in &self.key.kwargs {
            kwargs.insert(k.clone(), v.clone());
        }
        let mut dependencies = match any_dependencies {
            true => Some(AST::Subscript(Subscript::new_wrapped(
                params.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped(
                    "dependencies".into(),
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
            self.key.get_call(),
            args,
            kwargs,
            dependencies,
            self.key.get_preamble(),
            self.key.get_dialect(),
            endpoints.clone(),
        );
        let statements = singleton.get_statements();
        let for_loop = AST::ForLoop(ForLoop::new_wrapped(
            tpl.clone(),
            self.params_dict_name.clone(),
            statements.clone(),
        ));
        (
            vec![dict_assign, for_loop],
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}

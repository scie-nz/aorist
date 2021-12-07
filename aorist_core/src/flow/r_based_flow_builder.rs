use aorist_primitives::AVec;
use crate::flow::flow_builder::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::flow::flow_builder_input::FlowBuilderInput;
use crate::flow::native_r_based_flow::NativeRBasedFlow;
use crate::r::{RFlowBuilderInput, RPreamble};
use aorist_ast::AST;
use extendr_api::prelude::*;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RFlowBuilderError {
    #[error("{0}")]
    Generic(AString),
}

pub struct RBasedFlowBuilder {}
impl RBasedFlowBuilder {
    fn build_flow(
        &self,
        statements: AVec<(AString, Option<AString>, Option<AString>, AVec<AString>)>,
    ) -> AVec<(AString, AVec<AString>)> {
        statements
            .into_iter()
            .map(|(name, title, body, code)| {
                (
                    match title {
                        Some(t) => match body {
                            Some(b) => format!(
                                "## {}\n{}",
                                t,
                                b.split("\n")
                                    .map(|x| format!("# {}", x).to_string())
                                    .collect::<AVec<AString>>()
                                    .join("\n")
                            )
                            .to_string(),
                            None => format!("## {}", t).to_string(),
                        },
                        None => format!("## {}", name).to_string(),
                    },
                    code,
                )
            })
            .collect()
    }

    fn build_file(&self, sources: AVec<(Option<AString>, AString)>) -> String {
        sources
            .into_iter()
            .map(|(maybe_comment, block)| match maybe_comment {
                Some(comment) => format!("# {}\n{}\n", comment, block).to_string(),
                None => block,
            })
            .collect::<AVec<AString>>()
            .join("\n")
    }
}

impl FlowBuilderBase for RBasedFlowBuilder {
    type T = NativeRBasedFlow;
    fn new() -> Self {
        Self {}
    }
}

impl FlowBuilderMaterialize for RBasedFlowBuilder {
    type BuilderInputType = RFlowBuilderInput;
    type ErrorType = RFlowBuilderError;

    fn materialize(
        &self,
        statements_and_preambles: AVec<RFlowBuilderInput>,
    ) -> std::result::Result<AString, RFlowBuilderError> {
        let preambles: LinkedHashSet<RPreamble> = statements_and_preambles
            .iter()
            .map(|x| x.clone().get_preambles().into_iter())
            .flatten()
            .collect();

        let preamble_imports = Self::get_preamble_imports(&preambles);

        let imports = statements_and_preambles
            .iter()
            .map(|x| x.get_imports().clone().into_iter())
            .flatten()
            .chain(preamble_imports)
            .collect::<BTreeSet<_>>();

        let imports_ast: AVec<_> = imports
            .into_iter()
            .map(|x| {
                let call = x.to_r_ast_node(0);
                let deparsed = call!("deparse", call).unwrap();
                Vec::<AString>::from_robj(&deparsed).unwrap().join("\n")
            })
            .collect();

        let statements: AVec<(AString, Option<AString>, Option<AString>, AVec<AST>)> =
            statements_and_preambles
                .into_iter()
                .map(|x| {
                    (
                        x.get_constraint_name(),
                        x.get_constraint_title(),
                        x.get_constraint_body(),
                        x.get_statements(),
                    )
                })
                .collect();
        let mut statements_with_ast: AVec<_> = statements
            .into_iter()
            .filter(|x| x.3.len() > 0)
            .collect::<AVec<_>>();

        // ast_value without ancestry => short_name => keys
        let mut literals: LinkedHashMap<AST, LinkedHashMap<AString, AVec<_>>> = LinkedHashMap::new();

        for (short_name, _, _, asts) in statements_with_ast.iter() {
            for ast in asts {
                Self::extract_literals(ast, &short_name, &mut literals);
            }
        }
        let assignments_ast = Self::literals_to_assignments(literals);

        if assignments_ast.len() > 0 {
            statements_with_ast.insert(
                0,
                (
                    "assignments".into(),
                    Some("Common string literals".into()),
                    None,
                    assignments_ast,
                ),
            );
        }
        let statements_ast = statements_with_ast
            .into_iter()
            .map(|(name, title, body, x)| {
                (
                    name,
                    title,
                    body,
                    x.into_iter()
                        .map(|y| {
                            let deparsed = call!("deparse", y.to_r_ast_node(0)).unwrap();
                            Vec::<AString>::from_robj(&deparsed).unwrap().join("\n")
                        })
                        .collect(),
                )
            })
            .collect();

        let flow = self.build_flow(statements_ast);

        let content: AVec<(Option<AString>, AVec<_>)> =
            vec![(Some("RImports".into()), imports_ast)]
                .into_iter()
                .chain(preambles.into_iter().map(|x| (None, vec![x.get_body()])))
                .chain(flow.into_iter().map(|(x, y)| (Some(x), y)))
                .collect();

        let mut sources: AVec<(Option<AString>, AString)> = AVec::new();

        // This is needed since astor will occasionally forget to add a newline
        for (comment, block) in content {
            let mut lines: AVec<AString> = AVec::new();
            for item in block {
                lines.push(item);
            }
            sources.push((comment, lines.join("\n")))
        }
        Ok(self.build_file(sources))
    }
}

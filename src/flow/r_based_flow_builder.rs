use crate::flow::flow_builder::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::flow::flow_builder_input::FlowBuilderInput;
use crate::flow::native_r_based_flow::NativeRBasedFlow;
use crate::python::AST;
use crate::r::{RFlowBuilderInput, RPreamble};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RFlowBuilderError {
    #[error("{0}")]
    Generic(String),
}


struct RBasedFlowBuilder {}
impl RBasedFlowBuilder {
    fn build_flow(
        &self,
        statements: Vec<(String, Option<String>, Option<String>, Vec<String>)>,
    ) -> Vec<(String, Vec<String>)> {
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
                                    .collect::<Vec<String>>()
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

    fn build_file(
        &self,
        sources: Vec<(Option<String>, String)>,
    ) -> String {
        sources
            .into_iter()
            .map(|(maybe_comment, block)| match maybe_comment {
                Some(comment) => format!("# {}\n{}\n", comment, block).to_string(),
                None => block,
            })
            .collect::<Vec<String>>()
            .join("")
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
        statements_and_preambles: Vec<RFlowBuilderInput>,
    ) -> std::result::Result<String, RFlowBuilderError> {
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

        let imports_ast: Vec<_> = imports.into_iter().map(|x| x.to_r_ast_node(0).as_str().unwrap().to_string()).collect();

        let statements: Vec<(String, Option<String>, Option<String>, Vec<AST>)> =
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
        let mut statements_with_ast: Vec<_> = statements
            .into_iter()
            .filter(|x| x.3.len() > 0)
            .collect::<Vec<_>>();

        // ast_value without ancestry => short_name => keys
        let mut literals: LinkedHashMap<AST, LinkedHashMap<String, Vec<_>>> = LinkedHashMap::new();

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
                    "assignments".to_string(),
                    Some("Common string literals".to_string()),
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
                        .map(|y| y.to_r_ast_node(0).as_str().unwrap().to_string())
                        .collect(),
                )
            })
            .collect();

        let flow = self.build_flow(statements_ast);

        let content: Vec<(Option<String>, Vec<_>)> =
            vec![(Some("RImports".to_string()), imports_ast)]
                .into_iter()
                .chain(preambles.into_iter().map(|x| (None, vec![x.get_body()])))
                .chain(flow.into_iter().map(|(x, y)| (Some(x), y)))
                .collect();

        let mut sources: Vec<(Option<String>, String)> = Vec::new();

        // This is needed since astor will occasionally forget to add a newline
        for (comment, block) in content {
            let mut lines: Vec<String> = Vec::new();
            for item in block {
                lines.push(item);
            }
            sources.push((comment, lines.join("")))
        }
        Ok(self.build_file(sources))
    }
}
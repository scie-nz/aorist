use crate::attributes::{Attribute, InnerAttribute, InnerPredicate};
use crate::template::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use sqlparser::ast::{Query, Select, SelectItem, SetExpr, TableFactor};
use std::collections::HashMap;
create_exception!(aorist, SQLParseError, PyException);

pub type AttrMap = HashMap<String, HashMap<String, HashMap<String, Attribute>>>;

pub struct SQLParser {
    attr_map: AttrMap,
    template_name: String,
}
impl SQLParser {
    pub fn new(attr_map: AttrMap, template_name: String) -> Self {
        Self {
            attr_map,
            template_name,
        }
    }
    pub fn parse_select(&self, select: Select) -> PyResult<InnerFilter> {
        if select.distinct {
            return Err(SQLParseError::new_err("DISTINCT not supported."));
        }
        if select.having.is_some() {
            return Err(SQLParseError::new_err("HAVING not supported."));
        }
        if select.from.len() != 1 {
            return Err(SQLParseError::new_err(
                "Exactly 1 table must be in the FROM clause.",
            ));
        }
        if select.lateral_views.len() > 0 {
            return Err(SQLParseError::new_err("LATERAL VIEWs not supported."));
        }
        if select.group_by.len() > 0 {
            return Err(SQLParseError::new_err("GROUP BYs not supported."));
        }
        if select.cluster_by.len() > 0 {
            return Err(SQLParseError::new_err("CLUSTER BYs not supported."));
        }
        if select.distribute_by.len() > 0 {
            return Err(SQLParseError::new_err("DISTRIBUTE BYs not supported."));
        }
        if select.sort_by.len() > 0 {
            return Err(SQLParseError::new_err("SORT BYs not supported."));
        }
        if select.projection.len() != 1 {
            return Err(SQLParseError::new_err("Only SELECT * supported."));
        }
        if select.projection.into_iter().next().unwrap() != SelectItem::Wildcard {
            return Err(SQLParseError::new_err("Only SELECT * supported."));
        }
        if select.selection.is_none() {
            return Err(SQLParseError::new_err("A WHERE clause must be provided."));
        } else {
            let predicate =
                InnerPredicate::try_from(select.selection.unwrap(), &self.attr_map).unwrap();
            let source_table = select.from.into_iter().next().unwrap();
            if source_table.joins.len() > 0 {
                return Err(SQLParseError::new_err("JOINs not supported."));
            }
            if let TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } = source_table.relation
            {
                if alias.is_some() {
                    return Err(SQLParseError::new_err("Table aliases not supported."));
                }
                if args.len() > 0 {
                    return Err(SQLParseError::new_err("Table args not supported."));
                }
                if with_hints.len() > 0 {
                    return Err(SQLParseError::new_err("Table WITH hints not supported."));
                }
                if name.0.len() != 2 {
                    return Err(SQLParseError::new_err(
                        "Exactly 2 identifiers must be in each asset name.".to_string(),
                    ));
                }
                let source_asset_name = name.0.get(1).unwrap().clone().value;
                let dataset_name = name.0.get(0).unwrap().clone().value;
                if let Some(dataset_attr) = self.attr_map.get(&dataset_name) {
                    if let Some(asset_attr) = dataset_attr.get(&source_asset_name) {
                        let attributes = asset_attr
                            .values()
                            .map(|x| InnerAttribute::from(x.clone()))
                            .collect();
                        return Ok(InnerFilter::new(
                            attributes,
                            Some(predicate),
                            self.template_name.clone(),
                            source_asset_name,
                            None,
                        ));
                    } else {
                        return Err(SQLParseError::new_err(
                            format!(
                                "Cannot find asset name {} in dataset {}.",
                                source_asset_name, dataset_name
                            )
                            .to_string(),
                        ));
                    }
                } else {
                    return Err(SQLParseError::new_err(
                        format!("Cannot find dataset name {}.", dataset_name).to_string(),
                    ));
                }
            } else {
                return Err(SQLParseError::new_err("Subqueries not supported."));
            }
        }
    }
    pub fn parse_query(&self, query: Query) -> PyResult<InnerFilter> {
        if query.with.is_some() {
            return Err(SQLParseError::new_err("WITH clauses are not supported."));
        }
        if query.order_by.len() > 0 {
            return Err(SQLParseError::new_err("ORDER BY not supported."));
        }
        if query.limit.is_some() {
            return Err(SQLParseError::new_err("LIMIT not supported."));
        }
        if query.offset.is_some() {
            return Err(SQLParseError::new_err("OFFSET not supported."));
        }
        if query.fetch.is_some() {
            return Err(SQLParseError::new_err("FETCH not supported."));
        }
        if let SetExpr::Select(select) = query.body {
            return self.parse_select(*select);
        } else {
            return Err(SQLParseError::new_err(
                "A single SELECT statement should be provided.",
            ));
        }
    }
}

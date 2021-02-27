use crate::attributes::Attribute;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use sqlparser::ast::{Query, Select, SetExpr};
use std::collections::HashMap;
create_exception!(aorist, SQLParseError, PyException);

type AttrMap = HashMap<String, HashMap<String, HashMap<String, Attribute>>>;

pub struct SQLParser {
    attr_map: AttrMap,
}
impl SQLParser {
    pub fn new(attr_map: AttrMap) -> Self {
        Self { attr_map }
    }
    pub fn parse_select(&self, select: Select) -> PyResult<()> {
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
        if select.selection.is_none() {
            return Err(SQLParseError::new_err("A WHERE clause must be provided."));
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

        Ok(())
    }
    pub fn parse_query(&self, query: Query) -> PyResult<()> {
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

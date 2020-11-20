use sqlparser::ast::{
    Ident, ObjectName, Query, Select, SelectItem, SetExpr, TableFactor, TableWithJoins,
};

pub struct PrestoInsertQuery {
    query: Query,
}
impl PrestoInsertQuery {
    pub fn empty() -> Self {
        let projection = vec![SelectItem::Wildcard];
        let table = vec![TableWithJoins {
            relation: TableFactor::Table {
                // e.g.: vec![Ident::new("some_table")]
                name: ObjectName(Vec::new()),
                alias: None,
                args: Vec::new(),
                with_hints: Vec::new(),
            },
            joins: Vec::new(),
        }];
        let select = Select {
            distinct: false,
            top: None,
            projection: projection,
            from: table,
            selection: None,
            group_by: Vec::new(),
            having: None,
        };
        let query = Query {
            ctes: Vec::new(),
            body: SetExpr::Select(Box::new(select)),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            fetch: None,
        };
        Self { query }
    }
}

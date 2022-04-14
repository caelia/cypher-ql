pub mod parser;
pub mod processor;
mod types;

pub trait QueryProcessor<TConn> {
    fn execute(conn: TConn, query: types::Query) -> types::QueryResult;
}

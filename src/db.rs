pub use diesel::r2d2::ConnectionManager as DbConnectionManager;
pub use diesel::r2d2::ManageConnection as ManageDbConnection;
pub use diesel::r2d2::Pool as DbPool;
pub use diesel::Connection as DbConnection;
pub use diesel::PgConnection;

pub type PgPool = DbPool<DbConnectionManager<PgConnection>>;

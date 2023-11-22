use super::schema::casbin_rule;

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(not(feature = "uuid"))]
#[derive(Queryable, Identifiable)]
#[diesel(table_name = casbin_rule)]
pub(crate) struct CasbinRule {
    pub id: i32,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[cfg(all(feature = "uuid", feature = "mysql"))]
#[derive(Queryable, Identifiable)]
#[diesel(table_name = casbin_rule)]
pub(crate) struct CasbinRule {
    pub id: Vec<u8>,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[cfg(all(feature = "uuid", feature = "postgres"))]
#[derive(Queryable, Identifiable)]
#[diesel(table_name = casbin_rule)]
pub(crate) struct CasbinRule {
    pub id: Uuid,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = casbin_rule)]
pub(crate) struct NewCasbinRule {
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

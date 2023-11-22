#[cfg(any(feature = "sqlite", all(feature = "mysql", not(feature = "uuid"))))]
table! {
    casbin_rule (id) {
        id -> Integer,
        ptype -> Varchar,
        v0 -> Varchar,
        v1 -> Varchar,
        v2 -> Varchar,
        v3 -> Varchar,
        v4 -> Varchar,
        v5 -> Varchar,
    }
}

#[cfg(all(feature = "uuid", feature = "mysql"))]
table! {
    casbin_rule (id) {
        id -> Binary,
        ptype -> Varchar,
        v0 -> Varchar,
        v1 -> Varchar,
        v2 -> Varchar,
        v3 -> Varchar,
        v4 -> Varchar,
        v5 -> Varchar,
    }
}

#[cfg(all(feature = "uuid", feature = "postgres"))]
table! {
    casbin_rule (id) {
        id -> Uuid,
        ptype -> Text,
        v0 -> Text,
        v1 -> Text,
        v2 -> Text,
        v3 -> Text,
        v4 -> Text,
        v5 -> Text,
    }
}

#[cfg(all(not(feature = "uuid"), feature = "postgres"))]
table! {
    casbin_rule (id) {
        id -> Integer,
        ptype -> Text,
        v0 -> Text,
        v1 -> Text,
        v2 -> Text,
        v3 -> Text,
        v4 -> Text,
        v5 -> Text,
    }
}

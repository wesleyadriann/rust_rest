diesel::table! {
    contacts() {
        id -> Varchar,
        password -> Varchar,
        fullname -> Varchar,
        birthdate -> Varchar,
    }
}

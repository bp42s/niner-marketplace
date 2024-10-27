mod common;

struct Account {
    username: String,
    password: String,
    bio: String,
    rep: i32,
    birthday: common::Date,
    account_birthday: common::Date,
}

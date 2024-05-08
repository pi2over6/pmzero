use super::db;
use std::error::Error;

pub use db::Members;

pub fn members() -> Result<Members, Box<dyn Error>> {
    db::load_members()
}

pub fn append_member(new_member: &str) -> Result<(), Box<dyn Error>> {
    let mut members = db::load_members()?;

    members.insert(members.keys().max().unwrap() + 1, String::from(new_member));

    db::save_members(&members)?;
    Ok(())
}

pub fn get_member_id(name: &String) -> Result<usize, Box<dyn Error>> {
    let members = db::load_members()?;

    for (user_id, user_name) in members {
        if &user_name == name {
            return Ok(user_id);
        }
    }

    Err("player should be registered first")?
}
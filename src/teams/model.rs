use crate::db;
use crate::error_handler::CustomError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::teams;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

impl Team {
    pub fn find(team_id: i32) -> Result<Self, CustomError> {
        use crate::schema::teams::dsl::*;

        let conn = &mut db::connection()?;

        let team = teams.filter(id.eq(team_id)).first::<Team>(conn)?;

        Ok(team)
    }

    pub fn create(team: NewTeam) -> Result<Self, CustomError> {
        use crate::schema::teams::dsl::*;

        let conn = &mut db::connection()?;

        let team = NewTeam::from(team);
        log::info!("Before team insertion");

        let team = diesel::insert_into(teams)
            .values(&team)
            .get_result(conn)
            .expect("Error saving new team");

        Ok(team)
    }
}

impl NewTeam {
    pub fn from(team: NewTeam) -> NewTeam {
        NewTeam { name: team.name }
    }
}

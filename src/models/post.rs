use diesel::*; 

use crate::models::schema::posts::dsl::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn insert(conn: &MysqlConnection) -> QueryResult<usize> {

        insert_into(posts).values((
            title.eq("test insert"),
            body.eq("hhhhhhhhhh"),
            published.eq(false)
        )).execute(conn)
    }

    pub fn find_by_id(conn: &MysqlConnection, _id:i32) -> QueryResult<Post> {

        let result = posts.find(_id).first(conn);
        result
    }

    pub fn find_by(conn: &MysqlConnection, _id:i32) -> QueryResult<Post> {

        let result = posts.find(_id).first(conn);
        result
    }
}
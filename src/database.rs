use sqlx::{postgres::PgQueryResult, Error, Pool, Postgres};
use uuid::Uuid;

use crate::model::{PersonCountModel, StoredPersonModel};

type DatabaseError = Error;
type DatabaseQueryResult = PgQueryResult;

type DatabaseResult<T> = Result<T, DatabaseError>;

pub struct Database {
    database_pool: Pool<Postgres>,
}

impl Database {
    pub fn new(database_pool: Pool<Postgres>) -> Self {
        return Database { database_pool };
    }

    pub async fn count_persons(&self) -> DatabaseResult<PersonCountModel> {
        let query = "SELECT COUNT(*) AS count FROM persons";

        sqlx::query_as::<_, PersonCountModel>(query)
            .fetch_one(&self.database_pool)
            .await
    }

    pub async fn insert_person(
        &self,
        id: Uuid,
        nickname: String,
        name: String,
        birth: String,
        stack: Vec<&str>,
    ) -> DatabaseResult<DatabaseQueryResult> {
        let query =
            "INSERT INTO persons (id, nickname, name, birth, stack) VALUES ($1, $2, $3, $4, $5)";

        sqlx::query(query)
            .bind(id)
            .bind(nickname)
            .bind(name)
            .bind(birth)
            .bind(stack)
            .execute(&self.database_pool)
            .await
    }

    pub async fn select_person_by_term(
        &self,
        term: String,
    ) -> DatabaseResult<Vec<StoredPersonModel>> {
        let query = "SELECT p.id, p.nickname, p.name, p.birth, p.stack FROM persons p WHERE p.search_terms ILIKE $1 LIMIT 50";

        sqlx::query_as::<_, StoredPersonModel>(query)
            .bind(format!("%{term}%"))
            .fetch_all(&self.database_pool)
            .await
    }

    pub async fn select_person_by_id(&self, id: Uuid) -> DatabaseResult<StoredPersonModel> {
        let query =
            "SELECT p.id, p.nickname, p.name, p.birth, p.stack FROM persons p WHERE p.id = $1";

        sqlx::query_as::<_, StoredPersonModel>(query)
            .bind(id)
            .fetch_one(&self.database_pool)
            .await
    }
}

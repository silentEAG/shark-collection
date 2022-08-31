use crate::types::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub num: Option<isize>,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name, num: None }
    }

    /// Upsert tag table and return `id`
    pub async fn upsert(&self, pool: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<isize> {
        let res = sqlx::query!(
            r#"
                INSERT INTO sc_tag(name, num)
                VALUES($1, 1)
                ON conflict(name)
                DO UPDATE SET num = sc_tag.num + 1 RETURNING id
            "#,
            self.name
        )
        .fetch_one(&mut *pool)
        .await?;
        Ok(res.id as isize)
    }
}

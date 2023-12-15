use chrono::{DateTime, Local};
use eyre::Result;
use sqlx::{Pool, Postgres, Row};
use url::Url;

pub type Database = Pool<Postgres>;

pub async fn link_count(dbc: &Database, search_text: Option<&str>) -> Result<u32> {
    let (where_clause, params) = search_where(search_text, 1);
    let statement = format!("SELECT COUNT(*) FROM links {}", where_clause);
    let mut query = sqlx::query(&statement);
    for param in params {
        query = query.bind(param);
    }
    Ok(query
        .fetch_one(dbc)
        .await?
        .try_get::<i64, _>(0)?
        .try_into()?)
}

#[derive(sqlx::FromRow)]
pub struct SearchLinksRow {
    pub id: i32,
    pub href: String,
    pub description: String,
    pub extended: String,
    pub time: DateTime<Local>,
    pub shared: bool,
    pub to_read: bool,
    pub tags: Vec<String>,
}

pub async fn search_links(
    dbc: &Database,
    search_text: Option<&str>,
    page: u32,
    page_size: u32,
) -> Result<Vec<SearchLinksRow>> {
    let (where_clause, params) = search_where(search_text, 3);
    let statement = format!(
        r#"
SELECT
    id,
    href,
    description,
    extended,
    time,
    shared,
    toRead AS to_read,
    tags
FROM links {where_clause}
ORDER BY time DESC
OFFSET $1 LIMIT $2
"#
    );
    let mut query = sqlx::query_as::<_, SearchLinksRow>(&statement)
        .bind(((page - 1) * page_size) as i64)
        .bind(page_size as i64);
    for param in params {
        query = query.bind(param);
    }
    Ok(query.fetch_all(dbc).await?)
}

fn search_where(search_text: Option<&str>, first_param_index: i32) -> (String, Vec<String>) {
    let Some(text) = search_text else {
        return ("".to_string(), Vec::new());
    };
    if text.is_empty() {
        return ("".to_string(), Vec::new());
    }
    let search_terms = text.split_whitespace();
    let where_clause = format!("WHERE {}", search_terms.clone().enumerate().map(|(index, _)| {
        let p = index as i32 * 2 + first_param_index;
        format!("(href LIKE ${p} OR description LIKE ${p} OR extended LIKE ${p} OR (SELECT bool_or(t LIKE ${p1}) FROM unnest(tags) t))", p = p, p1 = p + 1)
    }).collect::<Vec<String>>().join(" AND "));
    let params = search_terms.flat_map(|term| [format!("%{}%", term), format!("{}%", term)]);
    (where_clause, params.collect())
}

pub struct CreateLink<'a> {
    pub href: Url,
    pub description: &'a str,
    pub extended: &'a str,
    pub tags: Vec<&'a str>,
}

/// true -> created, false -> already exists
pub async fn create_link(dbc: &Database, record: CreateLink<'_>) -> Result<bool> {
    let result = sqlx::query(
        r#"
INSERT INTO links (href, description, extended, time, shared, toRead, tags)
VALUES ($1, $2, $3, current_timestamp, false, false, $4)
"#,
    )
    .bind(record.href.as_str())
    .bind(record.description)
    .bind(record.extended)
    .bind(record.tags)
    .execute(dbc)
    .await;
    if let Err(sqlx::Error::Database(err)) = result {
        if err.is_unique_violation() {
            return Ok(false);
        }
    }
    Ok(true)
}

pub async fn delete_link(dbc: &Database, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM links WHERE id = $1")
        .bind(id)
        .execute(dbc)
        .await?;
    Ok(())
}

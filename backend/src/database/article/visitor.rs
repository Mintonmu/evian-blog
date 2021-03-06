use super::Database;

use super::models::*;
use super::super::neo4j_ops;
use neo4j_ops::Neo4jStatement;

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

impl Database {
    // Return all tags of type `TagMeta` with order of their last_revise_date
    pub async fn get_all_tags(&self) -> Result<Vec<TagMeta>> {
        let query_str = "\
MATCH (tag:Tag)<-[:HAS_TAG]-(article:Article)
RETURN {name: tag.name, last_revise_date: max(article.last_revise_date), article_count: count(article)} AS tag_meta
ORDER BY tag_meta.last_revise_date DESC, tag_meta.name ASC";
        let query_statment = Neo4jStatement {
            statement: query_str,
            parameters: None
        };
        Ok(neo4j_ops::query::<TagMeta>(&self.url, &self.client, &self.authorization, query_statment)
            .await?)
    }

    // Return all series of type `SeriesMeta` with order of their last_revise_date
    pub async fn get_all_series(&self) -> Result<Vec<SeriesMeta>> {
        let query_str = "\
MATCH (series:Series)<-[:IN_SERIES]-(article:Article)
RETURN {name: series.name, last_revise_date: max(article.last_revise_date), article_count: count(article)} AS tag_meta
ORDER BY tag_meta.last_revise_date DESC, tag_meta.name ASC";
        let query_statment = Neo4jStatement {
            statement: query_str,
            parameters: None
        };
        Ok(neo4j_ops::query::<SeriesMeta>(&self.url, &self.client, &self.authorization, query_statment)
            .await?)
    }

    // Return count of all articles
    pub async fn get_all_articles_count(&self) -> Result<usize> {
        let total_count_str = "\
MATCH (article:Article)
RETURN count(article)";
        let total_count_statement = Neo4jStatement {
            statement: total_count_str,
            parameters: None,
        };
        let total_count = neo4j_ops::query::<usize>(&self.url, &self.client, &self.authorization, total_count_statement)
            .await?
            .pop().ok_or(Error::Unexpected)?;

        Ok(total_count)
    }

    // Return titles of all articles
    pub async fn get_all_article_titles(&self) -> Result<Vec<String>> {
        let query_str = "\
MATCH (article:Article)
RETURN article.title
";
        let query_statement = Neo4jStatement {
            statement: query_str,
            parameters: None,
        };
        let article_titles = neo4j_ops::query::<String>(&self.url, &self.client, &self.authorization, query_statement)
            .await?;

        Ok(article_titles)
    }

    // Return all articles with order of their last_revise_date descending
    pub async fn get_all_articles(
        &self,
        page_index: usize,
        page_size: usize
    ) -> Result<Vec<ArticleMeta>> {
        let pagination_str = "\
MATCH (article:Article)
MATCH (article)-[:HAS_TAG]->(tag:Tag)
OPTIONAL MATCH (article)-[in_series:IN_SERIES]->(series:Series)
WITH article, tag, in_series, series
ORDER BY tag.name ASC
RETURN {title: article.title, publish_date: article.publish_date, last_revise_date: article.last_revise_date, tags: collect(tag.name), series: series.name, series_index: in_series.index} AS article_meta
ORDER BY article_meta.last_revise_date DESC, article_meta.title ASC
SKIP $skip
LIMIT $limit";
        let pagination_statement = Neo4jStatement {
            statement: pagination_str,
            parameters: Some(hashmap!{
                "skip" => serde_json::Value::from(page_index * page_size),
                "limit" => serde_json::Value::from(page_size)
            })
        };
        let article_metas = neo4j_ops::query::<ArticleMeta>(&self.url, &self.client, &self.authorization, pagination_statement)
            .await?;
        Ok(article_metas)
    }

    // Return count of all articles has tag `tag_name`
    pub async fn get_all_articles_count_of_tag(&self, tag_name: &String) -> Result<usize> {
        let total_count_str = "\
MATCH (:Tag {name: $tag_name})<-[:HAS_TAG]-(article:Article)
RETURN count(article)";
        let total_count_statement = Neo4jStatement {
            statement: total_count_str,
            parameters: Some(hashmap!{"tag_name" => serde_json::Value::from(tag_name.as_str())}),
        };
        let total_count = neo4j_ops::query::<usize>(&self.url, &self.client, &self.authorization, total_count_statement)
            .await?
            .pop().ok_or(Error::Unexpected)?;

        Ok(total_count)
    }

    // Return all articles has tag `tag_name` with order of their last_revise_date descending
    pub async fn get_all_articles_of_tag(
        &self,
        tag_name: &String,
        page_index: usize,
        page_size: usize
    ) -> Result<Vec<ArticleMeta>> {
        let pagination_str = "\
MATCH (:Tag {name: $tag_name})<-[:HAS_TAG]-(article:Article)
MATCH (article)-[:HAS_TAG]->(tag:Tag)
OPTIONAL MATCH (article)-[in_series:IN_SERIES]->(series:Series)
WITH article, tag, in_series, series
ORDER BY tag.name ASC
RETURN {title: article.title, publish_date: article.publish_date, last_revise_date: article.last_revise_date, tags: collect(tag.name), series: series.name, series_index: in_series.index} AS article_meta
ORDER BY article_meta.last_revise_date DESC, article_meta.title ASC
SKIP $skip
LIMIT $limit";
        let pagination_statement = Neo4jStatement {
            statement: pagination_str,
            parameters: Some(hashmap!{
                "tag_name" => serde_json::Value::from(tag_name.as_str()),
                "skip" => serde_json::Value::from(page_index * page_size),
                "limit" => serde_json::Value::from(page_size)
            })
        };
        let article_metas = neo4j_ops::query::<ArticleMeta>(&self.url, &self.client, &self.authorization, pagination_statement)
            .await?;
        Ok(article_metas)
    }

    // Return count of all articles in series `series_name`
    pub async fn get_all_articles_count_of_series(&self, series_name: &String) -> Result<usize> {
        let total_count_str = "\
MATCH (:Series {name: $series_name})<-[:IN_SERIES]-(article:Article)
RETURN count(article)";
        let total_count_statement = Neo4jStatement {
            statement: total_count_str,
            parameters: Some(hashmap!{"series_name" => serde_json::Value::from(series_name.as_str())}),
        };
        let total_count = neo4j_ops::query::<usize>(&self.url, &self.client, &self.authorization, total_count_statement)
            .await?
            .pop().ok_or(Error::Unexpected)?;
        
        Ok(total_count)
    }

    // Return all articles in series `series_name` with order of their series_index ascending
    pub async fn get_all_articles_of_series(
        &self,
        series_name: &String,
        page_index: usize,
        page_size: usize
    ) -> Result<Vec<ArticleMeta>> {
        let pagination_str = "\
MATCH (:Series {name: $series_name})<-[in_series:IN_SERIES]-(article:Article)
MATCH (article)-[:HAS_TAG]->(tag:Tag)
WITH article, tag, in_series
ORDER BY tag.name ASC
RETURN {title: article.title, publish_date: article.publish_date, last_revise_date: article.last_revise_date, tags: collect(tag.name), series: $series_name, series_index: in_series.index} AS article_meta
ORDER BY article_meta.series_index ASC
SKIP $skip
LIMIT $limit";
        let pagination_statement = Neo4jStatement {
            statement: pagination_str,
            parameters: Some(hashmap!{
                "series_name" => serde_json::Value::from(series_name.as_str()),
                "skip" => serde_json::Value::from(page_index * page_size),
                "limit" => serde_json::Value::from(page_size)
            })
        };
        let article_metas = neo4j_ops::query::<ArticleMeta>(&self.url, &self.client, &self.authorization, pagination_statement)
            .await?;
        Ok(article_metas)
    }

    // Return article in title `article_title`. If no such article, return Ok(None)
    pub async fn get_article(&self, article_title: &String) -> Result<Option<Article>> {
        let query_str = "\
MATCH (article:Article {title: $article_title})
MATCH (article)-[:HAS_TAG]->(tag:Tag)
OPTIONAL MATCH (article)-[in_series:IN_SERIES]->(series:Series)
WITH article, tag, in_series, series
ORDER BY tag.name ASC
RETURN {title: article.title, body: article.body, publish_date: article.publish_date, last_revise_date: article.last_revise_date, tags: collect(tag.name), series: series.name, series_index: in_series.index};";
        let query_statment = Neo4jStatement {
            statement: query_str,
            parameters: Some(hashmap!{
                "article_title" => serde_json::Value::from(article_title.as_str())
            })
        };
        Ok(neo4j_ops::query::<Article>(&self.url, &self.client, &self.authorization, query_statment)
            .await?
            .pop())
    }
}

#[derive(Debug)]
pub enum Error {
    Database(neo4j_ops::Error),
    Unexpected,
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let message = match &self {
            Database(database_error) => format!("{}", database_error),
            Unexpected => String::from("unexpected error."),
        };

        write!(f, "{}", message)
    }
}

impl From<neo4j_ops::Error> for Error {
    fn from(database_error: neo4j_ops::Error) -> Self {
        Self::Database(database_error)
    }
}
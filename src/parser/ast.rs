use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clause {
    pub id: Option<i64>,
    pub doc_id: Option<i64>,
    pub doc_title: String,
    pub chapter: String,
    pub section: String,
    pub article: String,
    pub breadcrumb: String,
    pub page_num: Option<i32>,
    pub content: String,
    pub table_data: Option<String>,
    pub status: String, // "effective", "trial", "draft", "superseded"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<ClauseReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClauseReference {
    pub ref_type: String,     // "external_doc", "internal_clause", "pharmacopoeia", "guideline"
    pub target_title: String, // e.g. "中国药典", "问题 6.(2)", "ICH Q5D"
    pub raw_mention: String,  // e.g. "《中国药典》现行版", "见问题 6、（2）"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: Option<i64>,
    pub title: String,
    pub path: String,
    pub hash: String,
    pub status: String,
    pub clause_count: usize,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub clause: Clause,
    pub score: f64,
    pub match_strategy: String,
    pub semantic_score: Option<f64>,
    pub fts_score: Option<f64>,
    pub rerank_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hop_references: Vec<HopReferenceTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HopReferenceTarget {
    pub reference: ClauseReference,
    pub matched_clause: Option<Clause>,
}




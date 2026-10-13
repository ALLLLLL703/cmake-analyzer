// AI-authored per-file incremental pipeline.
use super::model::{DocumentAnalysis, DocumentWork};
use crate::parser::{
    build_struct::model::{AnalysisKey, IncrementalBuildStruct},
    lexer::model::IncrementalLexer,
    nparser::model::IncrementalNParser,
    semanticer::model::{FileId, IncrementalSemanticer},
};
use std::sync::Arc;

impl DocumentAnalysis {
    pub(super) async fn new(text: Arc<str>, key: AnalysisKey) -> Self {
        let lexer = IncrementalLexer::new(text.clone());
        let parser = IncrementalNParser::new(&text, &lexer.buffer);
        let semantics =
            IncrementalSemanticer::new(text, parser.buffer.clone(), FileId { id: key.file }).await;
        let build = IncrementalBuildStruct::new(&semantics.buffer, key);
        Self {
            lexer,
            parser,
            semantics,
            build,
        }
    }
    pub(super) async fn update(&mut self, text: Arc<str>, key: AnalysisKey) {
        let change = self.lexer.update(text.clone());
        self.parser.update(&text, &self.lexer.buffer, &change);
        self.semantics
            .update(text, self.parser.buffer.clone(), &change)
            .await;
        self.build.update(&self.semantics.buffer, key, &change);
    }
    pub(super) fn work(&self) -> DocumentWork {
        DocumentWork {
            lexer: self.lexer.work.clone(),
            parser: self.parser.work.clone(),
            semantics: self.semantics.work.clone(),
            build: self.build.work.clone(),
        }
    }
}

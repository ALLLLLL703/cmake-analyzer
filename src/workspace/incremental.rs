//! Connect per-file lexer, parser, semantic and build caches in dependency order.
//! Updates replace shared buffers, preserving previously retained snapshots; semantic version
//! discovery may inspect the host CMake version but does not execute project code.
// Contains AI-generated documentation additions.
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
    /// Initialize all four per-file caches in dependency order. Semantic setup may inspect host
    /// CMake version; no project code runs.
    ///
    /// # Arguments
    /// - `text` `[in]`: Shared source text retained by incremental analysis.
    /// - `key` `[in]`: Stable file ID plus current revision for semantic/build locations.
    ///
    /// # Returns
    /// Fresh analysis state with lexer, parser, semantics and build results.
    ///
    /// # Examples
    /// ```text
    /// let analysis = DocumentAnalysis::new(text, key).await;
    /// ```
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
    /// Update all per-file caches using the lexer's unchanged-range mapping. Replaces output Arcs,
    /// leaving old snapshots intact; semantic version inspection may occur, not project execution.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Caches updated in lexer/parser/semantic/build order.
    /// - `text` `[in]`: Newest owned shared source text.
    /// - `key` `[in]`: Current file/revision identity for rebuilt sources.
    ///
    /// # Returns
    /// () after all caches refer to the new source.
    ///
    /// # Examples
    /// ```text
    /// analysis.update(new_text, new_key).await;
    /// ```
    pub(super) async fn update(&mut self, text: Arc<str>, key: AnalysisKey) {
        let change = self.lexer.update(text.clone());
        self.parser.update(&text, &self.lexer.buffer, &change);
        self.semantics
            .update(text, self.parser.buffer.clone(), &change)
            .await;
        self.build.update(&self.semantics.buffer, key, &change);
    }
    /// Copy last-operation counters without changing caches or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Pipeline state whose four counters are borrowed.
    ///
    /// # Returns
    /// DocumentWork containing each layer's recomputed/reused counts.
    ///
    /// # Examples
    /// ```text
    /// let work = analysis.work();
    /// ```
    pub(super) fn work(&self) -> DocumentWork {
        DocumentWork {
            lexer: self.lexer.work.clone(),
            parser: self.parser.work.clone(),
            semantics: self.semantics.work.clone(),
            build: self.build.work.clone(),
        }
    }
}

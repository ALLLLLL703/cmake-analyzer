//! Adapt reference requests, retaining the client declaration-inclusion policy.
//! Only returned protocol data changes; the request does not execute project code.

use super::model::Query;
use crate::server::model::CmakeAnalyzer;
use lspally::server::model::*;
impl CmakeAnalyzer {
    /// Return bound occurrences using the client's declaration-inclusion policy.
    /// Queues service work; does not run CMake or apply edits.
    ///
    /// # Arguments
    /// - `self` `[in]`: Analyzer service handle.
    /// - `context` `[in]`: Request context and cancellation.
    /// - `params` `[in]`: Document position and include_declaration flag; progress tokens are unused.
    ///
    /// # Returns
    /// `Ok(Some(locations))`, including an empty vector when no bindings match; this implementation does not return `Ok(None)`.
    ///
    /// # Errors
    /// `Err` forwards cancellation, URI/coordinate validation, service/channel or joined-worker failures.
    ///
    /// # Examples
    /// ```text
    /// let response = analyzer.query_references(context, params).await?;
    /// ```
    pub(in crate::server) async fn query_references(
        &self,
        context: Context,
        params: ReferenceParams,
    ) -> ServerResult<Option<Vec<Location>>> {
        self.navigate(
            context,
            params.text_document_position,
            Query::References {
                include_declaration: params.context.include_declaration,
            },
        )
        .await
        .map(Some)
    }
}

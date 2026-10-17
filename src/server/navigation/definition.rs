//! Adapt definition requests to shared cancellable navigation.
//! Only returned protocol data changes; the request does not execute project code.

use super::model::Query;
use crate::server::model::CmakeAnalyzer;
use lspally::server::model::*;
impl CmakeAnalyzer {
    /// Return source definitions or dependency file-start links for a definition request.
    /// Queues service work; does not run CMake or apply edits.
    ///
    /// # Arguments
    /// - `self` `[in]`: Analyzer service handle.
    /// - `context` `[in]`: Request context and cancellation.
    /// - `params` `[in]`: Document position; definition-specific progress tokens are unused.
    ///
    /// # Returns
    /// `Ok(Some(Array(locations)))` for nonempty results; `Ok(None)` if no definitions are available.
    ///
    /// # Errors
    /// `Err` forwards cancellation, URI/coordinate validation, service/channel or joined-worker failures.
    ///
    /// # Examples
    /// ```text
    /// let response = analyzer.query_definition(context, params).await?;
    /// ```
    pub(in crate::server) async fn query_definition(
        &self,
        context: Context,
        params: GotoDefinitionParams,
    ) -> ServerResult<Option<GotoDefinitionResponse>> {
        let locations = self
            .navigate(
                context,
                params.text_document_position_params,
                Query::Definition,
            )
            .await?;
        Ok((!locations.is_empty()).then_some(GotoDefinitionResponse::Array(locations)))
    }
}

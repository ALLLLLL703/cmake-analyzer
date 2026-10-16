use super::model::Query;
use crate::server::model::CmakeAnalyzer;
use lspally::server::model::*;
impl CmakeAnalyzer {
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

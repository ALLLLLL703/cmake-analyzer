use super::model::Query;
use crate::server::model::CmakeAnalyzer;
use lspally::server::model::*;
impl CmakeAnalyzer {
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

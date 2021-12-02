use crate::models::{ProposalFromDb, SearchProposalsRequest, SearchVotesRequest, VoteFromDb};
use crate::services::Services;

impl Services {
    pub async fn search_proposals(
        &self,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<ProposalFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_proposals(input).await
    }

    pub async fn search_votes(
        &self,
        input: SearchVotesRequest,
    ) -> Result<(Vec<VoteFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_votes(input).await
    }
}

use super::{types::covenant::{Conduit, ConduitIndex, Covenant, CovenantIndex, CovenantMedia, Soulbind, SoulbindIndex}, WorldOfWarcraftClient};
use anyhow::Result;

impl WorldOfWarcraftClient {
    pub async fn get_covenant_index(&self) -> Result<CovenantIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/index"), "static")
                                .await?
                                .json::<CovenantIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_covenant(&self, id: u32) -> Result<Covenant> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/{}", id), "static")
                                .await?
                                .json::<Covenant>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_covenant_media(&self, id: u32) -> Result<CovenantMedia> {
        let response = self.client
                                .send_request(format!("/data/wow/media/covenant/{}", id), "static")
                                .await?
                                .json::<CovenantMedia>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_soulbind_index(&self) -> Result<SoulbindIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/soulbind/index"), "static")
                                .await?
                                .json::<SoulbindIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_soulbind(&self, id: u32) -> Result<Soulbind> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/soulbind/{}", id), "static")
                                .await?
                                .json::<Soulbind>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_conduit_index(&self) -> Result<ConduitIndex> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/conduit/index"), "static")
                                .await?
                                .json::<ConduitIndex>()
                                .await?;
        
                                Ok(response)
    }

    pub async fn get_conduit(&self) -> Result<Conduit> {
        let response = self.client
                                .send_request(format!("/data/wow/covenant/conduit/index"), "static")
                                .await?
                                .json::<Conduit>()
                                .await?;
        
                                Ok(response)
    }
    
}
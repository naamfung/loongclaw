use std::path::Path;

use async_trait::async_trait;
use loongclaw_clawhub::client::ClawHubClient;
use loongclaw_clawhub::install::{install_skill, InstallOptions, InstallResult};
use loongclaw_clawhub::lockfile::read_lockfile;
use loongclaw_clawhub::types::{LockFile, SearchResult, SkillMeta};

use crate::config::Config;
use crate::error::LoongClawError;

#[async_trait]
pub trait ClawHubGateway: Send + Sync {
    async fn search(
        &self,
        query: &str,
        limit: usize,
        sort: &str,
    ) -> Result<Vec<SearchResult>, LoongClawError>;
    async fn get_skill(&self, slug: &str) -> Result<SkillMeta, LoongClawError>;
    async fn install(
        &self,
        slug: &str,
        version: Option<&str>,
        skills_dir: &Path,
        lockfile_path: &Path,
        options: &InstallOptions,
    ) -> Result<InstallResult, LoongClawError>;
    fn read_lockfile(&self, path: &Path) -> Result<LockFile, LoongClawError>;
}

pub struct RegistryClawHubGateway {
    client: ClawHubClient,
}

impl RegistryClawHubGateway {
    pub fn from_config(config: &Config) -> Self {
        let client = ClawHubClient::new(&config.clawhub.registry, config.clawhub.token.clone());
        Self { client }
    }
}

#[async_trait]
impl ClawHubGateway for RegistryClawHubGateway {
    async fn search(
        &self,
        query: &str,
        limit: usize,
        sort: &str,
    ) -> Result<Vec<SearchResult>, LoongClawError> {
        self.client.search(query, limit, sort).await
    }

    async fn get_skill(&self, slug: &str) -> Result<SkillMeta, LoongClawError> {
        self.client.get_skill(slug).await
    }

    async fn install(
        &self,
        slug: &str,
        version: Option<&str>,
        skills_dir: &Path,
        lockfile_path: &Path,
        options: &InstallOptions,
    ) -> Result<InstallResult, LoongClawError> {
        install_skill(
            &self.client,
            slug,
            version,
            skills_dir,
            lockfile_path,
            options,
        )
        .await
    }

    fn read_lockfile(&self, path: &Path) -> Result<LockFile, LoongClawError> {
        read_lockfile(path)
    }
}

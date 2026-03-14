//! Share service for generating shareable URLs for pattern analysis results.
//!
//! This module provides functionality to:
//! - Generate unique share IDs for analysis reports
//! - Store and retrieve analysis reports
//! - Handle expiration and cleanup of old shares

use crate::models::AnalysisReport;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a shared analysis with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedAnalysis {
    pub id: String,
    pub report: AnalysisReport,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Service for managing shared analysis reports
pub struct ShareService {
    storage_dir: PathBuf,
}

impl ShareService {
    /// Create a new ShareService with the specified storage directory
    pub fn new(storage_dir: PathBuf) -> std::io::Result<Self> {
        fs::create_dir_all(&storage_dir)?;
        Ok(Self { storage_dir })
    }

    /// Generate a unique share ID from the analysis report content
    fn generate_share_id(report: &AnalysisReport) -> String {
        let content = serde_json::to_string(report).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        // Use first 16 chars of hex digest for shorter URLs
        format!("{:x}", result)[..16].to_string()
    }

    /// Get current Unix timestamp in seconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Create a shareable link for an analysis report
    ///
    /// Returns the share ID which can be used to construct a URL
    pub fn create_share(&self, report: AnalysisReport) -> std::io::Result<String> {
        let share_id = Self::generate_share_id(&report);
        let share_path = self.storage_dir.join(format!("{}.json", share_id));

        // Check if this analysis was already shared
        if share_path.exists() {
            if let Ok(existing) = self.get_share(&share_id) {
                // Update expiration time for existing share
                let updated = SharedAnalysis {
                    expires_at: Self::current_timestamp() + (30 * 24 * 60 * 60), // 30 days
                    ..existing
                };
                let json = serde_json::to_string_pretty(&updated)?;
                fs::write(&share_path, json)?;
                return Ok(share_id);
            }
        }

        let now = Self::current_timestamp();
        let shared_analysis = SharedAnalysis {
            id: share_id.clone(),
            report,
            created_at: now,
            expires_at: now + (30 * 24 * 60 * 60), // Expires in 30 days
        };

        let json = serde_json::to_string_pretty(&shared_analysis)?;
        fs::write(&share_path, json)?;

        Ok(share_id)
    }

    /// Retrieve a shared analysis by its ID
    pub fn get_share(&self, share_id: &str) -> std::io::Result<SharedAnalysis> {
        let share_path = self.storage_dir.join(format!("{}.json", share_id));

        if !share_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Share not found",
            ));
        }

        let json = fs::read_to_string(&share_path)?;
        let shared_analysis: SharedAnalysis = serde_json::from_str(&json)?;

        // Check if expired
        let now = Self::current_timestamp();
        if now > shared_analysis.expires_at {
            // Clean up expired share
            let _ = fs::remove_file(&share_path);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Share has expired",
            ));
        }

        Ok(shared_analysis)
    }

    /// Clean up expired shares
    pub fn cleanup_expired(&self) -> std::io::Result<usize> {
        let now = Self::current_timestamp();
        let mut cleaned = 0;

        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(shared: SharedAnalysis) = serde_json::from_str(&json) {
                        if now > shared.expires_at {
                            if fs::remove_file(&path).is_ok() {
                                cleaned += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(cleaned)
    }

    /// List all active shares (for admin purposes)
    pub fn list_active_shares(&self) -> std::io::Result<Vec<SharedAnalysis>> {
        let now = Self::current_timestamp();
        let mut active_shares = Vec::new();

        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(shared: SharedAnalysis) = serde_json::from_str(&json) {
                        if now <= shared.expires_at {
                            active_shares.push(shared);
                        }
                    }
                }
            }
        }

        Ok(active_shares)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DecisionNode, DecisionType, Import, Pattern, ReportMetadata};
    use tempfile::TempDir;

    fn create_test_report() -> AnalysisReport {
        AnalysisReport {
            file_path: "test.rs".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            patterns: vec![Pattern {
                pattern_type: "Error Handling".to_string(),
                start_line: 1,
                end_line: 5,
                confidence: 0.95,
                reasoning: Some("Uses Result type".to_string()),
                code_snippet: "fn test() -> Result<()> {}".to_string(),
            }],
            import_suggestions: vec![Import {
                module: "std::error::Error".to_string(),
                reason: "Error handling".to_string(),
                confidence: 0.9,
            }],
            decision_nodes: vec![DecisionNode {
                id: "node1".to_string(),
                decision_type: DecisionType::ErrorHandling,
                description: "Error handling approach".to_string(),
                alternatives: vec![],
                chosen: "Result".to_string(),
                confidence: 0.95,
            }],
            overall_confidence: 0.93,
            metadata: ReportMetadata {
                analyzer_version: "1.0.0".to_string(),
                total_lines: 100,
                analyzed_constructs: 10,
            },
        }
    }

    #[test]
    fn test_create_and_retrieve_share() {
        let temp_dir = TempDir::new().unwrap();
        let service = ShareService::new(temp_dir.path().to_path_buf()).unwrap();

        let report = create_test_report();
        let share_id = service.create_share(report.clone()).unwrap();

        assert!(!share_id.is_empty());
        assert_eq!(share_id.len(), 16);

        let retrieved = service.get_share(&share_id).unwrap();
        assert_eq!(retrieved.id, share_id);
        assert_eq!(retrieved.report.file_path, report.file_path);
    }

    #[test]
    fn test_duplicate_share_same_id() {
        let temp_dir = TempDir::new().unwrap();
        let service = ShareService::new(temp_dir.path().to_path_buf()).unwrap();

        let report = create_test_report();
        let share_id1 = service.create_share(report.clone()).unwrap();
        let share_id2 = service.create_share(report).unwrap();

        // Same content should generate same ID
        assert_eq!(share_id1, share_id2);
    }

    #[test]
    fn test_share_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let service = ShareService::new(temp_dir.path().to_path_buf()).unwrap();

        let result = service.get_share("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_active_shares() {
        let temp_dir = TempDir::new().unwrap();
        let service = ShareService::new(temp_dir.path().to_path_buf()).unwrap();

        let report = create_test_report();
        service.create_share(report).unwrap();

        let active = service.list_active_shares().unwrap();
        assert_eq!(active.len(), 1);
    }
}

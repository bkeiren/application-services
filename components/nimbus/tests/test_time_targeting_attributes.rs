mod common;
#[cfg(feature = "rkv-safe-mode")]
#[cfg(test)]
mod test {

    use super::common::{new_app_context, new_test_client_with_db};
    #[cfg(feature = "rkv-safe-mode")]
    use nimbus::error::Result;
    use nimbus::TargetingAttributes;
    use serde_json::json;
    use tempdir::TempDir;

    #[test]
    fn test_days_since_install() -> Result<()> {
        let temp_dir = TempDir::new("test_restart_opt_in")?;
        let mut client = new_test_client_with_db(&temp_dir)?;
        let targeting_attributes = TargetingAttributes {
            app_context: new_app_context(),
            days_since_install: Some(10),
            days_since_update: None,
            is_already_enrolled: false,
        };
        client.with_targeting_attributes(targeting_attributes);
        client.initialize()?;
        let experiment_json = serde_json::to_string(&json!({
            "data": [{
                "schemaVersion": "1.0.0",
                "slug": "secure-gold",
                "endDate": null,
                "featureIds": ["some-feature"],
                "branches": [
                    {
                    "slug": "control",
                    "ratio": 1
                    },
                    {
                    "slug": "treatment",
                    "ratio": 1
                    }
                ],
                "channel": "nightly",
                "probeSets": [],
                "startDate": null,
                "appName": "fenix",
                "appId": "org.mozilla.fenix",
                "bucketConfig": {
                    "count": 10000,
                    "start": 0,
                    "total": 10000,
                    "namespace": "secure-gold",
                    "randomizationUnit": "nimbus_id"
                },
                "targeting": "days_since_install == 10",
                "userFacingName": "test experiment",
                "referenceBranch": "control",
                "isEnrollmentPaused": false,
                "proposedEnrollment": 7,
                "userFacingDescription": "This is a test experiment for testing purposes.",
                "id": "secure-copper",
                "last_modified": 1_602_197_324_372i64,
            }
        ]}))?;
        client.set_experiments_locally(experiment_json)?;
        client.apply_pending_experiments()?;

        // The targeting targeted days_since_install == 10, which is true in the client
        // so we should be enrolled in that experiment
        let active_experiments = client.get_active_experiments()?;
        assert_eq!(active_experiments.len(), 1);
        assert_eq!(active_experiments[0].slug, "secure-gold");
        Ok(())
    }

    #[test]
    fn test_days_since_install_failed_targeting() -> Result<()> {
        let temp_dir = TempDir::new("test_restart_opt_in")?;
        let mut client = new_test_client_with_db(&temp_dir)?;
        let targeting_attributes = TargetingAttributes {
            app_context: new_app_context(),
            days_since_install: Some(10),
            days_since_update: None,
            is_already_enrolled: false,
        };
        client.with_targeting_attributes(targeting_attributes);
        client.initialize()?;
        let experiment_json = serde_json::to_string(&json!({
            "data": [{
                "schemaVersion": "1.0.0",
                "slug": "secure-gold",
                "endDate": null,
                "featureIds": ["some-feature"],
                "branches": [
                    {
                    "slug": "control",
                    "ratio": 1
                    },
                    {
                    "slug": "treatment",
                    "ratio": 1
                    }
                ],
                "channel": "nightly",
                "probeSets": [],
                "startDate": null,
                "appName": "fenix",
                "appId": "org.mozilla.fenix",
                "bucketConfig": {
                    "count": 10000,
                    "start": 0,
                    "total": 10000,
                    "namespace": "secure-gold",
                    "randomizationUnit": "nimbus_id"
                },
                "targeting": "days_since_install < 10",
                "userFacingName": "test experiment",
                "referenceBranch": "control",
                "isEnrollmentPaused": false,
                "proposedEnrollment": 7,
                "userFacingDescription": "This is a test experiment for testing purposes.",
                "id": "secure-copper",
                "last_modified": 1_602_197_324_372i64,
            }
        ]}))?;
        client.set_experiments_locally(experiment_json)?;
        client.apply_pending_experiments()?;

        // The targeting targeted days_since_install < 10, which is false in the client
        // so we should be enrolled in that experiment
        let active_experiments = client.get_active_experiments()?;
        assert_eq!(active_experiments.len(), 0);
        Ok(())
    }

    #[test]
    fn test_days_since_update() -> Result<()> {
        let temp_dir = TempDir::new("test_restart_opt_in")?;
        let mut client = new_test_client_with_db(&temp_dir)?;
        let targeting_attributes = TargetingAttributes {
            app_context: new_app_context(),
            days_since_install: None,
            days_since_update: Some(10),
            is_already_enrolled: false,
        };
        client.with_targeting_attributes(targeting_attributes);
        client.initialize()?;
        let experiment_json = serde_json::to_string(&json!({
            "data": [{
                "schemaVersion": "1.0.0",
                "slug": "secure-gold",
                "endDate": null,
                "featureIds": ["some-feature"],
                "branches": [
                    {
                    "slug": "control",
                    "ratio": 1
                    },
                    {
                    "slug": "treatment",
                    "ratio": 1
                    }
                ],
                "channel": "nightly",
                "probeSets": [],
                "startDate": null,
                "appName": "fenix",
                "appId": "org.mozilla.fenix",
                "bucketConfig": {
                    "count": 10000,
                    "start": 0,
                    "total": 10000,
                    "namespace": "secure-gold",
                    "randomizationUnit": "nimbus_id"
                },
                "targeting": "days_since_update == 10",
                "userFacingName": "test experiment",
                "referenceBranch": "control",
                "isEnrollmentPaused": false,
                "proposedEnrollment": 7,
                "userFacingDescription": "This is a test experiment for testing purposes.",
                "id": "secure-copper",
                "last_modified": 1_602_197_324_372i64,
            }
        ]}))?;
        client.set_experiments_locally(experiment_json)?;
        client.apply_pending_experiments()?;

        // The targeting targeted days_since_update == 10, which is true in the client
        // so we should be enrolled in that experiment
        let active_experiments = client.get_active_experiments()?;
        assert_eq!(active_experiments.len(), 1);
        assert_eq!(active_experiments[0].slug, "secure-gold");
        Ok(())
    }

    #[test]
    fn test_days_since_update_failed_targeting() -> Result<()> {
        let temp_dir = TempDir::new("test_restart_opt_in")?;
        let mut client = new_test_client_with_db(&temp_dir)?;
        let targeting_attributes = TargetingAttributes {
            app_context: new_app_context(),
            days_since_install: None,
            days_since_update: Some(10),
            is_already_enrolled: false,
        };
        client.with_targeting_attributes(targeting_attributes);
        client.initialize()?;
        let experiment_json = serde_json::to_string(&json!({
            "data": [{
                "schemaVersion": "1.0.0",
                "slug": "secure-gold",
                "endDate": null,
                "featureIds": ["some-feature"],
                "branches": [
                    {
                    "slug": "control",
                    "ratio": 1
                    },
                    {
                    "slug": "treatment",
                    "ratio": 1
                    }
                ],
                "channel": "nightly",
                "probeSets": [],
                "startDate": null,
                "appName": "fenix",
                "appId": "org.mozilla.fenix",
                "bucketConfig": {
                    "count": 10000,
                    "start": 0,
                    "total": 10000,
                    "namespace": "secure-gold",
                    "randomizationUnit": "nimbus_id"
                },
                "targeting": "days_since_update < 10",
                "userFacingName": "test experiment",
                "referenceBranch": "control",
                "isEnrollmentPaused": false,
                "proposedEnrollment": 7,
                "userFacingDescription": "This is a test experiment for testing purposes.",
                "id": "secure-copper",
                "last_modified": 1_602_197_324_372i64,
            }
        ]}))?;
        client.set_experiments_locally(experiment_json)?;
        client.apply_pending_experiments()?;

        // The targeting targeted days_since_update < 10, which is false in the client
        // so we should be enrolled in that experiment
        let active_experiments = client.get_active_experiments()?;
        assert_eq!(active_experiments.len(), 0);
        Ok(())
    }
}

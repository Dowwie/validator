use std::{
    collections::HashMap,
    ffi::OsString,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

use rustix::{
    fs::{CWD, RenameFlags, renameat_with},
    io::Errno,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    Diagnostic, DiagnosticCode, Result,
    model::common::{
        ArtifactDigest, ArtifactSnapshot, EvidenceBinding, SourceDefinition, SourceId,
    },
};

/// Exact input snapshots retained before semantic validation.
#[derive(Debug)]
pub(crate) struct InputArtifacts {
    golden: ArtifactSnapshot,
    predictions: ArtifactSnapshot,
    config: ArtifactSnapshot,
}

/// Stored run inputs whose paths and manifest digests have been checked.
#[derive(Debug)]
pub(crate) struct StoredRun {
    root: PathBuf,
    artifacts: InputArtifacts,
    report: Value,
    report_digest: ArtifactDigest,
}

impl StoredRun {
    pub(crate) fn artifacts(&self) -> &InputArtifacts {
        &self.artifacts
    }

    pub(crate) fn report(&self) -> &Value {
        &self.report
    }

    pub(crate) const fn report_digest(&self) -> ArtifactDigest {
        self.report_digest
    }

    pub(crate) fn verify_evidence(
        &self,
        sources: &HashMap<SourceId, SourceDefinition>,
    ) -> Result<Vec<EvidenceBinding>> {
        let mut ordered_sources: Vec<_> = sources.iter().collect();
        ordered_sources.sort_unstable_by(|(left, _), (right, _)| {
            left.as_str().as_bytes().cmp(right.as_str().as_bytes())
        });

        let manifest = self
            .report
            .get("artifacts")
            .and_then(Value::as_array)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
        let mut bindings = Vec::new();
        for (source_id, source) in ordered_sources {
            for (evidence_index, original_path) in source.evidence().iter().enumerate() {
                let ordinal = bindings.len();
                let stored_path = PathBuf::from("evidence").join(format!("{ordinal}.bin"));
                let entry = manifest
                    .get(ordinal + 3)
                    .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Provenance))?;
                if entry.get("kind").and_then(Value::as_str) != Some("evidence")
                    || entry.get("source_id").and_then(Value::as_str) != Some(source_id.as_str())
                    || entry.get("evidence_index").and_then(Value::as_u64)
                        != Some(
                            u64::try_from(evidence_index)
                                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Provenance))?,
                        )
                    || entry.get("original_path").and_then(Value::as_str) != Some(original_path)
                    || entry.get("path").and_then(Value::as_str) != stored_path.to_str()
                {
                    return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
                }
                let (bytes, digest) = read_contained(&self.root, &stored_path)?;
                if entry.get("sha256").and_then(Value::as_str) != Some(digest.to_string().as_str())
                {
                    return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
                }
                bindings.push(EvidenceBinding::new(
                    source_id.clone(),
                    evidence_index,
                    original_path.clone(),
                    stored_path,
                    bytes,
                    digest,
                ));
            }
        }
        if manifest.len() != bindings.len() + 3 {
            return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
        }
        Ok(bindings)
    }
}

/// Reads a stored run through paths confined to its canonical directory.
pub(crate) fn load_stored_run(directory: &Path) -> Result<StoredRun> {
    let root = fs::canonicalize(directory).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
    if !root.is_dir() {
        return Err(Diagnostic::for_code(DiagnosticCode::Io));
    }
    let (report_bytes, report_digest) = read_contained(&root, Path::new("report.json"))?;
    let report: Value = serde_json::from_slice(&report_bytes)
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?;
    if report.get("schema_version").and_then(Value::as_u64) != Some(crate::WIRE_VERSION as u64)
        || report.get("kind").and_then(Value::as_str) != Some("evaluation")
        || report.get("status").and_then(Value::as_str) != Some("complete")
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Schema));
    }
    let golden = stored_snapshot(&root, &report, "golden", "golden.json")?;
    let predictions = stored_snapshot(&root, &report, "predictions", "predictions.json")?;
    let config = stored_snapshot(&root, &report, "config", "config.json")?;
    Ok(StoredRun {
        root,
        artifacts: InputArtifacts {
            golden,
            predictions,
            config,
        },
        report,
        report_digest,
    })
}

fn stored_snapshot(
    root: &Path,
    report: &Value,
    kind: &str,
    expected_path: &str,
) -> Result<ArtifactSnapshot> {
    let manifest = report
        .get("artifacts")
        .and_then(Value::as_array)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let entry = manifest
        .iter()
        .find(|entry| entry.get("kind").and_then(Value::as_str) == Some(kind))
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Provenance))?;
    if entry.get("path").and_then(Value::as_str) != Some(expected_path) {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    let (bytes, digest) = read_contained(root, Path::new(expected_path))?;
    if entry.get("sha256").and_then(Value::as_str) != Some(digest.to_string().as_str()) {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    Ok(ArtifactSnapshot::new(
        root.join(expected_path),
        PathBuf::from(expected_path),
        bytes,
        digest,
    ))
}

fn read_contained(root: &Path, relative: &Path) -> Result<(Vec<u8>, ArtifactDigest)> {
    if relative.is_absolute()
        || relative.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        })
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    let path = root.join(relative);
    let resolved = fs::canonicalize(&path).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
    if !resolved.starts_with(root) {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    read_bytes_and_digest(&resolved)
}

/// The absolute report receipt returned by a successful immutable publication.
#[derive(Debug)]
pub(crate) struct PublishedReport {
    report_path: PathBuf,
    report_digest: ArtifactDigest,
}

impl PublishedReport {
    /// Borrows the absolute path to the published report file.
    pub(crate) fn report_path(&self) -> &Path {
        &self.report_path
    }

    /// Returns the digest of the exact published report bytes.
    pub(crate) const fn report_digest(&self) -> ArtifactDigest {
        self.report_digest
    }
}

static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

impl InputArtifacts {
    /// Borrows the exact golden-dataset snapshot.
    pub(crate) fn golden(&self) -> &ArtifactSnapshot {
        &self.golden
    }

    /// Borrows the exact prediction-artifact snapshot.
    pub(crate) fn predictions(&self) -> &ArtifactSnapshot {
        &self.predictions
    }

    /// Borrows the exact evaluation-configuration snapshot.
    pub(crate) fn config(&self) -> &ArtifactSnapshot {
        &self.config
    }
}

/// Reads each submitted input artifact once and retains its exact bytes and digest.
pub(crate) fn load_input_artifacts(
    golden_path: &Path,
    predictions_path: &Path,
    config_path: &Path,
) -> Result<InputArtifacts> {
    Ok(InputArtifacts {
        golden: load_snapshot(golden_path, "golden.json")?,
        predictions: load_snapshot(predictions_path, "predictions.json")?,
        config: load_snapshot(config_path, "config.json")?,
    })
}

/// Reads every checked source-evidence entry in deterministic stored-path order.
pub(crate) fn load_evidence_bindings(
    predictions: &ArtifactSnapshot,
    sources: &HashMap<SourceId, SourceDefinition>,
) -> Result<Vec<EvidenceBinding>> {
    let mut ordered_sources: Vec<_> = sources.iter().collect();
    ordered_sources.sort_unstable_by(|(left, _), (right, _)| {
        left.as_str().as_bytes().cmp(right.as_str().as_bytes())
    });

    let mut bindings = Vec::new();
    for (source_id, source) in ordered_sources {
        for (evidence_index, original_path) in source.evidence().iter().enumerate() {
            let ordinal = bindings.len();
            let stored_path = PathBuf::from("evidence").join(format!("{ordinal}.bin"));
            let resolved_path = resolve_evidence_path(predictions.original_path(), original_path);
            let (bytes, digest) = read_bytes_and_digest(&resolved_path)?;
            bindings.push(EvidenceBinding::new(
                source_id.clone(),
                evidence_index,
                original_path.clone(),
                stored_path,
                bytes,
                digest,
            ));
        }
    }
    Ok(bindings)
}

/// Publishes complete exact snapshots without replacing an existing destination.
pub(crate) fn publish_run(
    final_directory: &Path,
    report: &[u8],
    artifacts: &InputArtifacts,
    evidence: &[EvidenceBinding],
) -> Result<PublishedReport> {
    publish_with_hook(final_directory, report, artifacts, evidence, |_| Ok(()))
}

/// Publishes one comparison document with the established private no-replace path.
pub(crate) fn publish_comparison(
    final_directory: &Path,
    document: &[u8],
) -> Result<PublishedReport> {
    publish_document(
        final_directory,
        "comparison.json",
        document,
        |_| Ok(()),
        |_| Ok(()),
    )
}

fn publish_document(
    final_directory: &Path,
    result_name: &str,
    document: &[u8],
    populate: impl FnOnce(&Path) -> Result<()>,
    before_publish: impl FnOnce(&Path) -> Result<()>,
) -> Result<PublishedReport> {
    let final_directory = absolute_path(final_directory)?;
    let temporary_directory = create_temporary_sibling(&final_directory)?;
    let result = (|| {
        populate(&temporary_directory)?;
        write_private_file(&temporary_directory.join(result_name), document)?;
        before_publish(&temporary_directory)?;
        rename_no_replace(&temporary_directory, &final_directory)?;
        Ok(PublishedReport {
            report_path: final_directory.join(result_name),
            report_digest: digest_bytes(document),
        })
    })();
    match result {
        Ok(published) => Ok(published),
        Err(error) => {
            fs::remove_dir_all(&temporary_directory)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
            Err(error)
        }
    }
}

fn publish_with_hook(
    final_directory: &Path,
    report: &[u8],
    artifacts: &InputArtifacts,
    evidence: &[EvidenceBinding],
    before_publish: impl FnOnce(&Path) -> Result<()>,
) -> Result<PublishedReport> {
    publish_document(
        final_directory,
        "report.json",
        report,
        |temporary_directory| write_inputs(temporary_directory, artifacts, evidence),
        before_publish,
    )
}

fn write_inputs(
    temporary_directory: &Path,
    artifacts: &InputArtifacts,
    evidence: &[EvidenceBinding],
) -> Result<()> {
    write_private_file(
        &temporary_directory.join(artifacts.golden().stored_path()),
        artifacts.golden().bytes(),
    )?;
    write_private_file(
        &temporary_directory.join(artifacts.predictions().stored_path()),
        artifacts.predictions().bytes(),
    )?;
    write_private_file(
        &temporary_directory.join(artifacts.config().stored_path()),
        artifacts.config().bytes(),
    )?;
    write_evidence(temporary_directory, evidence)?;
    Ok(())
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    std::env::current_dir()
        .map(|current_directory| current_directory.join(path))
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))
}

fn create_temporary_sibling(final_directory: &Path) -> Result<PathBuf> {
    let Some(parent) = final_directory.parent() else {
        return Err(Diagnostic::for_code(DiagnosticCode::Io));
    };
    let Some(name) = final_directory.file_name() else {
        return Err(Diagnostic::for_code(DiagnosticCode::Io));
    };

    for _ in 0..64 {
        let ordinal = NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let mut temporary_name = OsString::from(".");
        temporary_name.push(name);
        temporary_name.push(format!(".validator-{ordinal}"));
        let temporary_directory = parent.join(temporary_name);
        match fs::create_dir(&temporary_directory) {
            Ok(()) => {
                if let Err(error) = set_private_permissions(&temporary_directory, 0o700) {
                    fs::remove_dir_all(&temporary_directory)
                        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
                    return Err(error);
                }
                return Ok(temporary_directory);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err(Diagnostic::for_code(DiagnosticCode::Io)),
        }
    }

    Err(Diagnostic::for_code(DiagnosticCode::Io))
}

fn write_evidence(temporary_directory: &Path, evidence: &[EvidenceBinding]) -> Result<()> {
    if evidence.is_empty() {
        return Ok(());
    }

    let evidence_directory = temporary_directory.join("evidence");
    fs::create_dir(&evidence_directory).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
    set_private_permissions(&evidence_directory, 0o700)?;
    for binding in evidence {
        write_private_file(
            &temporary_directory.join(binding.stored_path()),
            binding.bytes(),
        )?;
    }
    Ok(())
}

fn write_private_file(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
    set_private_permissions(path, 0o600)
}

fn set_private_permissions(path: &Path, mode: u32) -> Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))
}

fn rename_no_replace(temporary_directory: &Path, final_directory: &Path) -> Result<()> {
    match renameat_with(
        CWD,
        temporary_directory,
        CWD,
        final_directory,
        RenameFlags::NOREPLACE,
    ) {
        Ok(()) => Ok(()),
        Err(Errno::EXIST | Errno::NOTEMPTY) => {
            Err(Diagnostic::for_code(DiagnosticCode::OutputExists))
        }
        Err(_) => Err(Diagnostic::for_code(DiagnosticCode::Io)),
    }
}

fn load_snapshot(path: &Path, stored_path: &str) -> Result<ArtifactSnapshot> {
    let (bytes, digest) = read_bytes_and_digest(path)?;
    Ok(ArtifactSnapshot::new(
        path.to_path_buf(),
        PathBuf::from(stored_path),
        bytes,
        digest,
    ))
}

fn resolve_evidence_path(prediction_path: &Path, original_path: &str) -> PathBuf {
    match prediction_path.parent() {
        Some(parent) => parent.join(original_path),
        None => PathBuf::from(original_path),
    }
}

fn read_bytes_and_digest(path: &Path) -> Result<(Vec<u8>, ArtifactDigest)> {
    let bytes = fs::read(path).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
    let digest = digest_bytes(&bytes);
    Ok((bytes, digest))
}

fn digest_bytes(bytes: &[u8]) -> ArtifactDigest {
    ArtifactDigest::from_bytes(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::HashMap,
        fs,
        os::unix::fs::{PermissionsExt, symlink},
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use serde_json::json;

    use super::{
        InputArtifacts, load_evidence_bindings, load_input_artifacts, publish_run,
        publish_with_hook,
    };
    use crate::{
        Diagnostic, DiagnosticCode,
        model::common::{
            ArtifactSnapshot, EvidenceBinding, SourceDefinition, SourceId, SourceKind,
        },
    };

    static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let ordinal = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "validator-artifacts-{}-{ordinal}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }

    fn write_file(path: &Path, bytes: &[u8]) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn input_paths(directory: &TestDirectory) -> (PathBuf, PathBuf, PathBuf) {
        let input = directory.path().join("input");
        let golden = input.join("golden.json");
        let predictions = input.join("predictions.json");
        let config = input.join("config.json");
        write_file(&golden, b"{\"value\":\"golden\"}");
        write_file(&predictions, b"{\"value\":\"predictions\"}");
        write_file(&config, b"{\"value\":\"config\"}");
        (golden, predictions, config)
    }

    fn source(evidence: &[&str]) -> SourceDefinition {
        SourceDefinition::new(
            SourceKind::Classifier,
            "model".to_owned(),
            json!({}),
            None,
            evidence.iter().map(ToString::to_string).collect(),
            HashMap::new(),
            None,
        )
        .unwrap()
    }

    fn source_id(value: &str) -> SourceId {
        SourceId::try_from(value).unwrap()
    }

    fn publication_inputs(directory: &TestDirectory) -> (InputArtifacts, Vec<EvidenceBinding>) {
        let (golden, predictions, config) = input_paths(directory);
        write_file(
            &directory.path().join("input/evidence/payload.bin"),
            b"evidence payload",
        );
        let artifacts = load_input_artifacts(&golden, &predictions, &config).unwrap();
        let sources = HashMap::from([(source_id("source"), source(&["evidence/payload.bin"]))]);
        let evidence = load_evidence_bindings(artifacts.predictions(), &sources).unwrap();
        (artifacts, evidence)
    }

    fn entries(path: &Path) -> Vec<String> {
        let mut entries = fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        entries.sort_unstable();
        entries
    }

    fn assert_snapshot(snapshot: &ArtifactSnapshot, bytes: &[u8], digest: &str, path: &str) {
        assert_eq!(snapshot.bytes(), bytes);
        assert_eq!(snapshot.digest().to_string(), digest);
        assert_eq!(snapshot.stored_path(), Path::new(path));
    }

    #[test]
    fn artifact_exact_bytes() {
        let directory = TestDirectory::new();
        let (golden, predictions, config) = input_paths(&directory);

        let artifacts = load_input_artifacts(&golden, &predictions, &config).unwrap();

        assert_eq!(artifacts.golden().original_path(), golden);
        assert_eq!(artifacts.predictions().original_path(), predictions);
        assert_eq!(artifacts.config().original_path(), config);
        assert_snapshot(
            artifacts.golden(),
            b"{\"value\":\"golden\"}",
            "b7fd587d6da1189fbf961d5162cf21ec7dfe27ffaf951e133f5048624b0eec4d",
            "golden.json",
        );
        assert_snapshot(
            artifacts.predictions(),
            b"{\"value\":\"predictions\"}",
            "3a579e1227f987caa851b378c3cb3b861ce60a3771c765744bde461c723732dd",
            "predictions.json",
        );
        assert_snapshot(
            artifacts.config(),
            b"{\"value\":\"config\"}",
            "75af282886e284f3f721eff48719a30f63b48239a8aeac90939b0f51506ef29e",
            "config.json",
        );
    }

    #[test]
    fn evidence_ordinal_binding() {
        let directory = TestDirectory::new();
        let (golden, predictions, config) = input_paths(&directory);
        write_file(
            &directory.path().join("input/same/duplicate.bin"),
            b"same basename one",
        );
        write_file(
            &directory.path().join("input/other/duplicate.bin"),
            b"same basename two",
        );
        write_file(
            &directory.path().join("parent-relative.bin"),
            b"parent relative evidence",
        );
        write_file(
            &directory.path().join("input/evidence/unused.bin"),
            b"alpha evidence",
        );
        write_file(
            &directory.path().join("input/evidence/repeated.bin"),
            b"alpha evidence",
        );
        let artifacts = load_input_artifacts(&golden, &predictions, &config).unwrap();
        let sources = HashMap::from([
            (
                source_id("z"),
                source(&["evidence/repeated.bin", "evidence/repeated.bin"]),
            ),
            (source_id("β"), source(&["evidence/repeated.bin"])),
            (
                source_id("a"),
                source(&[
                    "same/duplicate.bin",
                    "other/duplicate.bin",
                    "../parent-relative.bin",
                ]),
            ),
            (source_id("unused"), source(&["evidence/unused.bin"])),
        ]);

        let bindings = load_evidence_bindings(artifacts.predictions(), &sources).unwrap();
        let expected = [
            (
                "a",
                0,
                "same/duplicate.bin",
                "evidence/0.bin",
                b"same basename one".as_slice(),
                "c0ce1fd25c8c53d9855416855af6003cecd29030c906ba21691e41daf3e1f106",
            ),
            (
                "a",
                1,
                "other/duplicate.bin",
                "evidence/1.bin",
                b"same basename two".as_slice(),
                "1823e421813d2b4f001b4a5e200bf67f50612716f4aee619ed7bb212e07e81bc",
            ),
            (
                "a",
                2,
                "../parent-relative.bin",
                "evidence/2.bin",
                b"parent relative evidence".as_slice(),
                "5ade6c6055db617fc600b1fecf214b2901640d552152983664ae236f9094c634",
            ),
            (
                "unused",
                0,
                "evidence/unused.bin",
                "evidence/3.bin",
                b"alpha evidence".as_slice(),
                "6d821f2a541fe912714c3285118da83656218f4b12421ed3db79385da8d51696",
            ),
            (
                "z",
                0,
                "evidence/repeated.bin",
                "evidence/4.bin",
                b"alpha evidence".as_slice(),
                "6d821f2a541fe912714c3285118da83656218f4b12421ed3db79385da8d51696",
            ),
            (
                "z",
                1,
                "evidence/repeated.bin",
                "evidence/5.bin",
                b"alpha evidence".as_slice(),
                "6d821f2a541fe912714c3285118da83656218f4b12421ed3db79385da8d51696",
            ),
            (
                "β",
                0,
                "evidence/repeated.bin",
                "evidence/6.bin",
                b"alpha evidence".as_slice(),
                "6d821f2a541fe912714c3285118da83656218f4b12421ed3db79385da8d51696",
            ),
        ];

        assert_eq!(bindings.len(), expected.len());
        for (binding, (source, index, original, stored, bytes, digest)) in
            bindings.iter().zip(expected)
        {
            assert_eq!(binding.source_id().as_str(), source);
            assert_eq!(binding.evidence_index(), index);
            assert_eq!(binding.original_path(), original);
            assert_eq!(binding.stored_path(), Path::new(stored));
            assert_eq!(binding.bytes(), bytes);
            assert_eq!(binding.digest().to_string(), digest);
        }
    }

    #[test]
    fn missing_input_and_evidence_return_io() {
        let directory = TestDirectory::new();
        let (golden, predictions, config) = input_paths(&directory);

        assert_eq!(
            load_input_artifacts(
                &directory.path().join("missing.json"),
                &predictions,
                &config
            )
            .unwrap_err()
            .code(),
            DiagnosticCode::Io,
        );
        let artifacts = load_input_artifacts(&golden, &predictions, &config).unwrap();
        let sources = HashMap::from([(source_id("source"), source(&["missing.bin"]))]);
        assert_eq!(
            load_evidence_bindings(artifacts.predictions(), &sources)
                .unwrap_err()
                .code(),
            DiagnosticCode::Io,
        );
    }

    #[test]
    fn publish_no_replace_race() {
        let directory = TestDirectory::new();
        let (artifacts, evidence) = publication_inputs(&directory);
        let run_parent = directory.path().join("runs");
        fs::create_dir(&run_parent).unwrap();
        let report = b"{\"status\":\"complete\"}";
        let successful_directory = run_parent.join("successful");

        let published = publish_run(&successful_directory, report, &artifacts, &evidence).unwrap();

        assert_eq!(
            entries(&successful_directory),
            [
                "config.json",
                "evidence",
                "golden.json",
                "predictions.json",
                "report.json"
            ]
        );
        assert_eq!(
            fs::read(successful_directory.join("golden.json")).unwrap(),
            artifacts.golden().bytes()
        );
        assert_eq!(
            fs::read(successful_directory.join("predictions.json")).unwrap(),
            artifacts.predictions().bytes()
        );
        assert_eq!(
            fs::read(successful_directory.join("config.json")).unwrap(),
            artifacts.config().bytes()
        );
        assert_eq!(
            fs::read(successful_directory.join(evidence[0].stored_path())).unwrap(),
            evidence[0].bytes()
        );
        assert_eq!(
            fs::read(successful_directory.join("report.json")).unwrap(),
            report
        );
        assert!(published.report_path().is_absolute());
        assert_eq!(
            published.report_path(),
            successful_directory.join("report.json")
        );
        assert_eq!(
            published.report_digest().to_string(),
            "a1e73038b20b14c8814f26b22e8107c1b410db346c6736eea60480137a09109b"
        );
        assert_eq!(
            fs::metadata(&successful_directory)
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(successful_directory.join("evidence"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(successful_directory.join("report.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );

        for kind in ["file", "directory", "symlink"] {
            let destination = run_parent.join(kind);
            let temporary = RefCell::new(None);
            let result = publish_with_hook(
                &destination,
                report,
                &artifacts,
                &evidence,
                |temporary_directory| {
                    *temporary.borrow_mut() = Some(temporary_directory.to_path_buf());
                    assert!(temporary_directory.join("report.json").exists());
                    assert!(temporary_directory.join("evidence/0.bin").exists());
                    match kind {
                        "file" => write_file(&destination, b"file winner"),
                        "directory" => {
                            fs::create_dir(&destination).unwrap();
                            write_file(&destination.join("keep"), b"directory winner");
                        }
                        "symlink" => symlink("symlink winner", &destination).unwrap(),
                        _ => unreachable!(),
                    }
                    Ok(())
                },
            );

            assert_eq!(result.unwrap_err().code(), DiagnosticCode::OutputExists);
            assert!(!temporary.borrow().as_ref().unwrap().exists());
            match kind {
                "file" => assert_eq!(fs::read(&destination).unwrap(), b"file winner"),
                "directory" => {
                    assert!(destination.is_dir());
                    assert_eq!(
                        fs::read(destination.join("keep")).unwrap(),
                        b"directory winner"
                    );
                }
                "symlink" => {
                    assert!(
                        fs::symlink_metadata(&destination)
                            .unwrap()
                            .file_type()
                            .is_symlink()
                    );
                    assert_eq!(
                        fs::read_link(&destination).unwrap(),
                        PathBuf::from("symlink winner")
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn publish_late_failure() {
        let directory = TestDirectory::new();
        let (artifacts, evidence) = publication_inputs(&directory);
        let run_parent = directory.path().join("runs");
        fs::create_dir(&run_parent).unwrap();
        let final_directory = run_parent.join("failed");
        let unrelated = run_parent.join("unrelated");
        write_file(&unrelated, b"unrelated sibling");
        let temporary = RefCell::new(None);
        let golden_bytes = fs::read(artifacts.golden().original_path()).unwrap();
        let prediction_bytes = fs::read(artifacts.predictions().original_path()).unwrap();
        let config_bytes = fs::read(artifacts.config().original_path()).unwrap();
        let evidence_path = directory.path().join("input/evidence/payload.bin");
        let evidence_bytes = fs::read(&evidence_path).unwrap();

        let result = publish_with_hook(
            &final_directory,
            b"{\"status\":\"complete\"}",
            &artifacts,
            &evidence,
            |temporary_directory| {
                *temporary.borrow_mut() = Some(temporary_directory.to_path_buf());
                assert!(temporary_directory.join("golden.json").exists());
                assert!(temporary_directory.join("report.json").exists());
                Err(Diagnostic::for_code(DiagnosticCode::Io))
            },
        );

        assert_eq!(result.unwrap_err().code(), DiagnosticCode::Io);
        assert!(!final_directory.exists());
        assert!(!temporary.borrow().as_ref().unwrap().exists());
        assert_eq!(
            fs::read(artifacts.golden().original_path()).unwrap(),
            golden_bytes
        );
        assert_eq!(
            fs::read(artifacts.predictions().original_path()).unwrap(),
            prediction_bytes
        );
        assert_eq!(
            fs::read(artifacts.config().original_path()).unwrap(),
            config_bytes
        );
        assert_eq!(fs::read(evidence_path).unwrap(), evidence_bytes);
        assert_eq!(fs::read(unrelated).unwrap(), b"unrelated sibling");
    }
}

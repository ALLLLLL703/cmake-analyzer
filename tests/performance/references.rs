// AI-generated release benchmarks; no machine-specific acceptance thresholds.

//! Measure full analysis and cold/warm Workspace reference-index costs.
//! The ignored release benchmark writes temporary CMake text, prints samples, and removes its directory.
//! It parses fixtures only; it does not configure projects or execute their commands.

use cmake_analyzer::{
    parser::{
        lexer::model::{Lexer, RawBuffer},
        nparser::model::NParser,
        semanticer::model::{FileId, Semanticer},
    },
    workspace::model::Workspace,
};
use std::{hint::black_box, path::PathBuf, time::Instant};

/// Measured durations retained after warmups for each benchmark case.
static SAMPLES: usize = 21;
/// Untimed-for-report analysis iterations before retaining durations; warmup work still runs.
static WARMUPS: usize = 3;
/// Number of included files in the cold/warm Workspace fixture.
static MODULES: usize = 4;
/// Variable-write/message pairs generated in each included benchmark module.
static MODULE_PAIRS: usize = 300;

/// Build deterministic version-pinned CMake text with indexed set/message pairs.
/// Each message reads its variable twice; only memory is changed.
///
/// # Arguments
/// - `pairs` `[in]`: number of variable-write/message pairs to append after the version line.
///
/// # Returns
/// Owned source String, including a CMake 3.31 preamble and trailing newlines.
///
/// # Panics
/// If formatting into the String reports an error (the standard String writer does not ordinarily fail).
///
/// # Examples
/// Local test-module context:
/// ```text
/// let text = source(2);
/// assert!(text.contains("set(V1 value)"));
/// ```
fn source(pairs: usize) -> String {
    use std::fmt::Write;
    let mut source = String::from("cmake_minimum_required(VERSION 3.31)\n");
    for index in 0..pairs {
        writeln!(
            source,
            "set(V{index} value)\nmessage(\"${{V{index}}} ${{V{index}}}\")"
        )
        .unwrap();
    }
    source
}
/// Sort timing samples and print one BENCH JSON record on stdout; no file/process I/O.
///
/// # Arguments
/// - `case` `[in]`: benchmark case label included in JSON.
/// - `samples` `[in,out]`: consumed millisecond durations, sorted in place; caller storage is not retained.
///
/// # Returns
/// `()` after printing samples, median (upper middle), and p95 at integer index (len-1)*95/100.
///
/// # Panics
/// If samples is empty (indexing/length subtraction), or stdout printing fails.
///
/// # Examples
/// Local test-module context:
/// ```text
/// report("example", vec![1.0, 2.0, 3.0]);
/// ```
fn report(case: &str, mut samples: Vec<f64>) {
    samples.sort_by(f64::total_cmp);
    println!(
        "BENCH {}",
        serde_json::json!({"case":case,"unit":"ms","samples":samples,"median":samples[samples.len()/2],"p95":samples[(samples.len()-1)*95/100]})
    );
}
/// Run fresh lexer, command parser, and semantic analysis, retaining the result through black_box.
/// Does not run project commands; without a declared version semantic analysis may probe cmake --version.
/// Benchmark callers use version-pinned input to avoid that probe.
///
/// # Arguments
/// - `source` `[in]`: borrowed CMake text for all analysis layers, unchanged.
///
/// # Returns
/// `()`; the semantic result is measured then discarded, not validated here.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let text = source(2);
/// full(&text).await;
/// ```
async fn full(source: &str) {
    let parsed = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    black_box(
        Semanticer::new(parsed, Default::default())
            .parse(source, FileId { id: 0 })
            .await,
    );
}
/// Measure full analysis after warmups and print a case-specific BENCH record.
/// Allocates deterministic source and sample vectors; no files or project execution.
///
/// # Arguments
/// - `pairs` `[in]`: set/message pair count used by source and the output case label.
///
/// # Returns
/// `()` after reporting SAMPLES millisecond durations, excluding WARMUPS runs.
///
/// # Panics
/// If printing the result record fails through report.
///
/// # Algorithm
/// Build source once, time WARMUPS+SAMPLES fresh analyses, retain only measured runs, then report sorted samples.
///
/// # Flow
/// ```text
/// [source(pairs)] --> [full analysis loop]
///                            |        ^
///                            |        | next run
///                            +--------+
///                            | measured only
///                            v
///                       [report timings]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// full_samples(300).await;
/// ```
async fn full_samples(pairs: usize) {
    let source = source(pairs);
    let mut samples = Vec::with_capacity(SAMPLES);
    for iteration in 0..WARMUPS + SAMPLES {
        let start = Instant::now();
        full(&source).await;
        if iteration >= WARMUPS {
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
    }
    report(&format!("full_{pairs}_pairs"), samples);
}
/// Temporary four-module source tree for cold and warm-edit Workspace measurements.
/// [`Self::new`] writes the tree; [`Self::cold`] creates and flushes an independent [`Workspace`].
/// Two root texts differ only in a literal value so edits can reuse dependency visits.
/// There is no Drop cleanup; reference_analysis_cost removes directory on normal completion.
/// No CMake project code is executed.
///
/// # Flow
/// ```text
/// [directory] ---------> [root file]
///      |                     |
///      v                     v
/// [four modules] <-- [one / two overlays]
///      |                     |
///      +----------+----------+
///                 v
///           [Workspace::flush]
/// ```
///
/// # Examples
/// Local asynchronous benchmark context:
/// ```text
/// let fixture = Fixture::new().await;
/// let workspace = fixture.cold().await;
/// tokio::fs::remove_dir_all(&fixture.directory).await.unwrap();
/// ```
struct Fixture {
    /// Temporary tree named with the process ID; removed explicitly by the benchmark. Example: `tokio::fs::remove_dir_all(&fixture.directory).await` cleans it.
    directory: PathBuf,
    /// CMakeLists.txt entry path used by cold and overlay updates. Example: `workspace.set_overlay(&fixture.root, fixture.two.as_str())` selects edited text.
    root: PathBuf,
    /// Initial root text with EDIT one and four includes; also written to disk. Example: `fixture.one.contains("EDIT one")` reads the baseline marker.
    one: String,
    /// Alternate overlay text with EDIT two and unchanged include paths. Example: `fixture.two.contains("EDIT two")` reads the replacement marker.
    two: String,
}
impl Fixture {
    /// Create/overwrite a process-qualified temporary tree with four 300-pair modules and an EDIT one root.
    /// Also prepare an EDIT two overlay in memory; no project code is executed.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Fixture with directory/root paths and both root texts; no automatic cleanup guard.
    ///
    /// # Panics
    /// If creating the directory or writing any module/root file fails.
    ///
    /// # Algorithm
    /// Write MODULES version-pinned modules, assemble includes, derive the alternate literal value, then write the baseline root.
    ///
    /// # Flow
    /// ```text
    /// [temporary directory] --> [module write loop]
    ///        |                         |
    ///        v                         v
    /// [root + EDIT one] ----------> [include list]
    ///        |
    ///        +----> [EDIT two in memory]
    ///        |
    ///        v
    /// [write root file]
    /// ```
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// let fixture = Fixture::new().await;
    /// ```
    async fn new() -> Self {
        let directory =
            std::env::temp_dir().join(format!("cmake-reference-bench-{}", std::process::id()));
        tokio::fs::create_dir_all(&directory).await.unwrap();
        let root = directory.join("CMakeLists.txt");
        let mut one = String::from("cmake_minimum_required(VERSION 3.31)\nset(EDIT one)\n");
        for module in 0..MODULES {
            one.push_str(&format!("include(module{module}.cmake)\n"));
            tokio::fs::write(
                directory.join(format!("module{module}.cmake")),
                source(MODULE_PAIRS),
            )
            .await
            .unwrap();
        }
        let two = one.replace("EDIT one", "EDIT two");
        tokio::fs::write(&root, &one).await.unwrap();
        Self {
            directory,
            root,
            one,
            two,
        }
    }
    /// Create and flush a fresh Workspace from the saved fixture tree and black_box its graph.
    /// Reads fixture files and creates analysis workers; never configures the project.
    ///
    /// # Arguments
    /// - `self` `[in]`: saved paths/texts, unchanged; the new Workspace owns its own analysis state.
    ///
    /// # Returns
    /// Flushed Workspace retaining the root graph and snapshots for later overlay edits.
    ///
    /// # Panics
    /// If Workspace construction, flush, or root graph lookup fails.
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// let workspace = fixture.cold().await;
    /// ```
    async fn cold(&self) -> Workspace {
        let mut workspace = Workspace::new(&self.root, None).unwrap();
        workspace.flush().await.unwrap();
        black_box(workspace.root_graph().unwrap());
        workspace
    }
}
#[tokio::test]
#[ignore = "release benchmark; run alone with --ignored --nocapture"]
/// Measure version-pinned full analysis plus cold and warm-edit Workspace costs in release mode.
/// Writes temporary fixture files, prints JSON timings, and removes the directory on normal completion.
/// No CMake project code is executed; the test is ignored unless explicitly selected.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` after all benchmark cases and reuse assertions complete.
///
/// # Panics
/// In debug mode, on filesystem/setup/analysis errors, or if warm edits rebuild the graph or fail to reuse MODULES+1 visits.
///
/// # Algorithm
/// Measure 300/3000-pair fresh analyses; warm up and sample cold workspaces; alternate EDIT two/one overlays on one warm workspace, checking graph reuse each time, then remove the fixture.
///
/// # Flow
/// ```text
/// [full 300 / 3000]       [temporary 4-module tree]
///        |                          |
///        v                          v
/// [report full samples]      [cold sample loop]
///                                   |
///                                   v
/// [remove tree] <---------- [warm overlay loop]
///                                   |
///                                   v
///                          [reuse checks / report]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// reference_analysis_cost(); // ignored release test, selected through the harness
/// ```
async fn reference_analysis_cost() {
    assert!(!cfg!(debug_assertions), "Use cargo test --release");
    full_samples(300).await;
    full_samples(3000).await;
    let fixture = Fixture::new().await;
    let mut cold = Vec::with_capacity(SAMPLES);
    for iteration in 0..WARMUPS + SAMPLES {
        let start = Instant::now();
        let workspace = fixture.cold().await;
        if iteration >= WARMUPS {
            cold.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        drop(workspace);
    }
    report("workspace_cold_4x300_pairs", cold);
    let mut workspace = fixture.cold().await;
    let mut edits = Vec::with_capacity(SAMPLES);
    for iteration in 0..WARMUPS + SAMPLES {
        let text = if iteration % 2 == 0 {
            &fixture.two
        } else {
            &fixture.one
        };
        let start = Instant::now();
        workspace.set_overlay(&fixture.root, text.as_str()).unwrap();
        workspace.flush().await.unwrap();
        if iteration >= WARMUPS {
            edits.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        assert!(!workspace.graph_rebuilt);
        assert_eq!(workspace.reused_invocations, MODULES + 1);
        black_box(workspace.root_graph().unwrap());
    }
    report("workspace_warm_edit_4x300_pairs", edits);
    tokio::fs::remove_dir_all(fixture.directory).await.unwrap();
}

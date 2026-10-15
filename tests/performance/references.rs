// AI-generated release benchmarks; no machine-specific acceptance thresholds.
use cmake_analyzer::{
    parser::{
        lexer::model::{Lexer, RawBuffer},
        nparser::model::NParser,
        semanticer::model::{FileId, Semanticer},
    },
    workspace::model::Workspace,
};
use std::{hint::black_box, path::PathBuf, time::Instant};

static SAMPLES: usize = 21;
static WARMUPS: usize = 3;
static MODULES: usize = 4;
static MODULE_PAIRS: usize = 300;

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
fn report(case: &str, mut samples: Vec<f64>) {
    samples.sort_by(f64::total_cmp);
    println!(
        "BENCH {}",
        serde_json::json!({"case":case,"unit":"ms","samples":samples,"median":samples[samples.len()/2],"p95":samples[(samples.len()-1)*95/100]})
    );
}
async fn full(source: &str) {
    let parsed = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    black_box(
        Semanticer::new(parsed, Default::default())
            .parse(source, FileId { id: 0 })
            .await,
    );
}
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
struct Fixture {
    directory: PathBuf,
    root: PathBuf,
    one: String,
    two: String,
}
impl Fixture {
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
    async fn cold(&self) -> Workspace {
        let mut workspace = Workspace::new(&self.root, None).unwrap();
        workspace.flush().await.unwrap();
        black_box(workspace.root_graph().unwrap());
        workspace
    }
}
#[tokio::test]
#[ignore = "release benchmark; run alone with --ignored --nocapture"]
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

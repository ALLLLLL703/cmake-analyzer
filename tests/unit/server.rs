// AI-generated private server adapter tests.
use super::{
    roots,
    text::{TextIndex, apply_changes},
    uri,
};
use lspally::server::model::*;

#[test]
fn positions_handle_unicode_crlf_and_reject_split_surrogates() {
    let text = "🌟你好\r\nnext\n";
    for (encoding, width) in [
        (CoordinateEncoding::Utf8, 10),
        (CoordinateEncoding::Utf16, 4),
        (CoordinateEncoding::Utf32, 3),
    ] {
        let index = TextIndex::new(text, encoding);
        let position = index.position(10).unwrap();
        assert_eq!((position.line, position.character), (0, width));
        assert_eq!(index.position(12).unwrap().line, 1);
    }
    let bad = TextDocumentContentChangeEvent {
        range: Some(Range {
            start: Position {
                line: 0,
                character: 1,
            },
            end: Position {
                line: 0,
                character: 2,
            },
        }),
        range_length: None,
        text: "x".into(),
    };
    assert!(apply_changes(text, vec![bad], CoordinateEncoding::Utf16).is_err());
}
#[test]
fn byte_and_coordinate_mapping_roundtrips_at_every_character_boundary() {
    let source = "é🌟你好\r\nnext𐐷\rfinal\n";
    for encoding in [
        CoordinateEncoding::Utf8,
        CoordinateEncoding::Utf16,
        CoordinateEncoding::Utf32,
    ] {
        let index = TextIndex::new(source, encoding);
        for (byte, ch) in source.char_indices() {
            if ch == '\n' && byte > 0 && source.as_bytes()[byte - 1] == b'\r' {
                continue;
            }
            let range = Range {
                start: index.position(byte).unwrap(),
                end: index.position(byte).unwrap(),
            };
            assert_eq!(index.edit_range(&range).unwrap(), byte..byte);
        }
        assert_eq!(
            index.position(source.find("final").unwrap()).unwrap().line,
            2
        );
    }
}
#[test]
fn changes_are_sequential_and_allow_full_replacement() {
    let changes = vec![
        TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "🌟abc\n".into(),
        },
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 0,
                    character: 2,
                },
                end: Position {
                    line: 0,
                    character: 3,
                },
            }),
            range_length: None,
            text: "X".into(),
        },
    ];
    assert_eq!(
        apply_changes("old", changes, CoordinateEncoding::Utf16).unwrap(),
        "🌟Xbc\n"
    );
}
#[cfg(unix)]
#[test]
fn file_uris_decode_spaces_unicode_and_reject_remote_authorities() {
    assert_eq!(
        uri::to_path(&Uri("file:///tmp/hello%20world/%E4%BD%A0.cmake".into())).unwrap(),
        std::path::PathBuf::from("/tmp/hello world/你.cmake")
    );
    for value in [
        "https://example.com/a.cmake",
        "file://otherhost/tmp/a.cmake",
        "file:///tmp/%00.cmake",
        "file:relative.cmake",
    ] {
        assert!(uri::to_path(&Uri(value.into())).is_err());
    }
}
#[tokio::test]
async fn root_discovery_uses_nearest_entry_or_none_for_loose_files() {
    let base = std::env::temp_dir().join(format!("cmake-root-test-{}", std::process::id()));
    tokio::fs::create_dir_all(base.join("child/modules"))
        .await
        .unwrap();
    let loose = base.join("loose.cmake");
    assert!(roots::find_project(&loose).await.unwrap().is_none());
    tokio::fs::write(base.join("CMakeLists.txt"), "")
        .await
        .unwrap();
    tokio::fs::write(base.join("child/CMakeLists.txt"), "")
        .await
        .unwrap();
    let module = base.join("child/modules/foo.cmake");
    assert_eq!(
        roots::find_project(&module).await.unwrap().unwrap(),
        base.join("child/CMakeLists.txt")
    );
    assert!(
        roots::explicit_root(&base.join("child/modules"))
            .await
            .is_err()
    );
    tokio::fs::remove_dir_all(base).await.unwrap();
}

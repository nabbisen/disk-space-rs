use std::env;

use super::*;

#[test]
fn disk_space_new_ok() {
    // OSの一時ディレクトリ（必ず存在するはずのパス）でテスト
    let temp_path = env::temp_dir();
    let result = DiskSpace::new(&temp_path);

    assert!(result.is_ok(), "DiskSpace::new failed: {:?}", result.err());

    let space = result.unwrap();

    // 容量が取得できていることを確認 (0バイトということは通常ありえない)
    assert!(space.total > 0, "Total space should be greater than 0");
    assert!(
        space.available > 0,
        "Available space should be greater than 0"
    );

    // 空き容量が全容量を超えることはないはず
    assert!(
        space.available <= space.total,
        "Available space exceeds total space"
    );
}

#[test]
fn disk_space_invalid_path_err() {
    // 存在しないパスを指定した場合のエラーハンドリング
    let invalid_path = Path::new("/path/that/does/not/exist/12345");
    let result = DiskSpace::new(invalid_path);

    // WindowsとUnixで返ってくるエラーコードは異なる可能性があるが、Errであることを期待
    assert!(result.is_err(), "Should fail for non-existent path");
}

#[test]
fn unit_conversions() {
    // ダミーデータで単位変換ロジックをテスト
    let space = DiskSpace {
        available: 1024 * 1024 * 500,  // 500 MB
        total: 1024 * 1024 * 1024 * 2, // 2 GB
    };

    let mb = space.as_mb();
    assert_eq!(((10.0 * mb.available).round()) / 10.0, 500.0);
    assert_eq!(((10.0 * mb.total).round()) / 10.0, 2048.0);

    let gb = space.as_gb();
    assert_eq!(((10.0 * gb.available).round()) / 10.0, 0.5);
    assert_eq!(((10.0 * gb.total).round()) / 10.0, 2.0);
}

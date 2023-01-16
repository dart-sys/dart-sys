// Copyright 2021 Xayn AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A small rust script for updating this library.
//!
//! Usage: `cargo run -p update-lib -- <dart-version>`

use std::{
    env::{self, set_current_dir},
    fs::{self, read_to_string},
    io::ErrorKind,
    path::{Path, PathBuf},
    process::{exit, Command},
};

use semver::{BuildMetadata, Version};
use toml_edit::{Document, Formatted, Item, Value};

fn dart_version() -> String {
    let mut args = env::args();
    args.next().expect("bin name missing");
    args.next()
        .filter(|arg1| !arg1.starts_with('-') && args.next().is_none())
        .unwrap_or_else(|| {
            eprintln!("USAGE: update-lib <dart-version>");
            exit(1)
        })
}

fn main() {
    let dart_version = &dart_version();
    let workspace_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    set_current_dir(workspace_path).unwrap();

    let dart_src = &workspace_path.join("dart-src");

    remove_dir_all(dart_src);
    download_dart_src(dart_version, dart_src);

    if !has_dart_source_changed(dart_src) {
        eprintln!("Dart source didn't change.");
        return;
    }

    let (dl_major, dl_minor) = extract_dl_api_version(dart_src);

    update_crate_version(
        dl_major,
        dl_minor,
        &workspace_path.join("../Cargo.toml"),
    );
}

fn update_crate_version(dl_major: u64, dl_minor: u64, path: &Path) {
    let (mut manifest, version) = parse_manifest(path);
    let version = bump_version(dl_major, dl_minor, &version);
    eprintln!("Bumped version of {} to {}", path.display(), version);
    manifest["workspace"]["package"]["version"] =
        Item::Value(Value::String(Formatted::new(version.to_string())));
    fs::write(path, manifest.to_string())
        .unwrap_or_else(|err| panic!("Failed to write Manifest: {}\n{}", path.display(), err));
}

fn bump_version(dl_major: u64, dl_minor: u64, version: &Version) -> Version {
    let old_dl_version: Version = version
        .build
        .as_str()
        .parse()
        .expect("Failed to parse build version");
    let (major, minor) = if old_dl_version.major < dl_major {
        (version.major + 1, 0)
    } else if old_dl_version.minor <= dl_minor {
        (version.major, version.minor + 1)
    } else {
        eprintln!("WARNING: DOWNGRADING");
        (version.major + 1, 0)
    };

    let mut new_version = Version::new(major, minor, 0);
    new_version.build =
        BuildMetadata::new(&Version::new(dl_major, dl_minor, 0).to_string()).unwrap();
    new_version
}

fn parse_manifest(path: &Path) -> (Document, Version) {
    let manifest: Document = read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read manifest: {}\n{}", path.display(), err))
        .parse()
        .unwrap_or_else(|err| panic!("Failed to parse manifest: {}\n{}", path.display(), err));

    let version: Version = manifest["workspace"]["package"]["version"]
        .as_str()
        .unwrap_or_else(|| panic!("Failed find version in: {}", path.display()))
        .parse()
        .unwrap_or_else(|err| panic!("Failed to parse version: {}\n{}", path.display(), err));

    (manifest, version)
}

fn extract_dl_api_version(dart_src: &Path) -> (u64, u64) {
    let version_file =
        fs::read_to_string(dart_src.join("dart_version.h")).expect("version file can't be read");

    let mut minor = None;
    let mut major = None;
    const MAJOR_LINE: &str = "#define DART_API_DL_MAJOR_VERSION ";
    const MINOR_LINE: &str = "#define DART_API_DL_MINOR_VERSION ";
    for line in version_file.lines() {
        let (slot, end) = if let Some(major_str) = line.strip_prefix(MAJOR_LINE) {
            (&mut major, major_str)
        } else if let Some(minor_str) = line.strip_prefix(MINOR_LINE) {
            (&mut minor, minor_str)
        } else {
            continue;
        };

        if slot.is_some() {
            panic!("version defined multiple times: {:?}, {}", slot, line);
        }

        *slot = Some(end.trim().parse().expect("malformed version"));
    }

    (
        major.expect("can't find major version"),
        minor.expect("can't find minor version"),
    )
}

fn has_dart_source_changed(dart_src: &Path) -> bool {
    let out = command_output(
        Command::new("git")
            .args(["status", "-s", "--"])
            .arg(dart_src),
    );

    out.lines().filter(|l| !l.trim().is_empty()).count() > 0
}

fn command_output(cmd: &mut Command) -> String {
    let output = cmd
        .output()
        .unwrap_or_else(|err| panic!("failed to spawn command: {:?}\n{}", cmd, err));

    if !output.status.success() {
        panic!(
            "failed to run cmd {:?}: {}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).expect("non-utf8 in command output")
}

fn download_dart_src(dart_version: &str, out_dir: &Path) {
    eprintln!("Downloading dart version: {:?}", dart_version);
    let git_out_dir = temp_dir();
    //TODO use `git sparse-checkout` instead.
    let ec = Command::new("git")
        .args(["clone", "--depth", "1", "--branch"])
        .arg(dart_version)
        .args(["--", "https://github.com/dart-lang/sdk.git"])
        .arg(&git_out_dir)
        .output()
        .unwrap();

    if !ec.status.success() {
        panic!(
            "failed to fetch dart source: {}",
            String::from_utf8_lossy(&ec.stderr)
        );
    }

    create_dir(out_dir);
    copy_all_in(&git_out_dir.join("runtime/include"), out_dir, &["c", "h"]);
    copy_file(&git_out_dir.join("LICENSE"), &out_dir.join("LICENSE"));

    remove_dir_all(&git_out_dir);
}

fn copy_all_in(src_dir: &Path, dest_dir: &Path, extensions: &[&str]) {
    for src_entry in src_dir
        .read_dir()
        .unwrap_or_else(|e| panic!("Copying files failed: {}\n{}", src_dir.display(), e))
    {
        let src_entry = src_entry.unwrap();
        let src_type = src_entry.file_type().unwrap();
        let src_path = &src_entry.path();
        let dest_path = &dest_dir.join(src_entry.file_name());
        if src_type.is_dir() {
            create_dir(dest_path);
            copy_all_in(src_path, dest_path, extensions);
        } else if src_type.is_file() && check_extension(src_path, extensions) {
            copy_file(src_path, dest_path);
        }
    }
}

fn check_extension(path: &Path, extensions: &[&str]) -> bool {
    path.extension().map_or(false, |extension| {
        let extension = extension.to_str().unwrap();
        extensions.contains(&extension)
    })
}

fn temp_dir() -> PathBuf {
    let out = Command::new("mktemp")
        .args(["-d", "-t", "dart-api-dl-codegen.XXXX"])
        .output()
        .unwrap();
    if !out.status.success() {
        panic!("Creating temp dir failed.");
    }
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim())
}

fn create_dir(name: &Path) {
    if !name.is_dir() {
        fs::create_dir(name)
            .unwrap_or_else(|e| panic!("Failed to create dir: {}\n{}", name.display(), e));
    }
}

fn copy_file(from: &Path, to: &Path) {
    fs::copy(from, to).unwrap_or_else(|e| {
        panic!(
            "Failed to copy file from {} to {}\n{}",
            from.display(),
            to.display(),
            e
        )
    });
}

fn remove_dir_all(dir: &Path) {
    if let Err(err) = fs::remove_dir_all(dir) {
        if err.kind() != ErrorKind::NotFound {
            panic!("Failed to remove dir: {}\n{}", dir.display(), err);
        }
    }
}

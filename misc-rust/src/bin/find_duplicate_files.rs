use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use itertools::Itertools;

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry) -> io::Result<()>) -> Vec<io::Error> {
    return _visit_dirs(dir, cb, vec![]);
}

fn _visit_dirs(
    dir: &Path,
    cb: &mut dyn FnMut(&DirEntry) -> io::Result<()>,
    mut errors: Vec<io::Error>,
) -> Vec<io::Error> {
    if dir.is_dir() {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                errors.push(err);
                return errors;
            }
        };
        let mut entries = entries
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(err) => {
                    errors.push(err);
                    None
                }
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let path = entry.path();
            if path.is_symlink() {
                continue;
            } else if path.is_dir() {
                errors = _visit_dirs(&path, cb, errors);
            } else {
                let Err(err) = cb(&entry) else { continue };
                errors.push(err);
            }
        }
    }
    return errors;
}

fn get_name_len_and_hash(path: PathBuf) -> io::Result<(String, u64, u32)> {
    let name = path
        .as_os_str()
        .to_str()
        .expect("Non-unicode shenanigans")
        .to_string();
    let sum = crc32fast::hash(&fs::read(&path)?);
    // results.insert(name, );
    return Ok((name, path.metadata()?.len(), sum));
}

// TODO: make a recovery state
fn main() {
    let dirs = vec![Path::new("...")];

    for dir in dirs.iter() {
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    let mut total_results = HashMap::new();
    for (i, dir_path) in dirs.into_iter().enumerate() {
        println!("{:?}", dir_path);

        let results = Arc::new(Mutex::new((
            Vec::<(String, u64, u32)>::new(),
            Vec::<String>::new(),
        )));
        std::thread::scope(|s| {
            let paths = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
            let finished = Arc::new(AtomicBool::new(false));

            for _ in 0..4 {
                let paths = Arc::clone(&paths);
                let results = Arc::clone(&results);
                let finished = Arc::clone(&finished);
                s.spawn(move || loop {
                    let mut batch = vec![];
                    {
                        batch.extend((*paths.lock().unwrap()).drain(..))
                    }

                    if batch.len() == 0 && finished.load(std::sync::atomic::Ordering::Relaxed) {
                        return;
                    } else {
                        sleep(Duration::from_millis(100));
                    }

                    let mut good = vec![];
                    let errors = batch
                        .into_iter()
                        .filter_map(|entry| match get_name_len_and_hash(entry) {
                            Ok(stats) => {
                                good.push(stats);
                                None
                            }
                            Err(err) => Some(format!("{:?}", err)),
                        })
                        .collect_vec();

                    {
                        let mut lock = results.lock().unwrap();
                        lock.0.extend(good.into_iter());
                        lock.1.extend(errors.into_iter());
                    }
                });
            }

            let mut batch_files = vec![];
            let errors = visit_dirs(dir_path, &mut |file| {
                let file = file.path().canonicalize()?;

                batch_files.push(file);
                if batch_files.len() > 32 {
                    let mut lock = paths.lock().unwrap();
                    lock.extend(batch_files.drain(..));
                }
                return Ok(());
            })
            .into_iter()
            .map(|err| format!("{:?}", err))
            .collect_vec();

            finished.store(true, std::sync::atomic::Ordering::Relaxed);

            let mut lock = results.lock().unwrap();
            lock.1.extend(errors);
        });

        let (results, errors) = results.lock().unwrap().clone();

        let output = serde_json::to_string(&errors).unwrap();
        fs::write(Path::new(&format!("./outputs/errors_{}.json", i)), output).unwrap();

        for (name, len, hash) in results {
            total_results.insert(name, (len, hash));
        }
    }

    let mut inverse = HashMap::new();

    for (name, stats) in total_results {
        inverse.entry(stats).or_insert_with(|| vec![]).push(name);
    }

    let mut final_results = vec![];
    for (stats, names) in inverse.into_iter() {
        if names.len() > 1 {
            final_results.push((stats, names))
        }
    }

    let output = serde_json::to_string(&final_results).unwrap();
    fs::write(Path::new("./outputs/output.json"), output).unwrap();
}

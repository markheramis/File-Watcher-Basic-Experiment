use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use notify::recommended_watcher;
use notify::Result;
use notify::Watcher;
use notify::RecursiveMode;
use notify::RecommendedWatcher;
use notify::Error;
use notify::EventKind;
use notify::Event;
use notify::event::EventAttributes;
fn main() -> Result<()> {
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let r: Arc<AtomicBool> = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    let path: &Path = Path::new("C:\\WatchDir");
    if !path.exists() {
        eprintln!("Path {:?} does not exists", path.to_str().unwrap());
        exit(1);
    }
    let mut watcher: RecommendedWatcher = recommended_watcher(handle_watcher_events)?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_secs(1));
    }
    Ok(())
}
fn handle_watcher_events(result: Result<Event>) {
    match result {
        Ok(e) => handle_fs_event(e),
        Err(e) => handle_fs_error(e),
    }
}
fn handle_fs_event(e: Event) {
    let path_strings: Vec<String> = e.paths
        .iter()
        .map(path_buf_to_str)
        .collect();

    let event_attributes: EventAttributes = e.attrs;
    path_strings.iter().for_each(|path| {
        println!("Path: {}", path)
    });
    match e.kind {
        EventKind::Create(_) => {
            println!("Kind: is a write");
        },
        EventKind::Modify(_) => {
            println!("Kind: is a modify");
        },
        EventKind::Remove(_) => {
            println!("Kind: is a remove");
        },
        _ => {
            println!("Kind: unknown");
        }
    }
    println!("Attr: {:?}", event_attributes);
}
fn handle_fs_error(e: Error) {
    println!("watch error: {:?}", e)
}
fn path_buf_to_str(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}
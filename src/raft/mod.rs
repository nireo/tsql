use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Seek;
use std::sync::Mutex;
use std::{fs::File, path::PathBuf, time::Instant};
use tarpc::client;

trait StateMachine {
    fn apply(&self, data: Vec<u8>) -> Result<Vec<u8>>;
}

struct ApplyResult {
    res: Vec<u8>,
}

struct Entry {
    cmd: Vec<u8>,
    term: u64,
}

struct ClusterMember {
    id: u64,
    address: String,
    next_idx: u64,
    match_idx: u64,
    voted_for: u64,
}

enum NodeState {
    Leader,
    Follower,
    Candidate,
}

struct State {
    curr_term: u64,
    log: Vec<Entry>,
    id: u64,
    address: String,
    election_timeout: Instant,
    state_machine: Box<dyn StateMachine>,
    metadata_dir: PathBuf,
    fd: File,
    commit_idx: u64,
    last_applied: u64,
    state: NodeState,
    cluster: Vec<ClusterMember>,
}

pub struct Server {
    state: Mutex<State>,
}

const PAGE_SIZE: usize = 4096;
const ENTRY_HEADER: usize = 16;
const ENTRY_SIZE: usize = 128;

impl Server {
    fn persist(&mut self, write_log: bool, num_new_entries: usize) -> Result<()> {
        let t = Instant::now();

        let mut state = self.state.lock().unwrap();
        let num_new_entries = if num_new_entries == 0 && write_log {
            state.log.len()
        } else {
            num_new_entries
        };

        state.fd.seek(std::io::SeekFrom::Start(0))?;
        let mut page = [0u8; PAGE_SIZE];

        (&mut page[0..8]).write_u64::<LittleEndian>(state.curr_term)?;
        (&mut page[8..16]).write_u64::<LittleEndian>(state)

        Ok(())
    }
}

use chrono::prelude::*;
use std::{
    collections::HashSet,
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    thread,
    time::Duration,
};

use crate::sonyflake::{decompose, to_sonyflake_time, Sonyflake, BIT_LEN_SEQUENCE};

#[test]
fn test_once() {
    let machine_id = 1;
    let sf = Sonyflake::new(machine_id, Utc::now());

    let sleep_time = 50;
    thread::sleep(Duration::from_millis(10 * sleep_time));

    let id = sf.next_id();
    let parts = decompose(id);

    let actual_msb = parts.msb;
    assert_eq!(0, actual_msb, "Unexpected msb");

    let actual_time = parts.time;
    if actual_time < sleep_time || actual_time > sleep_time + 1 {
        panic!("Unexpected time {}", actual_time)
    }

    let actual_machine_id = parts.machine_id;
    assert_eq!(
        machine_id as u64, actual_machine_id,
        "Unexpected machine id"
    );
}

#[test]
fn test_run_for_10s() {
    let now = Utc::now();
    let start_time = to_sonyflake_time(now);
    let machine_id = 1;
    let sf = Sonyflake::new(machine_id, now);

    let mut last_id: u64 = 0;
    let mut max_sequence: u64 = 0;

    let initial = to_sonyflake_time(Utc::now());
    let mut current = initial;
    while current - initial < 1000 {
        let id = sf.next_id();
        let parts = decompose(id);

        if id <= last_id {
            panic!("duplicated id (id: {}, last_id: {})", id, last_id);
        }
        last_id = id;

        current = to_sonyflake_time(Utc::now());

        let actual_msb = parts.msb;
        if actual_msb != 0 {
            panic!("unexpected msb: {}", actual_msb);
        }

        let actual_time = parts.time as i64;
        let overtime = start_time + actual_time - current;
        if overtime > 0 {
            panic!("unexpected overtime: {}", overtime)
        }

        let actual_sequence = parts.sequence;
        if max_sequence < actual_sequence {
            max_sequence = actual_sequence;
        }

        let actual_machine_id = parts.machine_id;
        if actual_machine_id != machine_id as u64 {
            panic!("unexpected machine id: {}", actual_machine_id)
        }
    }

    assert_eq!(
        max_sequence,
        (1 << BIT_LEN_SEQUENCE) - 1,
        "unexpected max sequence"
    );
}

#[test]
fn test_threads() {
    let machine_id = 1;
    let sf = Sonyflake::new(machine_id, Utc::now());

    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();

    let mut children = Vec::new();
    for _ in 0..10 {
        let thread_sf = sf.clone();
        let thread_tx = tx.clone();
        children.push(thread::spawn(move || {
            for _ in 0..1000 {
                thread_tx.send(thread_sf.next_id()).unwrap();
            }
        }));
    }

    let mut ids = HashSet::new();
    for _ in 0..10_000 {
        let id = rx.recv_timeout(Duration::from_millis(100)).unwrap();
        assert!(!ids.contains(&id), "duplicate id: {}", id);
        ids.insert(id);
    }

    for child in children {
        child.join().expect("Child thread panicked");
    }
}

#[test]
fn test_generate_10_ids() {
    let sf = Sonyflake::new(1, Utc::now());
    let mut ids = vec![];
    for _ in 0..10 {
        let id = sf.next_id();
        if ids.iter().any(|vec_id| *vec_id == id) {
            panic!("duplicated id: {}", id)
        }
        ids.push(id);
    }
}

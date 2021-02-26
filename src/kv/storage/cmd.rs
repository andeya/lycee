use std::{mem, time};
use std::io::{Error, ErrorKind, Result};

use crate::kv::storage::crc64::{as_ne_bytes, kv_crc64};
use crate::kv::storage::kvdb::kvdb_s;

fn usage() {
    println!("{}{}{}{}{}{}{}{}", "    kv help                   -- this message \n",
             "    kv get <key>              -- get a key\n",
             "    kv put <key> <val>        -- set key\n",
             "    kv del <key>              -- delete a key\n",
             "    kv list                   -- list all key in the db\n",
             "    kv ins <start_key> <num>  -- insert records in batch mode\n",
             "    kv clr                    -- remove all records in the database\n",
             "    kv verify                 -- get all records and verify them\n");
}

struct cmd_s {
    cmd: &'static str,
    func: fn(db: &mut kvdb_s, args: Vec<String>) -> Result<()>,
}

const cmds: [cmd_s; 8] = [
    cmd_s { cmd: "get", func: fn_get },
    cmd_s { cmd: "put", func: fn_put },
    cmd_s { cmd: "del", func: fn_del },
    cmd_s { cmd: "list", func: fn_list },
    cmd_s { cmd: "dump", func: fn_dump },
    cmd_s { cmd: "ins", func: fn_ins },
    cmd_s { cmd: "clr", func: fn_clr },
    cmd_s { cmd: "verify", func: fn_verify },
];

fn args_err(error: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, error)
}

fn assert_args(args: &Vec<String>, count: usize) -> Result<()> {
    if args.len() != count {
        return Result::Err(args_err(format!("number of args must be {}", count).as_str()))
    }
    Ok(())
}

fn parse_u64(args: &Vec<String>, index: usize) -> Result<u64> {
    args[index].parse().map_err(|e|
        args_err(format!("type of the {} args must be u64", index).as_str()))
}

fn fn_get(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 3)?;
    let k: u64 = parse_u64(&args, 2)?;
    match db.get(k) {
        Ok(v) =>
            println!("found, key = {}, value = {}", k, v),
        Err(e) =>
            eprintln!("record not found: {}", e),
    }
    return Ok(());
}

fn fn_put(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 4)?;
    let k: u64 = parse_u64(&args, 2)?;
    let v: u64 = parse_u64(&args, 3)?;
    db.put(k, v)
}

fn fn_del(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 3)?;
    let k: u64 = parse_u64(&args, 2)?;
    match db.del(k) {
        Ok(_) =>
            println!("deletion success"),
        Err(e) =>
            eprintln!("deletion failed: {}", e),
    }
    Ok(())
}

fn fn_list(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 2)?;
    let k: u64 = parse_u64(&args, 2)?;
    for (k, v) in db.iter(0, u64::MAX)? {
        println!("k = {:>5}, v = {:>21}", k, v);
    }
    Ok(())
}

fn fn_dump(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 2)?;
    db.dump()
}


fn fn_ins(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    assert_args(&args, 4)?;
    let start_k = parse_u64(&args, 2)?;
    let n = parse_u64(&args, 3)?;
    let mut last_i = 0_u64;
    let t0 = time::Instant::now();
    let mut last = t0.clone();
    let mut seq;
    for i in 0..n {
        seq = start_k + i;
        let k = kv_crc64(as_ne_bytes(&seq));
        let v = kv_crc64(as_ne_bytes(&k));
        db.put(k, v)?;
        if (i % 100) == 0 {
            let now = time::Instant::now();
            if (now - last).as_secs() >= 1 {
                let us0 = 1000000 * (now - last);
                let us1 = 1000000 * (now - t0);
                println!("total: {} in {} sec, avarage: {} us/record",
                         i, (now - t0).as_secs(), us1.as_secs() / i);
                println!("last {} sec: {}, avarage: {} us/record",
                         i - last_i, (now - last).as_secs(), us0.as_secs() / (i - last_i + 1));
                last = now;
                last_i = i;
            }
        }
    }
    Ok(())
}

fn fn_clr(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    Ok(())
}

fn fn_verify(db: &mut kvdb_s, args: Vec<String>) -> Result<()> {
    Ok(())
}


fn exec(args: Vec<String>) -> Result<()> {
// struct cmd_s *c;
    if args.len() < 2 {
        usage();
        return Ok(());
    }
    let mut db = kvdb_s::open("aaa.db")?;
    let mut found = false;
    for c in &cmds {
        if c.cmd == args[1] {
            (c.func)(&mut db, args);
            found = true;
            break;
        }
    }
    if !found {
        usage();
    }
    return Ok(());
}


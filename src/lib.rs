pub mod kv;

pub mod proto {
    pub mod coprocessor {
        tonic::include_proto!("coprocessor");
    }

    pub mod eraftpb {
        tonic::include_proto!("eraftpb");
    }

    pub mod errorpb {
        tonic::include_proto!("errorpb");
    }

    pub mod helloworld {
        tonic::include_proto!("helloworld");
    }

    pub mod kvrpcpb {
        tonic::include_proto!("kvrpcpb");
    }

    pub mod metapb {
        tonic::include_proto!("metapb");
    }

    pub mod raft_cmdpb {
        tonic::include_proto!("raft_cmdpb");
    }

    pub mod raft_serverpb {
        tonic::include_proto!("raft_serverpb");
    }

    pub mod schedulerpb {
        tonic::include_proto!("schedulerpb");
    }

    pub mod tinykvpb {
        tonic::include_proto!("tinykvpb");
    }
}


pub fn catch_backtrace(skip: usize, max_depth: usize) -> backtrace::Backtrace {
    let mut backtrace = backtrace::Backtrace::new_unresolved();
    let frames = backtrace.frames();
    const INIT_START: usize = 5;
    let mut start = INIT_START + skip;
    if start > frames.len() { start = frames.len() }
    let mut end = start + max_depth;
    if end > frames.len() { end = frames.len() }
    backtrace = backtrace::Backtrace::from(frames[start..end].to_vec());
    backtrace.resolve();
    backtrace
}


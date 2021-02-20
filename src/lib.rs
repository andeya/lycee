use std::ffi::c_void;
use std::fmt;
use std::fmt::Display;
use std::path::Path;

use backtrace::{Backtrace, BacktraceSymbol, SymbolName};

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


pub fn catch_backtrace(skip: usize, max_depth: usize) -> Backtrace {
    let mut backtrace = Backtrace::new_unresolved();
    let frames = backtrace.frames();
    const INIT_START: usize = 5;
    let mut start = INIT_START + skip;
    if start > frames.len() { start = frames.len() }
    let mut end = start + max_depth;
    if end > frames.len() { end = frames.len() }
    backtrace = Backtrace::from(frames[start..end].to_vec());
    backtrace.resolve();
    backtrace
}

pub struct TraceSymbol {
    symbol: Option<BacktraceSymbol>,
}

impl TraceSymbol {
    pub fn is_none(&self) -> bool {
        self.symbol.is_none()
    }

    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn name(&self) -> Option<SymbolName<'_>> {
        if let Some(symbol) = &self.symbol {
            symbol.name()
        } else { None }
    }

    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn addr(&self) -> Option<*mut c_void> {
        if let Some(symbol) = &self.symbol {
            symbol.addr()
        } else { None }
    }

    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn filename(&self) -> Option<&Path> {
        if let Some(symbol) = &self.symbol {
            symbol.filename()
        } else { None }
    }

    pub fn short_filename(&self) -> String {
        if let Some(filename) = self.filename() {
            let cwd = std::env::current_dir();
            if let Ok(cwd) = &cwd {
                if let Ok(suffix) = filename.strip_prefix(cwd) {
                    return format!("{}", suffix.display())
                }
            }
        }
        "".to_string()
    }

    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn lineno(&self) -> Option<u32> {
        if let Some(symbol) = &self.symbol {
            symbol.lineno()
        } else { None }
    }

    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn colno(&self) -> Option<u32> {
        if let Some(symbol) = &self.symbol {
            symbol.colno()
        } else { None }
    }

    /// {filepath}:(lineno):(colno)
    pub fn fileline(&self) -> String {
        if let Some(lineno) = self.lineno() {
            return format!("{}:{:?}:{:?}", self.short_filename(), lineno, self.colno().unwrap());
        }
        "".to_string()
    }

    pub fn fn_name(&self) -> String {
        if let Some(name) = self.name() {
            format!("{:#}", name)
        } else {
            "".to_string()
        }
    }
}

impl Display for TraceSymbol {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            return Ok(())
        }
        fmt.write_str(self.fn_name().as_str())?;
        fmt.write_str("\n   at ")?;
        fmt.write_str(self.fileline().as_str())?;
        Ok(())
    }
}

pub fn innermost_symbol(b: &Backtrace) -> TraceSymbol {
    let frames = b.frames();
    if frames.len() == 0 {
        return TraceSymbol { symbol: None }
    }
    let symbols = frames[0].symbols();
    if symbols.len() == 0 {
        return TraceSymbol { symbol: None }
    }
    TraceSymbol { symbol: Some(symbols[0].clone()) }
}

pub fn catch_symbol(skip: usize) -> TraceSymbol {
    innermost_symbol(&catch_backtrace(skip, 1))
}

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use rsomics_common::{run, CommonFlags, RsomicsError, ToolMeta};

use rsomics_voterank::voterank_from_edge_list;

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

/// VoteRank influential-node ranking of an undirected graph
/// (`networkx.voterank`, Zhang et al. 2016).
///
/// Reads an edge list (`u v` per line; `#` comments and blank lines skipped;
/// string node names; parallel edges deduplicated, self-loops kept).
/// Output is the ordered list of elected seeds, one node label per line, most
/// influential first. Only positively-voted nodes are returned.
#[derive(Parser, Debug)]
#[command(name = "rsomics-voterank", version, about, long_about = None)]
pub struct Cli {
    /// Edge list; `-` or omitted reads stdin.
    #[arg(value_name = "EDGES")]
    pub edges: Option<PathBuf>,

    /// Number of ranked seeds to extract (default: all).
    #[arg(long, value_name = "N")]
    pub number_of_nodes: Option<usize>,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || {
            let mut input = String::new();
            match &self.edges {
                Some(p) if p.as_os_str() != "-" => {
                    File::open(p)
                        .map_err(RsomicsError::Io)?
                        .read_to_string(&mut input)
                        .map_err(RsomicsError::Io)?;
                }
                _ => {
                    io::stdin()
                        .lock()
                        .read_to_string(&mut input)
                        .map_err(RsomicsError::Io)?;
                }
            }
            let seeds = voterank_from_edge_list(&input, self.number_of_nodes);
            if !common.json {
                for label in &seeds {
                    println!("{label}");
                }
            }
            Ok(seeds)
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        super::Cli::command().debug_assert();
    }
}

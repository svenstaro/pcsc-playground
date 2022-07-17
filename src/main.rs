use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
// use log::info;

static SMARTCARD_LIST: &'static str = include_str!(concat!(env!("OUT_DIR"), "/smartcard_list.txt"));

#[derive(Parser)]
#[clap(name = "pcsc", author, about, version)]
pub enum Args {
    /// List detected readers
    List,

    /// Read current card once
    Read,

    /// Continuously read until canceled
    Scan,
}

fn list() -> Result<()> {
    let ctx = pcsc::Context::establish(pcsc::Scope::User).context("failed to establish context")?;
    let readers = ctx.list_readers_owned().context("Failed to list readers")?;
    for (i, name) in readers.iter().enumerate() {
        println!("Device No. {} - {}", i, name.to_str()?);
    }
    Ok(())
}

fn read() -> Result<()> {
    let ctx = pcsc::Context::establish(pcsc::Scope::User).context("failed to establish context")?;
    let reader = &ctx
        .list_readers_owned()
        .context("No readers are connected")?[0];
    println!("Using reader: {}", reader.to_str()?);
    let card = match ctx.connect(&reader, pcsc::ShareMode::Shared, pcsc::Protocols::ANY) {
        Ok(card) => card,
        Err(pcsc::Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };
    let status = card.status2_owned()?;
    let protocol = status.protocol2();
    println!(
        "Protocol: {}",
        protocol
            .map(|x| format!("{:?}", x))
            .unwrap_or("No protocol".to_string())
    );
    println!("Status: {:?}", status.status());
    println!(
        "{:?}",
        card.get_attribute_owned(pcsc::Attribute::VendorName)
            .context("failed to get vendor IFD version attribute")?
    );
    let atr = status.atr();

    let lines = SMARTCARD_LIST.lines();
    for (i, line) in lines.enumerate() {
        // If the line doesn't start with a whitespace, it's a regex that we have to check.
        if !line.starts_with('\t') {
            let regex = Regex::new(line)?;
            // If there's a match, go down the file until there's an empty line.
            if regex.is_match(line) {
                let current_index = i;
                let mut rofl = vec![];
                while !lines[i].is_empty() {
                    rofl.push(lines[i]);
                }
                dbg!(rofl);
            }
        }
    }
    Ok(())
}

fn scan() -> Result<()> {
    // let ctx = pcsc::Context::establish(pcsc::Scope::User).context("failed to establish context")?;
    // let reader = &ctx
    //     .list_readers_owned()
    //     .context("No readers are connected")?[0];
    // let mut reader_states = vec![
    //     // Listen for reader insertions/removals, if supported.
    //     pcsc::ReaderState::new(pcsc::PNP_NOTIFICATION(), pcsc::State::UNAWARE),
    // ];
    // loop {
    //     // Remove dead readers.
    //     for rs in &reader_states {
    //         if rs
    //             .event_state()
    //             .intersects(pcsc::State::UNKNOWN | pcsc::State::IGNORE)
    //         {
    //             info!("Removing reader {:?}", rs.name());
    //         }
    //     }
    //     // reader_states.retain(|rs| !is_dead(rs));
    //
    //     // Add new readers.
    //     let names = ctx.list_readers_owned().context("failed to list readers")?;
    //     for name in names {
    //         if !reader_states.iter().any(|rs| rs.name() == name.as_c_str()) {
    //             info!("Found reader {:?}", name);
    //             reader_states.push(pcsc::ReaderState::new(name, pcsc::State::UNAWARE));
    //         }
    //     }
    //
    //     // Update the view of the state to wait on.
    //     for rs in &mut reader_states {
    //         rs.sync_current_state();
    //     }
    //
    //     // Wait until the state changes.
    //     ctx.get_status_change(None, &mut reader_states)
    //         .context("failed to get status change")?;
    //
    //     // Print current state.
    //     println!();
    //     for rs in &reader_states {
    //         if rs.event_state().contains(pcsc::State::PRESENT) {
    //             info!("Detected card");
    //         }
    //     }
    // }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::List => list()?,
        Args::Read => read()?,
        Args::Scan => scan()?,
    }
    Ok(())
}

#[macro_use] extern crate serde_derive;
use crates_io_api::{SyncClient,AsyncClient};
use futures::prelude::*;
use std::collections::HashMap;

#[derive(Serialize,Deserialize)]
struct LicenseSnapshot {
    date: chrono::DateTime<chrono::Utc>,
    counts: HashMap<String, u64>,
}

fn load_all_full_crates(rt: &mut tokio::runtime::Runtime) -> failure::Fallible<Vec<Option<crates_io_api::FullCrate>>> {
    let cl = SyncClient::new();
    let asycl = AsyncClient::new();

    if std::fs::metadata("all-full-crates.json").is_err() {
        println!("Downloading all FullCrate from crates.io...");
        let all_crates = if std::fs::metadata("all-crates.json").is_err() {
            let all_crates = cl.all_crates(None).unwrap();
            serde_json::to_writer(std::fs::File::create("all-crates.json")?, &all_crates)?;
            all_crates
        } else {
            serde_json::from_reader(std::fs::File::open("all-crates.json")?)?
        };

        let all_full_crates_fut = futures::stream::iter_ok(all_crates.into_iter().map(move |x| {
            asycl.full_crate(&x.name, true).then(move |v| { match v {
                Ok(v) => {println!("fetched {}, {} versions", x.name, v.versions.len()); futures::future::ok::<_, ()>(Some(v))}
                Err(e) => {
                    eprintln!("failed to full_crate {}: {:?}", x.name, e);
                    futures::future::ok(None)
                }
            }})
        })).buffered(100).collect();

        let all_full_crates = rt.block_on(all_full_crates_fut).expect("failed to do a thing!");

        serde_json::to_writer(std::fs::File::create("all-full-crates.json")?, &all_full_crates)?;
        Ok(all_full_crates)
    } else {
        Ok(serde_json::from_reader(std::fs::File::open("all-full-crates.json")?)?)
    }
}

fn load_snapshots(rt: &mut tokio::runtime::Runtime) -> failure::Fallible<Vec<LicenseSnapshot>> {
    let all_full_crates = load_all_full_crates(rt)?;
    if std::fs::metadata("license-snapshots.json").is_err() {
        println!("Computing license snapshots...");

        let mut all_versions_and_licenses = vec ![];

        for krate in all_full_crates {
            match krate {
                Some(krate) =>
                for vers in &krate.versions {
                    all_versions_and_licenses.push((vers.created_at.clone(), vers.license.clone(), krate.id.clone()))
                },
                None => ()
            }
        }

        all_versions_and_licenses.sort_by(|a, b| a.0.cmp(&b.0));

        let mut latest_licenses = HashMap::new();

        let mut snapshots = vec![];

        fn snapshot_from_latest(date: chrono::DateTime<chrono::Utc>, latest: &HashMap<String, Option<String>>) -> LicenseSnapshot {

            let mut counts = HashMap::new();
            for (_, license) in latest.iter() {
                *counts.entry(license.clone().unwrap_or_else(|| "NO-LICENSE-RECORDED-OH-NO".into())).or_insert(0) += 1;
            }
            LicenseSnapshot {
                date: date,
                counts: counts
            }
        }

        for (date, license, id) in all_versions_and_licenses {
            match latest_licenses.insert(id.clone(), license.clone()) {
                Some(l2) => {
                    if l2 != license {
                        println!("{} changed license!", id)
                    }
                },
                None => ()
            }
            snapshots.push(snapshot_from_latest(date, &latest_licenses));
        }

        serde_json::to_writer(std::fs::File::create("license-snapshots.json")?, &snapshots)?;
        Ok(snapshots)
    } else {
        Ok(serde_json::from_reader(std::fs::File::open("license-snapshots.json")?)?)
    }
}

fn resampled_snapshots(rt: &mut tokio::runtime::Runtime) -> failure::Fallible<Vec<LicenseSnapshot>> {
    if std::fs::metadata("filtered-snapshots").is_err() {
        let all_snapshots = load_snapshots(rt)?;
        let mut selected_snapshots = vec![];
        let mut next_week_deadline = all_snapshots[0].date;
        for snap in all_snapshots {
            if snap.date >= next_week_deadline {
                selected_snapshots.push(snap);
                next_week_deadline = next_week_deadline.checked_add_signed(chrono::Duration::weeks(1)).expect("a crate was published within a week of the end of time? ominous.");
            }
        }
        serde_json::to_writer(std::fs::File::create("selected-snapshots.json")?, &selected_snapshots)?;
        Ok(selected_snapshots)
    } else {
        Ok(serde_json::from_reader(std::fs::File::open("selected-snapshots.json")?)?)
    }
}

fn main() -> failure::Fallible<()> {
    env_logger::init();
    let mut rt = ::tokio::runtime::Runtime::new().unwrap();

    let final_snapshots = resampled_snapshots(&mut rt)?;

    println!("Computed all {} license snapshots!", final_snapshots.len());
    Ok(())
}

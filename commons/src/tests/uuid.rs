use std::collections::HashSet;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

use crate::database::types::{DBUuid, DBUuidType};

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

#[test]
pub fn test_collision_for_size_12() {
    let times = 10usize;
    for i in 0..times {
        println!("Executing {}", i);

        let _guard = RUNTIME.enter();
        futures::executor::block_on(async move {
            let cpu = 4;
            let limit = 10_000;

            let mut jobs = Vec::with_capacity(cpu);

            for _ in 0..cpu {
                jobs.push(tokio::spawn(async move {
                    let mut set = HashSet::with_capacity(limit);

                    for _ in 0..limit {
                        let code = DBUuid::new(DBUuidType::DBKey);

                        if set.contains(&code) {
                            return Err(format!(
                                "Collision with: {} at {}",
                                code,
                                code.date().unwrap().0
                            ));
                        }

                        set.insert(code);
                    }

                    Ok(set)
                }));
            }

            let mut results = Vec::with_capacity(cpu);

            for job in jobs {
                results.push(job.await.unwrap().expect("Error during set creation"));
            }

            for i in 0..results.len() {
                let a = results.get(i).unwrap();

                for j in (i + 1)..results.len() {
                    let b = results.get(j).unwrap();

                    if a.intersection(b).next().is_some() {
                        panic!("Collision between sets");
                    }
                }
            }
        });
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// This test is used to generate a set of Uuids for external use.
#[test]
pub fn test_generate_set() {
    let _guard = RUNTIME.enter();
    futures::executor::block_on(async move {
        for _ in 0..10 {
            println!("{}", DBUuid::new(DBUuidType::DBKey));
            sleep(Duration::from_millis(10)).await;
        }
    });
}

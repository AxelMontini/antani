use sqlx::PgPool;

// async fn occupancy(_client: &State<Client>, db: &State<PgPool>, trainNr: i32, date: String) -> anyhow::Result<()> {
//     let stops = stops(_client, db, trainNr).await;
//     let abbrevs = stops.stops.iter().map(|e| abbrev(db, e.to_string()));
//     let capacity

//     let capacity: Option<i32> = Some(0); // SELECT max(capacity) FROM dataset WHERE connectionDate=date and trainNr=trainNumber;
//     let stopsAmount = stops.len();
//     let occupancy = vec![0; stopsAmount - 1];
//     // O(n^3)
//     // specchio riflesso buttati nel cesso + ratio + based
//     for i in 0..stopsAmount {
//         for j in (i + 1)..stopsAmount {
//             for ()
//             occupancy[]
//         }
//     }
//     Ok(())
// }
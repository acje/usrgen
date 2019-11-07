use chrono::NaiveDate;
use postgres::{Connection, TlsMode};
#[allow(dead_code)]

struct UsrMessage {
    id: i32,
    message: String,
    slaktedato: NaiveDate,
    efta: i32,
    skrottnr: i64,
    posteringsside: String,
    duplicate: bool,
}

fn main() {
    let conn =
        Connection::connect("postgres://postgres:rpg@localhost:5432", TlsMode::None).unwrap();
        
    conn.execute("DROP TABLE usrmessage", &[]).unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usrmessage (
                    id              SERIAL PRIMARY KEY,
                    message         VARCHAR NOT NULL,
                    slaktedato      DATE NOT NULL,
                    efta            INTEGER NOT NULL,
                    skrottnr        BIGINT NOT NULL,
                    posteringsside  VARCHAR NOT NULL,
                    duplicate       BOOL DEFAULT FALSE
                  )",
        &[],
    )
    .unwrap();

    let message = UsrMessage {
        id: 0,
        message: "abc".to_string(),
        slaktedato: NaiveDate::from_ymd(2015, 3, 14),
        efta: 140,
        skrottnr: 12345678,
        posteringsside: "debet".to_string(),
        duplicate: false,
    };

    conn.execute("INSERT INTO usrmessage (message, slaktedato, efta, skrottnr, posteringsside, duplicate) VALUES ($1, $2, $3, $4, $5, $6)",
                 &[&message.message, &message.slaktedato, &message.efta, &message.skrottnr, &message.posteringsside, &message.duplicate]).unwrap();

    let message = UsrMessage {
        id: 0,
        message: "abc".to_string(),
        slaktedato: NaiveDate::from_ymd(2016, 3, 14),
        efta: 140,
        skrottnr: 12345678,
        posteringsside: "kredit".to_string(),
        duplicate: false,
    };

    conn.execute("INSERT INTO usrmessage (message, slaktedato, efta, skrottnr, posteringsside, duplicate) VALUES ($1, $2, $3, $4, $5, $6)",
                 &[&message.message, &message.slaktedato, &message.efta, &message.skrottnr, &message.posteringsside, &message.duplicate]).unwrap();


   let message = UsrMessage {
        id: 0,
        message: "abc".to_string(),
        slaktedato: NaiveDate::from_ymd(2016, 3, 14),
        efta: 140,
        skrottnr: 12345678,
        posteringsside: "debet".to_string(),
        duplicate: false,
    };

    conn.execute("INSERT INTO usrmessage (message, slaktedato, efta, skrottnr, posteringsside, duplicate) VALUES ($1, $2, $3, $4, $5, $6)",
                 &[&message.message, &message.slaktedato, &message.efta, &message.skrottnr, &message.posteringsside, &message.duplicate]).unwrap();

  let message = UsrMessage {
        id: 0,
        message: "abc".to_string(),
        slaktedato: NaiveDate::from_ymd(2016, 3, 14),
        efta: 140,
        skrottnr: 12345678,
        posteringsside: "debet".to_string(),
        duplicate: false,
    };

    conn.execute("INSERT INTO usrmessage (message, slaktedato, efta, skrottnr, posteringsside, duplicate) VALUES ($1, $2, $3, $4, $5, $6)",
                 &[&message.message, &message.slaktedato, &message.efta, &message.skrottnr, &message.posteringsside, &message.duplicate]).unwrap();

}
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
   
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user_preferences (
                id INTEGER PRIMARY KEY,
                user_id TEXT NOT NULL,
                zipcode TEXT NOT NULL,
                general_aqi_threshold INTEGER,
                o3_threshold INTEGER,
                pm25_threshold INTEGER,
                pm10_threshold INTEGER,
                UNIQUE(user_id)
            )",
            params![],
        )?;
        Ok(())
    }

    pub fn set_user_preference(
        &self,
        user_id: &str,
        zipcode: &str,
        general_aqi_threshold: i32,
        o3_threshold: i32,
        pm25_threshold: i32,
        pm10_threshold: i32,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO user_preferences 
            (user_id, zipcode, general_aqi_threshold, o3_threshold, pm25_threshold, pm10_threshold) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![user_id, zipcode, general_aqi_threshold, o3_threshold, pm25_threshold, pm10_threshold],
        )?;
        Ok(())
    }

    pub fn get_user_preference(&self, user_id: &str) -> Result<Option<(String, i32, i32, i32, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT zipcode, general_aqi_threshold, o3_threshold, pm25_threshold, pm10_threshold 
            FROM user_preferences WHERE user_id = ?1",
        )?;
        let mut rows = stmt.query(params![user_id])?;

        if let Some(row) = rows.next()? {
            let zipcode: String = row.get(0)?;
            let general_aqi_threshold: i32 = row.get(1)?;
            let o3_threshold: i32 = row.get(2)?;
            let pm25_threshold: i32 = row.get(3)?;
            let pm10_threshold: i32 = row.get(4)?;
            return Ok(Some((zipcode, general_aqi_threshold, o3_threshold, pm25_threshold, pm10_threshold)));
        }
        Ok(None)
    }
}

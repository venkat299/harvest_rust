// CREATE TABLE eod(
//    SYMBOL      VARCHAR(10) NOT NULL PRIMARY KEY
//   ,SERIES      VARCHAR(2) NOT NULL
//   ,OPEN        NUMERIC(7,2) NOT NULL
//   ,HIGH        NUMERIC(8,2) NOT NULL
//   ,LOW         NUMERIC(7,2) NOT NULL
//   ,CLOSE       NUMERIC(8,2) NOT NULL
//   ,LAST        NUMERIC(7,2) NOT NULL
//   ,PREVCLOSE   NUMERIC(8,2) NOT NULL
//   ,TOTTRDQTY   INTEGER  NOT NULL
//   ,TOTTRDVAL   NUMERIC(11,2) NOT NULL
//   ,TIMESTAMP   VARCHAR(11) NOT NULL
//   ,TOTALTRADES INTEGER  NOT NULL
//   ,ISIN        VARCHAR(12) NOT NULL
// );

#[derive(Debug)]
pub struct Eod {
    pub symbol: String,
    pub series: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub last: f64,
    pub prevclose: f64,
    pub tottrdqty: i64,
    pub tottrdval: f64,
    pub timestamp: String,
    pub totaltrades: i64,
    pub isin: String, /* id: i32,
                       * name: String,
                       * time_created: Timespec,
                       * data: Option<Vec<u8>> */
}

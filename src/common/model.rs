use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_SIZE: u64 = 20;

pub struct Page {
    offset: u64,
    size: u64,
}

impl Page {
    pub fn new(page: Option<u64>, size: Option<u64>) -> Self {
        let page = page.unwrap_or(DEFAULT_PAGE);
        let size = size.unwrap_or(DEFAULT_SIZE);
        let page = if page <= 0 { DEFAULT_PAGE } else { page };
        let size = if size <= 0 { DEFAULT_SIZE } else { size };
        Self {
            offset: (page - 1) * size,
            size,
        }
    }

    pub fn from(page: u64, size: u64) -> Self {
        let size = if size == 0 { DEFAULT_SIZE } else { size };
        Self {
            offset: (page - 1) * size,
            size,
        }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Pager<T> {
    pub count: String,
    pub data: Vec<T>,
}

impl<T: Serialize> Pager<T> {
    pub fn new(data: Vec<T>, count: u64) -> Pager<T> {
        let count = count.to_string();
        Self { data, count }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ExposePager<T> {
    pub count: u64,
    pub data: Vec<T>,
}

impl<T: Serialize> ExposePager<T> {
    pub fn new(data: Vec<T>, count: u64) -> ExposePager<T> {
        Self { data, count }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginIpLimit {
    pub count: u32,
    pub last_time: NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IdReq {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdsReq {
    pub id: Vec<Uuid>,
}
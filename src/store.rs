use crate::env::{MAXSIZE, TIMEOUT};
use crate::post::Post;
use chrono::Local;
use log::info;
use std::collections::BTreeMap;

pub struct PostStore {
    posts_map: BTreeMap<i64, String>,
    total_size: usize,
    last_post_time: i64,
}

impl PostStore {
    pub fn new() -> Self {
        Self {
            posts_map: BTreeMap::new(),
            total_size: 0,
            last_post_time: 0,
        }
    }

    pub fn find_by_time(&self, time: i64) -> Option<&String> {
        self.posts_map.get(&time)
    }

    pub fn save(&mut self, post: Post) -> Option<i64> {
        let time = Local::now().timestamp_millis();
        if time <= self.last_post_time {
            return None;
        }

        let post_json = serde_json::to_string(&post).ok()?;

        self.total_size += post_json.len();
        self.last_post_time = time;
        self.posts_map.insert(time, post_json);

        let delta = self.size() - MAXSIZE.clone();
        if delta > 0 {
            let mut tot: usize = 0;
            let mut keys = Vec::new();
            for (&t, s) in &self.posts_map {
                tot += s.len();
                keys.push(t);
                if tot > delta {
                    break;
                }
            }
            for t in &keys {
                self.total_size -= self.posts_map.remove(t).unwrap().len();
            }
        }

        Some(self.last_post_time)
    }

    pub fn clean(&mut self) {
        let limit = Local::now().timestamp_millis() - TIMEOUT.clone();
        let mut keys = Vec::new();
        for &t in self.posts_map.keys() {
            if t < limit {
                keys.push(t)
            } else {
                break;
            }
        }
        for t in &keys {
            self.total_size -= self.posts_map.remove(t).unwrap().len();
        }

        info!("store size = {}", self.size());
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.total_size
    }
}

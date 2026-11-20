use crate::content::Topic;
use crate::storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STORAGE_KEY: &str = "educa-brasil-progress";
const XP_PER_CORRECT: u32 = 10;
const XP_PER_LESSON: u32 = 5;
const XP_PER_LEVEL: u32 = 100;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct TopicProgress {
    pub lesson_read: bool,
    pub best_score: u8,
    pub attempts: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub topics: HashMap<String, TopicProgress>,
}

impl Progress {
    pub fn load() -> Self {
        storage::load(STORAGE_KEY)
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    fn save(&self) {
        if let Ok(raw) = serde_json::to_string(self) {
            storage::save(STORAGE_KEY, &raw);
        }
    }

    pub fn get(&self, id: &str) -> TopicProgress {
        self.topics.get(id).copied().unwrap_or_default()
    }

    pub fn mark_lesson(&mut self, id: &str) {
        let entry = self.topics.entry(id.to_string()).or_default();
        if !entry.lesson_read {
            entry.lesson_read = true;
            self.save();
        }
    }

    pub fn record_quiz(&mut self, id: &str, score: u8) {
        let entry = self.topics.entry(id.to_string()).or_default();
        entry.attempts += 1;
        entry.best_score = entry.best_score.max(score);
        self.save();
    }

    pub fn reset(&mut self) {
        self.topics.clear();
        self.save();
    }

    pub fn stars(&self, topic: &Topic) -> u8 {
        let total = topic.questions.len();
        if total == 0 {
            return 0;
        }
        ((self.get(&topic.id).best_score as usize * 3) / total).min(3) as u8
    }

    pub fn completed(&self, topic: &Topic) -> bool {
        self.stars(topic) > 0
    }

    pub fn completion(&self, topics: &[&Topic]) -> f32 {
        if topics.is_empty() {
            return 0.0;
        }
        topics.iter().filter(|topic| self.completed(topic)).count() as f32 / topics.len() as f32
    }

    pub fn xp(&self) -> u32 {
        self.topics
            .values()
            .map(|entry| {
                entry.best_score as u32 * XP_PER_CORRECT
                    + if entry.lesson_read { XP_PER_LESSON } else { 0 }
            })
            .sum()
    }

    pub fn level(&self) -> u32 {
        self.xp() / XP_PER_LEVEL + 1
    }

    pub fn level_fraction(&self) -> f32 {
        (self.xp() % XP_PER_LEVEL) as f32 / XP_PER_LEVEL as f32
    }

    pub fn total_stars(&self, topics: &[Topic]) -> u32 {
        topics.iter().map(|topic| self.stars(topic) as u32).sum()
    }
}

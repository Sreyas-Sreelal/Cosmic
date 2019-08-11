use crate::ai::AI;

use serenity::{
    client::bridge::voice::ClientVoiceManager, model::id::GuildId, prelude::*, voice::LockedAudio,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

pub struct VoiceManager;
impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct AIStore;
impl TypeMapKey for AIStore {
    type Value = Arc<Mutex<AI>>;
}

pub struct PlayListStore;
impl TypeMapKey for PlayListStore {
    type Value = Arc<Mutex<HashMap<GuildId, VecDeque<LockedAudio>>>>;
}

use crate::ai::AI;

use serenity::{client::bridge::voice::ClientVoiceManager, prelude::*};
use std::sync::Arc;

pub struct VoiceManager;
impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct AIStore;
impl TypeMapKey for AIStore {
    type Value = Arc<Mutex<AI>>;
}

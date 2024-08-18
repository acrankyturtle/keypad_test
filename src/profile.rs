extern crate alloc;
extern crate serde_json_core;

use alloc::string::String;
use alloc::vec::Vec;

use crate::TagList;

pub struct KeyboardProfile {
	pub keys: Vec<DeviceKey>,
}

pub struct DeviceKey {
	pub key_id: KeyId,
	pub layers: Vec<TaggedDeviceKeyLayer>,
	pub default_layer: DeviceKeyLayer,
}

impl DeviceKey {
	pub fn get_active_layer(&self, tags: &TagList) -> &DeviceKeyLayer {
		match self.layers.iter().find(|layer| layer.is_match(tags)) {
			Some(layer) => &layer.layer,
			None => &self.default_layer,
		}
	}
}

pub struct TaggedDeviceKeyLayer {
	pub layer: DeviceKeyLayer,
	pub tags: Vec<LayerTag>,
	pub match_type: TagMatchType,
}

impl TaggedDeviceKeyLayer {
	fn is_match(&self, tags: &TagList) -> bool {
		match self.match_type {
			TagMatchType::All => tags.contains_all(&self.tags),
			TagMatchType::Any => tags.contains_any(&self.tags),
		}
	}
}

pub struct DeviceKeyLayer {
	pub id: LayerId,
	pub macros: Vec<Macro>,
}

pub struct Macro {
	pub id: MacroId,
	pub name: String,
	pub play_channel: Option<Channel>,
	pub cut_channels: Vec<Channel>,
	pub start_sequence: Sequence,
	pub loop_sequence: Sequence,
	pub end_sequence: Sequence,
}

pub struct Sequence {
	pub actions: Vec<Action>,
}

pub struct Action {
	pub predelay_ms: u32,
	pub action_event: ActionEvent,
}

pub enum ActionEvent {
	None,
	Keyboard(KeyboardEvent),
	Mouse(MouseEvent),
	Layer(LayerEvent),
}

pub enum TagMatchType {
	All,
	Any,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerId(i128);

impl LayerId {
	pub fn new(id: i128) -> Self {
		LayerId(id)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MacroId(i128);

impl MacroId {
	pub fn new(id: i128) -> Self {
		MacroId(id)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyId(i128);

impl KeyId {
	pub fn new(id: i128) -> Self {
		KeyId(id)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Channel(i128);

impl Channel {
	pub fn new(id: i128) -> Self {
		Channel(id)
	}
}

#[derive(Debug, PartialEq)]

pub struct LayerTag(String);

impl LayerTag {
	pub fn new(tag: String) -> Self {
		LayerTag(tag)
	}
}

pub enum KeyboardEvent {
	KeyDown(KeyboardKey),
	KeyUp(KeyboardKey),
}

pub enum KeyboardKey {
	A,
	B,
	C,
}

pub enum MouseEvent {
	ButtonDown(MouseButton),
	ButtonUp(MouseButton),
	ScrollUp(i32),
	ScrollDown(i32),
	ScrollLeft(i32),
	ScrollRight(i32),
	Move(i32, i32),
}

pub enum MouseButton {
	Left,
	Right,
	Middle,
	Back,
	Forward,
}

pub enum LayerEvent {
	Clear(LayerTag),
	Set(LayerTag),
}

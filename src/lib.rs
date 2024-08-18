#![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate serde_json_core;

use alloc::vec::Vec;
use profile::LayerTag;

pub mod profile;
pub mod state;

pub struct TagList {
	internal: Vec<LayerTag>,
	external: Vec<LayerTag>,
}

impl TagList {
	pub fn new() -> Self {
		TagList {
			internal: Vec::new(),
			external: Vec::new(),
		}
	}

	pub fn add_internal(&mut self, tag: LayerTag) {
		self.internal.push(tag);
	}

	pub fn add_many_internal(&mut self, tags: Vec<LayerTag>) {
		self.internal.extend(tags);
	}

	pub fn remove_internal(&mut self, tag: LayerTag) {
		if let Some(index) = self.internal.iter().position(|t| *t == tag) {
			self.internal.remove(index);
		}
	}

	pub fn remove_many_internal(&mut self, tags: Vec<LayerTag>) {
		for tag in tags {
			self.remove_internal(tag);
		}
	}

	pub fn clear_internal(&mut self) {
		self.internal.clear();
	}

	pub fn set_external(&mut self, tags: Vec<LayerTag>) {
		self.external = tags;
	}

	pub fn contains_all(&self, tags: &Vec<LayerTag>) -> bool {
		tags.iter()
			.all(|tag| self.internal.contains(tag) || self.external.contains(tag))
	}

	pub fn contains_any(&self, tags: &Vec<LayerTag>) -> bool {
		tags.iter()
			.any(|tag| self.internal.contains(tag) || self.external.contains(tag))
	}
}

#[cfg(test)]
mod tests {}

//! Particles (`pulsar.particles`): the plugin's root crate, the one place the
//! plugin registers everything it contributes.
//!
//! The plugin's other crates hold the code: `helio-pass-corona` (the
//! feature: Corona's GPU simulation, its overlay drawer and the join that
//! places emitters and hands out the shared particle pool),
//! `particles_component` (the SceneDB emitter component and its row). None
//! of them registers a contribution; this crate imports them and registers them through its
//! [`build`], so this file and the module below are the whole list of
//! what the plugin brings:
//!
//! - [`runtime`]: what a game links. The component class, the Helio feature,
//!   the scene rows and the animated content (live emitters keep a realtime
//!   viewport drawing).
//! - The editor (with the `editor` feature) registers nothing: the inspector
//!   edits an emitter through its reflected properties.
//!
//! Every registration names this crate as its origin
//! (`p.origin()`), so the plugin gate skips all of them while
//! a project disables the plugin, and the plugin catalog lists them under
//! `pulsar.particles`. Other plugins never depend on this crate; hosts reach
//! it through the installed set or a generated game, a game without the
//! `editor` feature.

// The plugin's descriptor (`pulsar-plugin.toml`): linking this crate installs
// the plugin; a project enables it or not (`pulsar_plugins`).
pulsar_plugins::installed!("pulsar-plugin.toml", build);

/// Everything the plugin registers.
pub fn build(p: &mut pulsar_plugins::PluginBuilder) {
    runtime::build(p);
}

pub use particles_component;

pub mod runtime;

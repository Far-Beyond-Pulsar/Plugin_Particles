//! What the particles plugin registers for a game (and the editor): every
//! runtime contribution, grouped by the engine seam it plugs into.

use engine_backend::scene::helio_bridge::SceneRowsRegistration;
use engine_backend::subsystems::render::{AnimatedContentRegistration, RenderFeatureRegistration};
use particles_component::ParticleEmitterComponent;

// ── Component classes ───────────────────────────────────────────────────────

pulsar_world_registry::submit_world_component!(ParticleEmitterComponent);

// ── Rendering ───────────────────────────────────────────────────────────────

// Corona's simulation and its overlay drawer.
inventory::submit! {
    RenderFeatureRegistration {
        origin: pulsar_plugins::origin!(),
        features: particles_component::render_features,
        configure: |config| config,
    }
}

// The emitters' source rows and the join that places them at their owners
// with their ranges of the shared particle pool.
inventory::submit! {
    SceneRowsRegistration {
        origin: pulsar_plugins::origin!(),
        name: "particles.rows",
        register_columns: particles_component::register_columns,
        derivation: particles_component::derivation,
    }
}

// Particles move on their own: a realtime viewport keeps drawing while an
// enabled emitter is in the world.
inventory::submit! {
    AnimatedContentRegistration {
        origin: pulsar_plugins::origin!(),
        name: "particles.live_emitters",
        watch: particles_component::watch_particles,
        animated: particles_component::particles_live,
    }
}

#[cfg(test)]
mod tests {
    use engine_backend::subsystems::render::helio_renderer::render_features;

    #[test]
    fn registers_its_feature() {
        let features = render_features::plugin_features();
        assert!(
            features.ids().contains(&"helio.corona"),
            "features: {:?}",
            features.ids()
        );
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(
            ours.iter()
                .any(|summary| summary.category == "component.class")
        );
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.particles"),
                "{summary:?}"
            );
        }
    }
}

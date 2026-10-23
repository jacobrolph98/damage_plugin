#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::*;
    use crate::messages::*;
    use crate::plugin::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(DamagePlugin);
        app
    }

    #[test]
    fn damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 5.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 5.);
    }

    #[test]
    fn fatal_hit_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 10.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let messages = app.world().resource::<Messages<FatalHitMessage>>();
        assert!(!messages.is_empty());
    }

    #[test]
    fn negative_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: -5.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn heal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: 5.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 15.);
    }

    #[test]
    fn negative_heal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: -5.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn overheal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(8.),
            MaxHealth(10.)
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: 4.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn max_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MaxDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 5.);
    }

    #[test]
    fn over_min_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MinDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 2.);
    }

    #[test]
    fn under_min_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MinDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 3.,
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn armour_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Armour {
                rating: 2.,
                formula: |damage, rating| damage * (1.0 / rating)
            },
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8., // Should get halved
        });
        app.world_mut().run_schedule(FixedUpdate);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 6.);
    }
}
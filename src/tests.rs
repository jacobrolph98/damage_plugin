#[cfg(test)]
mod tests {
    use std::time::Duration;
    use bevy::platform::collections::HashMap;
    use bevy::prelude::*;
    use crate::components::*;
    use crate::messages::*;
    use crate::plugin::*;
    use crate::types::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(DamagePlugin);
        app
    }

    fn step(app: &mut App) {
        let delta = Duration::from_millis(16);
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(delta);
        }
        app.world_mut().run_schedule(FixedUpdate);
    }

    #[test]
    fn damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 5.)])
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 10.)])
        });
        step(&mut app);
        let messages = app.world().resource::<Messages<FatalHitMessage>>();
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn negative_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, -5.)])
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictHealMessage {
            target: trgt,
            source: None,
            amount: 5.,
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictHealMessage {
            target: trgt,
            source: None,
            amount: -5.,
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictHealMessage {
            target: trgt,
            source: None,
            amount: 4.,
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 8.)])
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 8.)])
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 3.)])
        });
        step(&mut app);
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
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt,
            source: None,
            damage: Damage::new(&[(None, 8.)])
        });
        step(&mut app);
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 6.);
    }

    #[test]
    fn modifier_test() {
        let mut app = build_app();

        let f1: fn(f32) -> f32 = |dmg| dmg * 0.5;
        let trgt_1 = app.world_mut().spawn((
            Name::new("Target1"),
            DamageModifier(HashMap::from([(None, f1)])),
            Health(10.),
        )).id();
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt_1,
            source: None,
            damage: Damage::new(&[(None, 6.)])
        });

        let f2: fn(f32) -> f32 = |dmg| dmg * 2.;
        let trgt_2 = app.world_mut().spawn((
            Name::new("Target2"),
            DamageModifier(HashMap::from([(Some(0), f2)])),
            Health(10.),
        )).id();
        app.world_mut().write_message(InflictDamageMessage {
            target: trgt_2,
            source: None,
            damage: Damage::new(&[(Some(0), 4.)])
        });

        step(&mut app);
        let health_1 = app.world().get::<Health>(trgt_1).unwrap();
        assert_eq!(health_1.0, 7.);
        let health_2 = app.world().get::<Health>(trgt_2).unwrap();
        assert_eq!(health_2.0, 2.);
    }
}
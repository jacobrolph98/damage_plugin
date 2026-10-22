This library provides various components, messages & systems to handle the storage & change of health of entities. 
```
let npc_entity = commands.spawn((
    Name::new("NPC"),
    Health(100.),
    MaxHealth(100.), // Optional but recommended
)).id();

damage_msg.write(DamageMessage {
    target: npc_entity,
    source: None,
    amount: 20.,
});

heal_msg.write(HealMessage {
    target: npc_entity,
    source: None,
    amount: 10.,
});

for fatality in fatal_hits_msg.read() {
    info!("Entity has 0 health, despawning");
    commands.entity(fatality.target).despawn();
}
```
Interface with this library with the following;
### Messages
- Write DamageMessage
    - Target entity, amount, and optionally source entity
- Write HealMessage
    - Target, amount, optionally source
- Read FatalHitMessage
    - Written when an entity's health reaches 0

### Components
- Health, f32
    - Naturally, everything this plugin interacts with requires this component
- MaxHealth (optional)
    - Health will not go above this value 
- DamageMultiplier
    - Multiplies incoming damage
- HealMultiplier
    - Multiplies incoming healing
- MaxDamage
    - Incoming damage will be capped to this value
- MinDamage
    - Incoming damage less than this value will be ignored
- Armour
    - Provide an armour rating & custom formula which receives and returns an f32

Want to add damage types in the future
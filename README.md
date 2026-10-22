Interface with this library with the following;
### Messages
- Write DamageMessage
    - Target entity, amount, and optionally source entity
- Write HealMessage
    - Target, amount, optionally source
- Read FatalHitMessage
    - Written when an entity's health reaches 0

### Components
- Health
    - f32 amount
- MaxHealth (optional)
    - Health will not go above this value 
- DamageMultiplier
    - Multiplies incoming damage by f32 
- HealMultiplier
    - Multiplies incoming healing by f32 
- MaxDamage
    - Incoming damage will be capped to this value
- MinDamage
    - Incoming damage less than this value will be ignored
- Armour
    - Provide an armour rating & custom formula which receives and returns an f32

Want to add damage types in the future
pub struct DamageInstance {
    pub dam_type: Option<usize>,
    pub amount: f32
}

pub struct Damage(pub Vec<DamageInstance>);

impl Damage {
    pub fn new(dmg: &[(Option<usize>, f32)]) -> Self {
        Damage(
            dmg.iter().map(|(dam_type, amount)| DamageInstance {
                dam_type: *dam_type,
                amount: *amount
            }).collect()
        )
    }
}
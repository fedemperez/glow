use nbt::{Value, Map};

pub fn gen_default_dim() -> Value {
    let mut values = Map::new();
    values.insert("piglin_safe".into(), Value::Byte(0));
    values.insert("natural".into(), Value::Byte(1));
    values.insert("ambient_light".into(), Value::Float(0.0));
    values.insert("infiniburn".into(), Value::String("minecraft:infiniburn_overworld".into()));
    values.insert("respawn_anchor_works".into(), Value::Byte(0));
    values.insert("has_skylight".into(), Value::Byte(1));
    values.insert("bed_works".into(), Value::Byte(1));
    values.insert("effects".into(), Value::String("minecraft:overworld".into()));
    values.insert("has_raids".into(), Value::Byte(1));
    values.insert("min_y".into(), Value::Int(0));
    values.insert("height".into(), Value::Int(256));
    values.insert("logical_height".into(), Value::Int(256));
    values.insert("coordinate_scale".into(), Value::Double(1.0));
    values.insert("ultrawarm".into(), Value::Byte(0));
    values.insert("has_ceiling".into(), Value::Byte(0));
    Value::Compound(values)
}

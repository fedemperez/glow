use nbt::{Value, Map};

pub fn gen_dimension_codec() -> Value {
    let dimensions = gen_registry(
        "minecraft:dimension_type",
        gen_registry_entry("minecraft:overworld", 0, gen_default_dim()));
    let biomes = gen_registry(
        "minecraft:worldgen/biome",
        gen_registry_entry("minecraft:plains", 0, gen_default_biome()));
    let mut values = Map::new();
    values.insert("minecraft:dimension_type".into(), dimensions);
    values.insert("minecraft:worldgen/biome".into(), biomes);
    Value::Compound(values)
}

fn gen_registry(reg_type: &str, value: Value) -> Value {
    let mut values = Map::new();
    values.insert("type".into(), Value::String(reg_type.into()));
    values.insert("value".into(), Value::List(vec![value]));
    Value::Compound(values)
}

fn gen_registry_entry(name: &str, id: i32, element: Value) -> Value {
    let mut values = Map::new();
    values.insert("name".into(), Value::String(name.into()));
    values.insert("id".into(), Value::Int(id));
    values.insert("element".into(), element);
    Value::Compound(values)
}

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
    values.insert("logical_height".into(), Value::Int(256));
    values.insert("coordinate_scale".into(), Value::Double(1.0));
    values.insert("ultrawarm".into(), Value::Byte(0));
    values.insert("has_ceiling".into(), Value::Byte(0));
    Value::Compound(values)
}

fn gen_default_biome() -> Value {
    let mut values = Map::new();
    values.insert("precipitation".into(), Value::String("rain".into()));
    values.insert("depth".into(), Value::Float(1.0));
    values.insert("temperature".into(), Value::Float(1.0));
    values.insert("scale".into(), Value::Float(1.0));
    values.insert("downfall".into(), Value::Float(1.0));
    values.insert("category".into(), Value::String("forest".into()));
    let mut effects = Map::new();
    effects.insert("sky_color".into(), Value::Int(0x7FA1FF));
    effects.insert("water_fog_color".into(), Value::Int(0x7FA1FF));
    effects.insert("fog_color".into(), Value::Int(0x7FA1FF));
    effects.insert("water_color".into(), Value::Int(0x7FA1FF));
    values.insert("effects".into(), Value::Compound(effects));
    Value::Compound(values)
}
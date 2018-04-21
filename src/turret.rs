use cgmath::Vector2;
use std::rc::Weak;

#[derive(Debug, Clone)]
pub struct Turret {
    pub position: Vector2<f32>,
    pub range: f32,
    pub projectile: TurretProjectileDefinition,
}

#[derive(Debug, Clone)]
pub struct TurretProjectileDefinition {
    pub travel_speed: f32,
    pub area_of_effect: f32,
    pub linger: f32,
    pub damage: f32,
}

#[derive(Debug, Clone)]
pub struct TurretProjectile {
    pub shooter: Weak<Turret>,
    pub definition: TurretProjectileDefinition,
    pub position: Vector2<f32>,
    pub target_position: Vector2<f32>,
    pub arrival_time: Option<u64>,
}

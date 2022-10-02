use bevy::prelude::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_target);
    }
}

fn spawn_target(
    mut commands: Commands,
) {
    // spawn a square off to the right
    let x = 50.0;
    let y = 0.0;
    let radius = 10.0;
    let target = Target::new(Shape::Square, Vec2 { x, y }, radius);

    let sprite_bundle = target.create_sprite_bundle();
    commands.spawn_bundle(sprite_bundle)
        .insert(target)
        .insert(Name::new("TargetSquare"));

    // let target = Target::new(Shape::Square, Vec2 { x: -x, y }, radius);
    // let sprite_bundle = target.create_sprite_bundle();
    // commands.spawn_bundle(sprite_bundle)
    //     .insert(target)
    //     .insert(Name::new("TargetSquare"));
}


#[derive(Component)]
pub struct Target {
    pub shape: Shape,
    pub position: Vec2,
    pub radius: f32,
}

pub enum Shape {
    Square,
    RightPlane,
}

impl Target {
    pub fn new(shape: Shape, position: Vec2, radius: f32) -> Self {
        match &shape {
            Shape::Square => {
                Self {
                    shape,
                    position,
                    radius,
                }
            },
            Shape::RightPlane => {
                Self {
                    shape,
                    position,
                    radius: 1000.0,
                }
            },
        }
    }

    pub fn is_in_target(&self, position: Vec2) -> bool {
        match self.shape {
            Shape::Square => {
                let x = position.x;
                let y = position.y;
                let x1 = self.position.x - self.radius;
                let x2 = self.position.x + self.radius;
                let y1 = self.position.y - self.radius;
                let y2 = self.position.y + self.radius;

                x >= x1 && x <= x2 && y >= y1 && y <= y2
            },
            Shape::RightPlane => {
                let x = position.x;
                let x1 = self.position.x;

                x > x1
            },
        }
    }

    pub fn create_sprite_bundle(&self) -> SpriteBundle {
        let x = self.position.x;
        let y = self.position.y;
        let z = 100.0;
        let radius = self.radius;
        match self.shape {
            Shape::Square => {
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, z),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                        color: Color::GREEN,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
            Shape::RightPlane => {
                let plane_x = x + radius;
                SpriteBundle {
                    transform: Transform::from_xyz(plane_x, y, z),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                        color: Color::GREEN,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
        }
    }
}


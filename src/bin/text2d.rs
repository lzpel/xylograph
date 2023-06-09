//! Shows text rendering with moving, rotating and scaling text.
//!
//! Note that this uses [`Text2dBundle`] to display text alongside your other entities in a 2D scene.
//!
//! For an example on how to render text as part of a user interface, independent from the world
//! viewport, you may want to look at `2d/contributors.rs` or `ui/text.rs`.

use bevy::{prelude::*, window::WindowResolution, sprite::Anchor, text::{BreakLineOn, Text2dBounds}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "text2d".into(),
                resolution: WindowResolution::new(400.0,400.0),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(animate_translation)
        .add_system(animate_rotation)
        .add_system(animate_scale)
        //.add_system(system_hello_world)
        //.add_startup_system(system_hello_world)
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        AnimateTranslation,
    ));
    // Demonstrate changing rotation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("rotation", text_style.clone()).with_alignment(text_alignment),
            ..default()
        },
        AnimateRotation,
    ));
    // Demonstrate changing scale
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("scale", text_style).with_alignment(text_alignment),
            ..default()
        },
        AnimateScale,
    ));
    // Demonstrate text wrapping
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        color: Color::WHITE,
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behaviour: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.20, 0.3, 0.70),
                custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(other_box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(AnyCharacter linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behaviour: BreakLineOn::AnyCharacter,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: other_box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    for (text_anchor, color) in [
        (Anchor::TopLeft, Color::RED),
        (Anchor::TopRight, Color::GREEN),
        (Anchor::BottomRight, Color::BLUE),
        (Anchor::BottomLeft, Color::YELLOW),
    ] {
        commands.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!(" Anchor::{text_anchor:?} "),
                    TextStyle {
                        color,
                        ..slightly_smaller_text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(250. * Vec3::Y),
            text_anchor,
            ..default()
        });
    }
    //custom

    let another_box_size = Vec2::new(300.0, 200.0);
    let another_box_position = Vec2::new(-320.0, -250.0);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("images/shrimp.jpg"),
            sprite: Sprite {
                //color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(another_box_size.x, another_box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(another_box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behaviour: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: another_box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);
        transform.scale = Vec3::splat((time.elapsed_seconds().sin() + 1.1) * 2.0);
    }
}
fn system_hello_world() {
    println!("Hello from system");
}
//! A shader and a material that uses it.

use std::f32::consts::PI;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::egui;
use shader_playground::ShaderPlaygroundPlugin;
use shader_playground::RadialDissolveStandardMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_plugin(ShaderPlaygroundPlugin)
        .add_system(inspector_ui)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut pbr_materials: ResMut<Assets<RadialDissolveStandardMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            radius: 5.0,
            height: 0.1,
            resolution: 20,
            segments: 20,
        })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: pbr_materials.add(RadialDissolveStandardMaterial {
            alpha_mode: AlphaMode::Blend,
            base_color: Color::GRAY,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: standard_materials.add(Color::GREEN.into()),
        transform: Transform {
            translation: Vec3::Y * 2.0,
            rotation: Quat::from_axis_angle(Vec3::Z, PI / 4.0),
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-4.0, 5.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings { ..default() },
    ));
}

fn inspector_ui(world: &mut World) {
    let egui_context = world
        .query::<&mut EguiContext>()
        .get_single_mut(world)
        .unwrap()
        .get_mut()
        .clone();
    egui::Window::new("UI").show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::CollapsingHeader::new("Materials")
                .default_open(true)
                .open(Some(true))
                .show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_assets::<
                        RadialDissolveStandardMaterial,
                    >(world, ui);
                });
        });
    });
}

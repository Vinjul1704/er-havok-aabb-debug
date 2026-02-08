use eldenring::{
    cs::{RendMan, WorldChrMan},
    position::HavokPosition,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{F32Vector4, FromStatic, program::Program};
use std::time::Duration;

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui::Condition;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::*;

struct AabbDebugGui {
    pos_available: bool,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    debug_box: bool,
    debug_center: bool,
    debug_grid: bool,
}

impl AabbDebugGui {
    fn new() -> Self {
        Self {
            pos_available: false,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            debug_box: false,
            debug_center: false,
            debug_grid: false,
        }
    }
}

impl ImguiRenderLoop for AabbDebugGui {
    fn render(&mut self, ui: &mut imgui::Ui) {
        // Grab the main player from WorldChrMan if it's available.
        if let Some(player) = unsafe { WorldChrMan::instance() }
            .ok()
            .and_then(|w| w.main_player.as_ref())
        {
            self.pos_available = true;

            // Grab physics module from player.
            let physics = &player.chr_ins.module_container.physics;

            self.pos_x = physics.position.0;
            self.pos_y = physics.position.1;
            self.pos_z = physics.position.2;
        } else {
            self.pos_available = false;
        }

        ui.window("Havok AABB Debug")
            .size([150.0, 182.0], Condition::FirstUseEver)
            .position([10.0, 10.0], Condition::FirstUseEver)
            .build(|| {
                if self.pos_available {
                    ui.text(format!("X: {}", self.pos_x));
                    ui.text(format!("Y: {}", self.pos_y));
                    ui.text(format!("Z: {}", self.pos_z));
                } else {
                    ui.text("X: ---");
                    ui.text("Y: ---");
                    ui.text("Z: ---");
                }

                ui.separator();

                ui.checkbox("AABB Box", &mut self.debug_box);
                ui.checkbox("Center Lines", &mut self.debug_center);
                ui.checkbox("Grid Crosses", &mut self.debug_grid);

                ui.separator();

                if ui.button("Eject") {
                    hudhook::eject();
                }
            });

        // Grab the debug ez draw from RendMan if it's available.
        if let Some(ez_draw) = unsafe { RendMan::instance() }
            .ok()
            .map(|r| r.debug_ez_draw.as_mut())
        {
            if self.debug_box {
                // Set color for the box
                ez_draw.set_color(&F32Vector4(1.0, 0.0, 0.0, 1.0));

                // Draw the AABB box
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, -32.0, 32.0),
                    &HavokPosition::from_xyz(-32.0, -32.0, -32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, -32.0, -32.0),
                    &HavokPosition::from_xyz(-32.0, 32.0, -32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, 32.0, -32.0),
                    &HavokPosition::from_xyz(-32.0, 32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, 32.0, 32.0),
                    &HavokPosition::from_xyz(-32.0, -32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(32.0, -32.0, 32.0),
                    &HavokPosition::from_xyz(32.0, -32.0, -32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(32.0, -32.0, -32.0),
                    &HavokPosition::from_xyz(32.0, 32.0, -32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(32.0, 32.0, -32.0),
                    &HavokPosition::from_xyz(32.0, 32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(32.0, 32.0, 32.0),
                    &HavokPosition::from_xyz(32.0, -32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, -32.0, 32.0),
                    &HavokPosition::from_xyz(32.0, -32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, 32.0, 32.0),
                    &HavokPosition::from_xyz(32.0, 32.0, 32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, 32.0, -32.0),
                    &HavokPosition::from_xyz(32.0, 32.0, -32.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(-32.0, -32.0, -32.0),
                    &HavokPosition::from_xyz(32.0, -32.0, -32.0),
                );
            }

            if self.debug_center {
                // Set color for the center cross
                ez_draw.set_color(&F32Vector4(0.0, 1.0, 0.0, 1.0));

                // Draw cross through center
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(32.0, 0.0, 0.0),
                    &HavokPosition::from_xyz(-32.0, 0.0, 0.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(0.0, 32.0, 0.0),
                    &HavokPosition::from_xyz(0.0, -32.0, 0.0),
                );
                ez_draw.draw_line(
                    &HavokPosition::from_xyz(0.0, 0.0, 32.0),
                    &HavokPosition::from_xyz(0.0, 0.0, -32.0),
                );
            }

            if self.debug_grid {
                // Set color for small crosses
                ez_draw.set_color(&F32Vector4(0.0, 0.0, 1.0, 1.0));

                for xi in -4..=4 {
                    for yi in -4..=4 {
                        for zi in -4..=4 {
                            let x = xi as f32 * 8.0;
                            let y = yi as f32 * 8.0;
                            let z = zi as f32 * 8.0;

                            ez_draw.draw_line(
                                &HavokPosition::from_xyz(x + 0.5, y, z),
                                &HavokPosition::from_xyz(x - 0.5, y, z),
                            );
                            ez_draw.draw_line(
                                &HavokPosition::from_xyz(x, y + 0.5, z),
                                &HavokPosition::from_xyz(x, y - 0.5, z),
                            );
                            ez_draw.draw_line(
                                &HavokPosition::from_xyz(x, y, z + 0.5),
                                &HavokPosition::from_xyz(x, y, z - 0.5),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        if let Err(e) = Hudhook::builder()
            .with::<ImguiDx12Hooks>(AabbDebugGui::new())
            .with_hmodule(hmodule)
            .build()
            .apply()
        {
            tracing::error!("Couldn't apply hooks: {e:?}");
            eject();
        }
    });

    return true;
}

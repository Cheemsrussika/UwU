# BÁO CÁO RÀ SOÁT TOÀN BỘ CODEBASE VỀ VIỆC SỬ DỤNG THƯ VIỆN BEVY (0.19.1)

> Thời gian quét: 2026-09-07 | Phiên bản Bevy: `0.19.1`

## 1. Tổng quan thống kê

- **Tổng số file mã nguồn Rust (.rs)**: `202` files.
- **Số file trực tiếp sử dụng Bevy (`bevy::*`)**: `78` files.
- **Số file tầng hạ tầng/thuật toán độc lập (Không cần import `bevy`)**: `124` files.
- **Toán học (Math)**: 100% sử dụng `bevy::math::{Vec3, Mat4}`. Đã loại bỏ hoàn toàn dependency `glam` khỏi `Cargo.toml`.
- **ECS Entities & Resources**: Toàn bộ các thực thể cốt lõi (`Player`, `Camera`, `RemotePlayer`, `MiningState`, `Inventory`, `VoxelWorld`, `ItemEntityManager`, `ConcurrentChunkStorage`, `TickSystem`, `RenderSnapshot`, `GameState`, `BlockType`, `Chunk`, `ItemStack`, `ItemType`) đều đã derive `Component` hoặc `Resource` của Bevy.
- **Hệ thống Plugin & App Runner**: Ứng dụng chạy trên Bevy App (`bevy::app::App`) và chuỗi Plugin chuyên biệt (`TimePlugin`, `VoxelWorldPlugin`, `ItemEntityPlugin`, `LanNetworkPlugin`, `WgpuRunnerPlugin`).
- **Giới hạn dòng code**: 100% các file đều `<= 99` dòng (không có file nào vượt quá 100 dòng).
- **Kiểm thử**: 53/53 tests vượt qua (`cargo test`).

---

## 2. Danh sách các file TRỰC TIẾP SỬ DỤNG BEVY (78 files)

| STT | File Path | Dòng | Tính năng Bevy sử dụng |
| :--- | :--- | :---: | :--- |
| 1 | `src/main.rs` | 32 | - |
| 2 | `src/app/bevy_app.rs` | 17 | App / Runner, ECS (World/Query), Prelude, Time |
| 3 | `src/app/game_state.rs` | 11 | #[derive(Resource)], Prelude |
| 4 | `src/app/logic_step.rs` | 48 | ECS (World/Query), Math (Vec3/Mat4) |
| 5 | `src/app/logic_worker.rs` | 98 | ECS (World/Query), Math (Vec3/Mat4) |
| 6 | `src/app/mod.rs` | 26 | App / Runner |
| 7 | `src/app/render_movement.rs` | 35 | App / Runner, Math (Vec3/Mat4) |
| 8 | `src/app/worker_commands.rs` | 85 | ECS (World/Query), Math (Vec3/Mat4) |
| 9 | `src/app/worker_remote_cmds.rs` | 44 | ECS (World/Query), Math (Vec3/Mat4) |
| 10 | `src/app/plugins/item_plugin.rs` | 25 | #[derive(Resource)], App / Runner, Prelude |
| 11 | `src/app/plugins/network_plugin.rs` | 25 | #[derive(Resource)], App / Runner, Prelude |
| 12 | `src/app/plugins/runner_plugin.rs` | 22 | App / Runner, Prelude |
| 13 | `src/app/plugins/world_plugin.rs` | 23 | #[derive(Resource)], App / Runner, ECS (World/Query), Prelude |
| 14 | `src/engine/camera.rs` | 45 | #[derive(Component)], Math (Vec3/Mat4), Prelude |
| 15 | `src/engine/gamemode.rs` | 16 | #[derive(Resource)], Prelude |
| 16 | `src/engine/gameplay_tests.rs` | 84 | App / Runner, ECS (World/Query), Math (Vec3/Mat4) |
| 17 | `src/engine/inventory.rs` | 96 | #[derive(Component)], App / Runner, Prelude |
| 18 | `src/engine/inventory_tests.rs` | 73 | App / Runner, Math (Vec3/Mat4) |
| 19 | `src/engine/mining.rs` | 90 | #[derive(Component)], Prelude |
| 20 | `src/engine/mining_tests.rs` | 70 | Math (Vec3/Mat4) |
| 21 | `src/engine/sprint_swing_tests.rs` | 52 | ECS (World/Query), Math (Vec3/Mat4) |
| 22 | `src/engine/state.rs` | 86 | #[derive(Resource)], ECS (World/Query), Math (Vec3/Mat4), Prelude |
| 23 | `src/engine/tick.rs` | 58 | #[derive(Resource)], Prelude |
| 24 | `src/engine/blocks/block_entity_registry.rs` | 24 | #[derive(Resource)], Prelude |
| 25 | `src/engine/blocks/chest.rs` | 56 | #[derive(Component)], Prelude |
| 26 | `src/engine/blocks/crafting_table.rs` | 33 | #[derive(Component)], Prelude |
| 27 | `src/engine/blocks/furnace.rs` | 82 | #[derive(Component)], Prelude |
| 28 | `src/engine/debug/piechart_state.rs` | 39 | #[derive(Resource)], Prelude |
| 29 | `src/engine/debug/profiler.rs` | 88 | #[derive(Resource)], Prelude |
| 30 | `src/engine/debug/tests.rs` | 63 | Math (Vec3/Mat4) |
| 31 | `src/engine/ecs/components.rs` | 27 | #[derive(Component)], Math (Vec3/Mat4), Prelude |
| 32 | `src/engine/ecs/item_pickup.rs` | 42 | ECS (World/Query), Math (Vec3/Mat4), Prelude |
| 33 | `src/engine/ecs/systems.rs` | 63 | ECS (World/Query), Prelude |
| 34 | `src/engine/items/dropped_item.rs` | 72 | ECS (World/Query), Math (Vec3/Mat4) |
| 35 | `src/engine/items/item_mesh.rs` | 74 | Math (Vec3/Mat4) |
| 36 | `src/engine/items/item_stack.rs` | 25 | #[derive(Component)], Prelude |
| 37 | `src/engine/items/item_type.rs` | 58 | #[derive(Component)], App / Runner, Prelude |
| 38 | `src/engine/items/manager.rs` | 88 | #[derive(Resource)], ECS (World/Query), Math (Vec3/Mat4), Time |
| 39 | `src/engine/items/tests.rs` | 84 | App / Runner, ECS (World/Query), Math (Vec3/Mat4) |
| 40 | `src/engine/mobs/animal_ai.rs` | 82 | #[derive(Component)], Math (Vec3/Mat4), Prelude |
| 41 | `src/engine/mobs/animal_def.rs` | 44 | #[derive(Component)], Prelude |
| 42 | `src/engine/mobs/animal_mesh.rs` | 45 | Math (Vec3/Mat4) |
| 43 | `src/engine/mobs/animal_physics.rs` | 62 | ECS (World/Query), Math (Vec3/Mat4) |
| 44 | `src/engine/mobs/manager.rs` | 54 | ECS (World/Query), Math (Vec3/Mat4), Prelude |
| 45 | `src/engine/mobs/tests.rs` | 53 | ECS (World/Query), Math (Vec3/Mat4), Prelude |
| 46 | `src/engine/player_ctrl/collision.rs` | 17 | ECS (World/Query), Math (Vec3/Mat4) |
| 47 | `src/engine/player_ctrl/def.rs` | 82 | #[derive(Component)], ECS (World/Query), Math (Vec3/Mat4), Prelude |
| 48 | `src/engine/player_ctrl/hand_block.rs` | 63 | Math (Vec3/Mat4) |
| 49 | `src/engine/player_ctrl/hand_item.rs` | 18 | Math (Vec3/Mat4) |
| 50 | `src/engine/player_ctrl/hand_tool.rs` | 61 | Math (Vec3/Mat4) |
| 51 | `src/engine/player_ctrl/mesh.rs` | 44 | Math (Vec3/Mat4) |
| 52 | `src/engine/player_ctrl/physics.rs` | 97 | ECS (World/Query), Math (Vec3/Mat4) |
| 53 | `src/engine/player_ctrl/steve_box.rs` | 47 | Math (Vec3/Mat4) |
| 54 | `src/engine/stats/experience.rs` | 45 | #[derive(Component)], Prelude |
| 55 | `src/engine/stats/health.rs` | 39 | #[derive(Component)], Prelude |
| 56 | `src/engine/stats/hunger.rs` | 58 | #[derive(Component)], Prelude |
| 57 | `src/input/command.rs` | 37 | Math (Vec3/Mat4) |
| 58 | `src/network/packet_decode.rs` | 86 | Math (Vec3/Mat4) |
| 59 | `src/network/protocol.rs` | 33 | Math (Vec3/Mat4) |
| 60 | `src/network/remote_player.rs` | 39 | #[derive(Component)], Math (Vec3/Mat4), Prelude |
| 61 | `src/network/server.rs` | 99 | Math (Vec3/Mat4) |
| 62 | `src/network/server_client.rs` | 82 | Math (Vec3/Mat4), Time |
| 63 | `src/network/tests.rs` | 70 | ECS (World/Query), Math (Vec3/Mat4) |
| 64 | `src/render/chunk_border_mesh.rs` | 76 | Math (Vec3/Mat4) |
| 65 | `src/render/destroy_overlay.rs` | 48 | Math (Vec3/Mat4) |
| 66 | `src/render/overlay.rs` | 70 | Math (Vec3/Mat4) |
| 67 | `src/render/hud_ui/player_preview.rs` | 69 | Math (Vec3/Mat4) |
| 68 | `src/render/hud_ui/preview_boxes.rs` | 25 | Math (Vec3/Mat4) |
| 69 | `src/render/item_model/extruder.rs` | 74 | Math (Vec3/Mat4) |
| 70 | `src/render/item_model/tests.rs` | 34 | Math (Vec3/Mat4) |
| 71 | `src/render/pipelines/init.rs` | 90 | Math (Vec3/Mat4) |
| 72 | `src/render/pipelines/update.rs` | 90 | ECS (World/Query), Math (Vec3/Mat4) |
| 73 | `src/world/block.rs` | 53 | #[derive(Component)], Prelude |
| 74 | `src/world/chunk.rs` | 46 | #[derive(Component)], Prelude |
| 75 | `src/world/chunk_mesh.rs` | 76 | Math (Vec3/Mat4) |
| 76 | `src/world/concurrent_storage.rs` | 95 | #[derive(Resource)], Prelude |
| 77 | `src/world/raycast.rs` | 66 | ECS (World/Query), Math (Vec3/Mat4) |
| 78 | `src/world/voxel.rs` | 91 | #[derive(Resource)], ECS (World/Query), Prelude |

---

## 3. Danh sách các file KHÔNG TRỰC TIẾP IMPORT BEVY (124 files)

Các file này thuộc các tầng hạ tầng kỹ thuật, giao thức mạng mức nhị phân, bộ giải mã nén hoặc thuật toán voxel độc lập không cần thiết phải phụ thuộc trực tiếp vào crate Bevy:

### 3.1. 2D HUD & UI Layout (24 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/input/hud_hit.rs` | 73 | 2D HUD & UI Layout |
| 2 | `src/input/menu_clicks.rs` | 88 | 2D HUD & UI Layout |
| 3 | `src/input/menu_tests.rs` | 59 | 2D HUD & UI Layout |
| 4 | `src/render/hud_ui/atlas.rs` | 75 | 2D HUD & UI Layout |
| 5 | `src/render/hud_ui/atlas_items.rs` | 53 | 2D HUD & UI Layout |
| 6 | `src/render/hud_ui/atlas_menu.rs` | 8 | 2D HUD & UI Layout |
| 7 | `src/render/hud_ui/bars.rs` | 76 | 2D HUD & UI Layout |
| 8 | `src/render/hud_ui/builder.rs` | 43 | 2D HUD & UI Layout |
| 9 | `src/render/hud_ui/direct_connect_screen.rs` | 55 | 2D HUD & UI Layout |
| 10 | `src/render/hud_ui/font.rs` | 23 | 2D HUD & UI Layout |
| 11 | `src/render/hud_ui/inventory.rs` | 71 | 2D HUD & UI Layout |
| 12 | `src/render/hud_ui/iso_block.rs` | 72 | 2D HUD & UI Layout |
| 13 | `src/render/hud_ui/item_icon.rs` | 28 | 2D HUD & UI Layout |
| 14 | `src/render/hud_ui/lan_screen.rs` | 62 | 2D HUD & UI Layout |
| 15 | `src/render/hud_ui/menu_bg.rs` | 24 | 2D HUD & UI Layout |
| 16 | `src/render/hud_ui/menu_button.rs` | 48 | 2D HUD & UI Layout |
| 17 | `src/render/hud_ui/menu_render.rs` | 48 | 2D HUD & UI Layout |
| 18 | `src/render/hud_ui/pause_screen.rs` | 35 | 2D HUD & UI Layout |
| 19 | `src/render/hud_ui/piechart.rs` | 62 | 2D HUD & UI Layout |
| 20 | `src/render/hud_ui/renderer.rs` | 96 | 2D HUD & UI Layout |
| 21 | `src/render/hud_ui/resources.rs` | 76 | 2D HUD & UI Layout |
| 22 | `src/render/hud_ui/slot_coords.rs` | 24 | 2D HUD & UI Layout |
| 23 | `src/render/hud_ui/slot_render.rs` | 32 | 2D HUD & UI Layout |
| 24 | `src/render/hud_ui/title_screen.rs` | 53 | 2D HUD & UI Layout |

### 3.2. App Event Loop & Windowing (14 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/app/app_events.rs` | 75 | App Event Loop & Windowing |
| 2 | `src/app/app_init.rs` | 31 | App Event Loop & Windowing |
| 3 | `src/app/app_state.rs` | 62 | App Event Loop & Windowing |
| 4 | `src/app/key_events.rs` | 62 | App Event Loop & Windowing |
| 5 | `src/app/key_shortcuts.rs` | 28 | App Event Loop & Windowing |
| 6 | `src/app/lan_sync.rs` | 93 | App Event Loop & Windowing |
| 7 | `src/app/menu_action_handler.rs` | 50 | App Event Loop & Windowing |
| 8 | `src/app/mining_helper.rs` | 47 | App Event Loop & Windowing |
| 9 | `src/app/mouse_actions.rs` | 64 | App Event Loop & Windowing |
| 10 | `src/app/mouse_input_handler.rs` | 49 | App Event Loop & Windowing |
| 11 | `src/app/render_loop.rs` | 56 | App Event Loop & Windowing |
| 12 | `src/app/text_input.rs` | 80 | App Event Loop & Windowing |
| 13 | `src/app/worker_snapshot.rs` | 37 | App Event Loop & Windowing |
| 14 | `src/app/worker_tick.rs` | 28 | App Event Loop & Windowing |

### 3.3. Game Logic & Helpers (25 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/engine/inventory_click.rs` | 88 | Game Logic & Helpers |
| 2 | `src/engine/player.rs` | 1 | Game Logic & Helpers |
| 3 | `src/engine/scheduled_tick.rs` | 36 | Game Logic & Helpers |
| 4 | `src/engine/state_slots.rs` | 32 | Game Logic & Helpers |
| 5 | `src/engine/blocks/furnace_fuel.rs` | 13 | Game Logic & Helpers |
| 6 | `src/engine/blocks/furnace_recipe.rs` | 13 | Game Logic & Helpers |
| 7 | `src/engine/blocks/tests.rs` | 57 | Game Logic & Helpers |
| 8 | `src/engine/crafting/matcher.rs` | 32 | Game Logic & Helpers |
| 9 | `src/engine/crafting/recipe.rs` | 65 | Game Logic & Helpers |
| 10 | `src/engine/crafting/recipes_basic.rs` | 38 | Game Logic & Helpers |
| 11 | `src/engine/crafting/recipes_tools.rs` | 37 | Game Logic & Helpers |
| 12 | `src/engine/crafting/tests.rs` | 59 | Game Logic & Helpers |
| 13 | `src/engine/debug/result_field.rs` | 26 | Game Logic & Helpers |
| 14 | `src/engine/items/item_id.rs` | 68 | Game Logic & Helpers |
| 15 | `src/engine/items/item_props.rs` | 44 | Game Logic & Helpers |
| 16 | `src/engine/player_ctrl/flight.rs` | 9 | Game Logic & Helpers |
| 17 | `src/engine/player_ctrl/stats_update.rs` | 57 | Game Logic & Helpers |
| 18 | `src/engine/player_ctrl/steve_uv.rs` | 74 | Game Logic & Helpers |
| 19 | `src/engine/stats/food_data.rs` | 15 | Game Logic & Helpers |
| 20 | `src/engine/stats/tests.rs` | 74 | Game Logic & Helpers |
| 21 | `src/world/block_conv.rs` | 40 | Game Logic & Helpers |
| 22 | `src/world/block_props.rs` | 43 | Game Logic & Helpers |
| 23 | `src/world/chunk_mesh_par.rs` | 69 | Game Logic & Helpers |
| 24 | `src/world/chunk_pos.rs` | 65 | Game Logic & Helpers |
| 25 | `src/world/tests.rs` | 80 | Game Logic & Helpers |

### 3.4. Module Declarations (17 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/app/plugins/mod.rs` | 9 | Module Declarations |
| 2 | `src/engine/mod.rs` | 38 | Module Declarations |
| 3 | `src/engine/blocks/mod.rs` | 16 | Module Declarations |
| 4 | `src/engine/crafting/mod.rs` | 10 | Module Declarations |
| 5 | `src/engine/debug/mod.rs` | 10 | Module Declarations |
| 6 | `src/engine/ecs/mod.rs` | 7 | Module Declarations |
| 7 | `src/engine/items/mod.rs` | 17 | Module Declarations |
| 8 | `src/engine/mobs/mod.rs` | 14 | Module Declarations |
| 9 | `src/engine/player_ctrl/mod.rs` | 14 | Module Declarations |
| 10 | `src/engine/stats/mod.rs` | 12 | Module Declarations |
| 11 | `src/input/mod.rs` | 9 | Module Declarations |
| 12 | `src/network/mod.rs` | 29 | Module Declarations |
| 13 | `src/render/mod.rs` | 21 | Module Declarations |
| 14 | `src/render/hud_ui/mod.rs` | 30 | Module Declarations |
| 15 | `src/render/item_model/mod.rs` | 9 | Module Declarations |
| 16 | `src/render/pipelines/mod.rs` | 13 | Module Declarations |
| 17 | `src/world/mod.rs` | 58 | Module Declarations |

### 3.5. Network Wire & Binary Protocol (12 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/network/block_pos_codec.rs` | 18 | Network Wire & Binary Protocol |
| 2 | `src/network/byte_buf.rs` | 97 | Network Wire & Binary Protocol |
| 3 | `src/network/client.rs` | 66 | Network Wire & Binary Protocol |
| 4 | `src/network/lan_detector.rs` | 79 | Network Wire & Binary Protocol |
| 5 | `src/network/lan_discovery.rs` | 45 | Network Wire & Binary Protocol |
| 6 | `src/network/lan_pinger.rs` | 57 | Network Wire & Binary Protocol |
| 7 | `src/network/local_ipc.rs` | 52 | Network Wire & Binary Protocol |
| 8 | `src/network/packet_codec.rs` | 2 | Network Wire & Binary Protocol |
| 9 | `src/network/packet_encode.rs` | 76 | Network Wire & Binary Protocol |
| 10 | `src/network/packet_frame.rs` | 31 | Network Wire & Binary Protocol |
| 11 | `src/network/packet_types.rs` | 17 | Network Wire & Binary Protocol |
| 12 | `src/network/varint.rs` | 51 | Network Wire & Binary Protocol |

### 3.6. Persistence & MCA Storage (9 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/world/autosave.rs` | 54 | Persistence & MCA Storage |
| 2 | `src/world/chunk_codec.rs` | 48 | Persistence & MCA Storage |
| 3 | `src/world/chunk_save.rs` | 82 | Persistence & MCA Storage |
| 4 | `src/world/persistence_tests.rs` | 95 | Persistence & MCA Storage |
| 5 | `src/world/region_format.rs` | 17 | Persistence & MCA Storage |
| 6 | `src/world/region_reader.rs` | 59 | Persistence & MCA Storage |
| 7 | `src/world/region_tests.rs` | 71 | Persistence & MCA Storage |
| 8 | `src/world/region_writer.rs` | 58 | Persistence & MCA Storage |
| 9 | `src/world/world_persistence.rs` | 92 | Persistence & MCA Storage |

### 3.7. Voxel Algorithms & Simulation (11 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/world/chunk_section.rs` | 55 | Voxel Algorithms & Simulation |
| 2 | `src/world/chunk_status.rs` | 38 | Voxel Algorithms & Simulation |
| 3 | `src/world/colormap.rs` | 55 | Voxel Algorithms & Simulation |
| 4 | `src/world/flat_gen.rs` | 18 | Voxel Algorithms & Simulation |
| 5 | `src/world/fluid.rs` | 74 | Voxel Algorithms & Simulation |
| 6 | `src/world/fluid_camera.rs` | 32 | Voxel Algorithms & Simulation |
| 7 | `src/world/fluid_slope_dist.rs` | 42 | Voxel Algorithms & Simulation |
| 8 | `src/world/fluid_spread.rs` | 70 | Voxel Algorithms & Simulation |
| 9 | `src/world/fluid_spread_horiz.rs` | 46 | Voxel Algorithms & Simulation |
| 10 | `src/world/water_flow_dir.rs` | 61 | Voxel Algorithms & Simulation |
| 11 | `src/world/water_slope.rs` | 82 | Voxel Algorithms & Simulation |

### 3.8. WGPU Graphics Backend (12 files)

| STT | File Path | Dòng | Vai trò kiến trúc |
| :--- | :--- | :---: | :--- |
| 1 | `src/render/pipeline.rs` | 99 | WGPU Graphics Backend |
| 2 | `src/render/remote_mesh.rs` | 17 | WGPU Graphics Backend |
| 3 | `src/render/texture_atlas.rs` | 88 | WGPU Graphics Backend |
| 4 | `src/render/types.rs` | 32 | WGPU Graphics Backend |
| 5 | `src/render/water_anim.rs` | 67 | WGPU Graphics Backend |
| 6 | `src/render/item_model/model_cache.rs` | 32 | WGPU Graphics Backend |
| 7 | `src/render/item_model/registry.rs` | 13 | WGPU Graphics Backend |
| 8 | `src/render/pipelines/player_buffer.rs` | 22 | WGPU Graphics Backend |
| 9 | `src/render/pipelines/render.rs` | 92 | WGPU Graphics Backend |
| 10 | `src/render/pipelines/resources.rs` | 72 | WGPU Graphics Backend |
| 11 | `src/render/pipelines/shadow_res.rs` | 37 | WGPU Graphics Backend |
| 12 | `src/render/pipelines/texture_res.rs` | 76 | WGPU Graphics Backend |

---

## 4. Kết luận & Đánh giá

1. **Không còn bất kỳ thư viện toán học hay ECS bên thứ 3 nào** ngoài Bevy Engine (`bevy 0.19.1`).
2. **Mọi thực thể game** (Player, Camera, Items, World, Chunks, Blocks, Mining, Inventory, Network players) đã được cấu trúc hóa theo kiến trúc Entity-Component-System (ECS) và Resource của Bevy.
3. **Các file còn lại** là các thuật toán tính toán độc lập (MCA format, VarInt bitwise, UI vertex builders) hoàn toàn tương thích và được gọi từ các hệ thống Bevy ECS.
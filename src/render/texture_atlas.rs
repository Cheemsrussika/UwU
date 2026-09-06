use image::GenericImageView;

pub struct TextureAtlas;

impl TextureAtlas {
    pub const RESOLUTION: u32 = 64;
    pub const LAYER_COUNT: u32 = 28;

    fn load(bytes: &[u8], tint: Option<[f32; 3]>, fallback: [u8; 4]) -> Vec<u8> {
        let res = Self::RESOLUTION as usize;
        if let Ok(img) = image::load_from_memory(bytes) {
            let mut out = Vec::with_capacity(res * res * 4);
            let (iw, ih) = (img.width(), img.height());
            for y in 0..Self::RESOLUTION {
                for x in 0..Self::RESOLUTION {
                    let p = img.get_pixel((x * iw) / Self::RESOLUTION, (y * ih) / Self::RESOLUTION);
                    let (r, g, b, a) = if p.0[3] <= 1 {
                        (0, 0, 0, 0)
                    } else {
                        let (cr, cg, cb) = match tint {
                            Some(t) => (
                                ((p.0[0] as f32 / 255.0) * t[0] * 255.0).clamp(0.0, 255.0) as u8,
                                ((p.0[1] as f32 / 255.0) * t[1] * 255.0).clamp(0.0, 255.0) as u8,
                                ((p.0[2] as f32 / 255.0) * t[2] * 255.0).clamp(0.0, 255.0) as u8,
                            ),
                            None => (p.0[0], p.0[1], p.0[2]),
                        };
                        (cr, cg, cb, p.0[3])
                    };
                    out.extend_from_slice(&[r, g, b, a]);
                }
            }
            out
        } else {
            vec![fallback; res * res].into_iter().flatten().collect()
        }
    }

    pub fn generate_pixel_atlas() -> Vec<u8> {
        let mut pixels = Vec::with_capacity((64 * 64 * 4 * Self::LAYER_COUNT) as usize);
        let cm = super::super::world::colormap::ColorMap::new();
        let grass_tint = cm.get_grass_color(0.8, 0.4);
        let foliage_tint = cm.get_foliage_color(0.8, 0.4);
        let water_tint = [0.247, 0.463, 0.894];

        // Blocks: 0=GrassTop, 1=Dirt, 2=Wood, 3=Stone, 4=WaterStill, 5=WaterFlow, 6=Steve, 7=GrassSide
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/grass_top.png"), Some(grass_tint), [121, 192, 90, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/dirt.png"), None, [120, 80, 45, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/wood.png"), None, [160, 115, 65, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/stone.png"), None, [130, 130, 130, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/water_still.png"), Some(water_tint), [40, 100, 220, 210]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/water_flow.png"), Some(water_tint), [55, 120, 230, 210]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/entity/player/steve.png"), None, [180, 100, 80, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/grass_side.png"), None, [130, 95, 60, 255]));

        // Items: 8=Apple, 9=Bread, 10=Bucket, 11=WaterBucket, 12=DiamondSword, 13=DiamondPickaxe
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/apple.png"), None, [220, 40, 40, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/bread.png"), None, [180, 130, 70, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/bucket.png"), None, [180, 180, 180, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/water_bucket.png"), None, [80, 140, 240, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/diamond_sword.png"), None, [70, 220, 220, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/item/diamond_pickaxe.png"), None, [70, 220, 220, 255]));

        // Wood & Leaves: 14=OakLogSide, 15=OakLogTop, 16=OakPlanks, 17=OakLeaves
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/oak_log.png"), None, [140, 100, 60, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/oak_log_top.png"), None, [160, 130, 80, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/oak_planks.png"), None, [170, 135, 85, 255]));
        pixels.extend(Self::load(include_bytes!("../../assets/textures/block/oak_leaves.png"), Some(foliage_tint), [60, 140, 40, 255]));
 
        // Destroy stages: 18..=27 (0 to 9)
        for stage_bytes in [
            include_bytes!("../../assets/textures/block/destroy_stage_0.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_1.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_2.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_3.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_4.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_5.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_6.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_7.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_8.png").as_slice(),
            include_bytes!("../../assets/textures/block/destroy_stage_9.png").as_slice(),
        ] {
            pixels.extend(Self::load(stage_bytes, None, [0, 0, 0, 0]));
        }

        pixels
    }
}
mod v3 {
    use crate::v3::{Entity, GridLayer, Layer, Level, TileDef, TileLayer};

    const ENTITY_1: &str = include_str!("test_data/v3/entity_1.json");
    const ENTITY_2: &str = include_str!("test_data/v3/entity_2.json");
    const ENTITY_3: &str = include_str!("test_data/v3/entity_3.json");

    const ENTITY_LAYER: &str = include_str!("test_data/v3/entity_layer.json");
    const GRID_2_LAYER: &str = include_str!("test_data/v3/grid_2_layer.json");
    const TILE_1_LAYER: &str = include_str!("test_data/v3/tile_1_layer.json");
    const TILE_2_LAYER: &str = include_str!("test_data/v3/tile_2_layer.json");

    const LEVEL: &str = include_str!("test_data/v3/level.json");

    #[test]
    fn parse_entity_1() {
        let entity: Entity = serde_json::from_str(ENTITY_1).unwrap();

        assert_eq!(entity.name, "butterfly".to_string());
        assert_eq!(entity.id, 6);

        assert_eq!(entity.x, 1216);
        assert_eq!(entity.y, 800);

        assert_eq!(entity.origin_x, Some(16));
        assert_eq!(entity.origin_y, Some(16));
        assert_eq!(entity.rotation, Some(0.0));

        let entity_values = entity.values.unwrap();
        assert_eq!(
            entity_values.get("transition_mode_time"),
            Some(&"1".to_string())
        );
        assert_eq!(entity_values.get("fire_mode_time"), Some(&"3".to_string()));
        assert_eq!(entity_values.get("fire_spacing"), Some(&"0.1".to_string()));
    }

    #[test]
    fn parse_entity_2() {
        let entity: Entity = serde_json::from_str(ENTITY_2).unwrap();
        assert_eq!(entity.name, "codex_unlock".to_string());
    }

    #[test]
    fn parse_entity_3() {
        let entity: Entity = serde_json::from_str(ENTITY_3).unwrap();
        assert_eq!(entity.name, "respawn".to_string());
    }

    #[test]
    fn parse_entity_layer() {
        let layer: Layer = serde_json::from_str(ENTITY_LAYER).unwrap();

        if let Layer::Entity {
            name,
            offset_x,
            offset_y,
            grid_cell_width,
            grid_cell_height,
            grid_cells_x,
            grid_cells_y,
            entities,
            ..
        } = layer
        {
            assert_eq!(name, "entities".to_string());
            assert_eq!(offset_x, Some(0));
            assert_eq!(offset_y, Some(0));

            assert_eq!(grid_cell_width, 32);
            assert_eq!(grid_cell_height, 32);
            assert_eq!(grid_cells_x, 40);
            assert_eq!(grid_cells_y, 45);

            assert_eq!(entities.len(), 4);
            assert_eq!(entities.get(0).unwrap().name, "player_start".to_string());
            assert_eq!(entities.get(1).unwrap().name, "codex_unlock".to_string());
            assert_eq!(entities.get(2).unwrap().name, "respawn".to_string());
            assert_eq!(entities.get(3).unwrap().name, "butterfly".to_string());
        } else {
            panic!("Entity layer deserialized as the wrong variant");
        }
    }

    #[test]
    fn parse_grid_2_layer() {
        let layer: Layer = serde_json::from_str(GRID_2_LAYER).unwrap();

        if let Layer::Grid(GridLayer::Dim2 { grid_2d, .. }) = layer {
            let grid_row = grid_2d.get(0).unwrap();
            assert_eq!(grid_row.get(0), Some(&"1".to_string()));
            assert_eq!(grid_row.get(1), Some(&"0".to_string()));
        } else {
            panic!("Grid layer deserialized as the wrong variant");
        }
    }

    #[test]
    fn parse_tile_1_layer() {
        let layer: Layer = serde_json::from_str(TILE_1_LAYER).unwrap();

        if let Layer::Tile(TileLayer::Dim1 { data_coords, .. }) = layer {
            assert_eq!(data_coords.len(), 1840);
            assert_eq!(data_coords.get(0), Some(&TileDef::Dim1([-1])));
            assert_eq!(data_coords.get(1177), Some(&TileDef::Dim2([3, 0])));
        } else {
            panic!("Tile layer deserialized as the wrong variant");
        }
    }

    #[test]
    fn parse_tile_2_layer() {
        let layer: Layer = serde_json::from_str(TILE_2_LAYER).unwrap();

        if let Layer::Tile(TileLayer::Dim2 { data_coords_2d, .. }) = layer {
            assert_eq!(data_coords_2d.len(), 46);
            assert_eq!(data_coords_2d[0].len(), 40);
            assert_eq!(data_coords_2d[0][0], TileDef::Dim1([-1]));
        } else {
            panic!("Tile layer deseriawlized as the wrong variant");
        }
    }

    #[test]
    fn parse_level() {
        let level: Level = serde_json::from_str(LEVEL).unwrap();

        assert_eq!(level.width, 1280);
        assert_eq!(level.height, 1440);
        assert_eq!(level.offset_x, Some(0));
        assert_eq!(level.offset_y, Some(0));

        assert_eq!(level.values.get("cardinality"), Some(&"0".to_string()));
        assert_eq!(level.values.get("name"), Some(&"".to_string()));
        assert_eq!(level.values.get("campaign"), Some(&"".to_string()));
    }
}

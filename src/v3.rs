use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Deserialize)]
pub struct Level {
    pub width: i32,
    pub height: i32,
    #[serde(alias = "offsetX")]
    pub offset_x: Option<i32>,
    #[serde(alias = "offsetY")]
    pub offset_y: Option<i32>,
    pub layers: Vec<Layer>,
    pub values: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Layer {
    Entity {
        name: String,
        #[serde(alias = "_eid")]
        eid: String,
        #[serde(alias = "offsetX")]
        offset_x: Option<i32>,
        #[serde(alias = "offsetY")]
        offset_y: Option<i32>,
        #[serde(alias = "gridCellWidth")]
        grid_cell_width: i32,
        #[serde(alias = "gridCellHeight")]
        grid_cell_height: i32,
        #[serde(alias = "gridCellsX")]
        grid_cells_x: i32,
        #[serde(alias = "gridCellsY")]
        grid_cells_y: i32,

        entities: Vec<Entity>,
    },
    Grid(GridLayer),
    Tile(TileLayer),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GridLayer {
    Dim1 {
        name: String,
        #[serde(alias = "_eid")]
        eid: String,
        #[serde(alias = "offsetX")]
        offset_x: Option<i32>,
        #[serde(alias = "offsetY")]
        offset_y: Option<i32>,
        #[serde(alias = "gridCellWidth")]
        grid_cell_width: i32,
        #[serde(alias = "gridCellHeight")]
        grid_cell_height: i32,
        #[serde(alias = "gridCellsX")]
        grid_cells_x: i32,
        #[serde(alias = "gridCellsY")]
        grid_cells_y: i32,

        grid: Vec<String>,
    },
    Dim2 {
        name: String,
        #[serde(alias = "_eid")]
        eid: String,
        #[serde(alias = "offsetX")]
        offset_x: Option<i32>,
        #[serde(alias = "offsetY")]
        offset_y: Option<i32>,
        #[serde(alias = "gridCellWidth")]
        grid_cell_width: i32,
        #[serde(alias = "gridCellHeight")]
        grid_cell_height: i32,
        #[serde(alias = "gridCellsX")]
        grid_cells_x: i32,
        #[serde(alias = "gridCellsY")]
        grid_cells_y: i32,

        #[serde(alias = "grid2D")]
        grid_2d: Vec<Vec<String>>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TileLayer {
    Dim1 {
        name: String,
        #[serde(alias = "_eid")]
        eid: String,
        #[serde(alias = "offsetX")]
        offset_x: Option<i32>,
        #[serde(alias = "offsetY")]
        offset_y: Option<i32>,
        #[serde(alias = "gridCellWidth")]
        grid_cell_width: i32,
        #[serde(alias = "gridCellHeight")]
        grid_cell_height: i32,
        #[serde(alias = "gridCellsX")]
        grid_cells_x: i32,
        #[serde(alias = "gridCellsY")]
        grid_cells_y: i32,

        tileset: String,
        #[serde(alias = "data")]
        data: Vec<TileDef>,
    },
    Dim2 {
        name: String,
        #[serde(alias = "_eid")]
        eid: String,
        #[serde(alias = "offsetX")]
        offset_x: Option<i32>,
        #[serde(alias = "offsetY")]
        offset_y: Option<i32>,
        #[serde(alias = "gridCellWidth")]
        grid_cell_width: i32,
        #[serde(alias = "gridCellHeight")]
        grid_cell_height: i32,
        #[serde(alias = "gridCellsX")]
        grid_cells_x: i32,
        #[serde(alias = "gridCellsY")]
        grid_cells_y: i32,

        tileset: String,
        // XXX: this is probably broken but I don't need it.
        #[serde(rename = "dataCoords2D")]
        data_coords_2d: Vec<Vec<TileDef>>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TileDef {
    Dim1([i32; 1]),
    Dim2([i32; 2]),
}

#[derive(Deserialize)]
pub struct Entity {
    pub name: String,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    #[serde(alias = "originX")]
    pub origin_x: Option<i32>,
    #[serde(alias = "originY")]
    pub origin_y: Option<i32>,
    pub nodes: Option<Vec<Position>>,
    pub rotation: Option<f32>,
    pub values: Option<BTreeMap<String, Value>>,
}

#[derive(Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

use zertz_core::coordinate::Coordinate as CoreCoordinate;

const X_PADDING: u16 = 4;
const Y_PADDING: u16 = 2;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl Coordinate {
    #[inline]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn into_core_coord(self, origin: Self) -> CoreCoordinate {
        let x = (origin.y * X_PADDING + 2 * self.x * Y_PADDING
            - 2 * origin.x * Y_PADDING
            - X_PADDING * self.y)
            / (2 * X_PADDING * Y_PADDING);
        let y = (origin.y - self.y) / Y_PADDING;

        CoreCoordinate::new(x as usize, y as usize)
    }

    pub fn from_core_coord(coord: CoreCoordinate, origin: Self) -> Self {
        let CoreCoordinate { x, y } = coord;
        let coord_x = x as u16;
        let coord_y = y as u16;

        Self::new(
            origin.x + X_PADDING * coord_x - X_PADDING / 2 * coord_y,
            origin.y - Y_PADDING * coord_y,
        )
    }
}

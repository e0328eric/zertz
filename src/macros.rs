#[macro_export]
macro_rules! append_occupied_coordinate {
    ($self: ident: $coord_list: expr, $coord: expr, $direction: expr) => {
        let Some(catched_coord) = $coord.adjacent($direction) else { return $coord_list; };
        let Some(marble_land_coord) = catched_coord.adjacent($direction) else { return $coord_list; };

        if let Some(Ring::Occupied(_)) = $self.board.get(catched_coord) {
            if let Some(Ring::Vacant) = $self.board.get(marble_land_coord) {
                $coord_list.push(CatchableMove { catched_coord, marble_land_coord });
            }
        }
    };
}

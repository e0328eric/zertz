#[macro_export]
macro_rules! append_occupied_coordinate {
    ($self: ident: $coord_list: expr, $coord: expr, $direction: expr) => {
        let _ = 'blk: {
            let Some(catched_coord) = $coord.adjacent($direction) else { break 'blk; };
            let Some(marble_land_coord) = catched_coord.adjacent($direction) else { break 'blk; };

            if let Some(Ring::Occupied(_)) = $self.board.get(catched_coord) {
                if let Some(Ring::Vacant) = $self.board.get(marble_land_coord) {
                    $coord_list.push(CatchableMove {
                        start_coord: $coord,
                        catched_coord,
                        marble_land_coord,
                    });
                }
            }
        };
    };
}

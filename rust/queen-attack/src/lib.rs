#[derive(Debug)]
pub struct ChessPosition {
    position: (i32, i32)
}

#[derive(Debug)]
pub struct Queen {
    position: (i32, i32)
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 
        || rank > 7
        || file < 0
        || file > 7 {
            None
        }
        else {
            Some(ChessPosition {
                position: (rank, file)
            })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position: position.position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        other.position.1 == self.position.1 
        || other.position.0 == self.position.0 
        || ((other.position.1 - self.position.1) / (other.position.0 - self.position.0)).abs() == 1 
    }
}

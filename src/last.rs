const MAX_IDS: usize = 4;

pub struct LastIDs {
    ids: Vec<i64>,
}

impl LastIDs {
    pub fn new() -> LastIDs {
        LastIDs {
            ids: Vec::with_capacity(MAX_IDS),
        }
    }

    pub fn add_id(&mut self, new_id: i64) {
        let ids = &mut self.ids;

        if ids.len() >= MAX_IDS {
            ids.remove(0);
        }

        ids.push(new_id);
    }

    pub fn get_ids(&self) -> &Vec<i64> {
        &self.ids
    }
}

use time::{self, Tm, Duration};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Item {
    uuid: String,
    solution: String,
    tries_left: usize,
    expires: i64
}

impl Item {
    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn solution(&self) -> String {
        self.solution.clone()
    }

    pub fn tries_left(&self) -> usize {
        self.tries_left
    }

    pub fn expires(&self) -> i64 {
        self.expires
    }

    pub fn dec_tries_left(&self) -> Item {
        let r = self.clone();
        Item { tries_left: self.tries_left - 1, .. r }
    }
}

pub struct ItemBuilder {
    uuid: Option<String>,
    solution: Option<String>,
    tries_left: Option<usize>,
    expires: Option<Tm>,
}

pub fn build_item() -> ItemBuilder {
    ItemBuilder {
        uuid: None,
        solution: None,
        tries_left: None,
        expires: None
    }
}

impl ItemBuilder {
    pub fn uuid<T: ToString>(&mut self, uuid: T) -> &mut Self {
        self.uuid = Some(uuid.to_string());
        self
    }

    pub fn solution<T: ToString>(&mut self, solution: T) -> &mut Self {
        self.solution = Some(solution.to_string());
        self
    }

    pub fn tries_left(&mut self, tries_left: usize) -> &mut Self {
        self.tries_left = Some(tries_left);
        self
    }

    pub fn expires(&mut self, expires: Tm) -> &mut Self {
        self.expires = Some(expires);
        self
    }

    pub fn ttl(&mut self, ttl: i64) -> &mut Self {
        self.expires = Some(time::now() + Duration::seconds(ttl));
        self
    }

    pub fn item(&self) -> Result<Item, ()> {
        Ok(Item {
            uuid      : self.uuid.clone().ok_or(())?.clone(),
            solution  : self.solution.clone().ok_or(())?,
            tries_left: self.tries_left.ok_or(())?,
            expires   : self.expires.ok_or(())?.to_timespec().sec
        })
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Narrow {
    operator: String,
    operand: String,
    negated: bool,
}

impl Narrow {
    pub fn sender(user: String, negated: bool) -> Narrow {
        Narrow {
            operator: "sender".to_string(),
            operand: user,
            negated,
        }
    }

    pub fn stream(stream: String, negated: bool) -> Narrow {
        Narrow {
            operator: "stream".to_string(),
            operand: stream,
            negated,
        }
    }

    pub fn search(keyword: String, negated: bool) -> Narrow {
        Narrow {
            operator: "search".to_string(),
            operand: keyword,
            negated,
        }
    }

    pub fn pm_with(users: Vec<String>, negated: bool) -> Narrow {
        Narrow {
            operator: "pm-with".to_string(),
            operand: users.join(","),
            negated,
        }
    }

    pub fn near(id: String) -> Narrow {
        Narrow {
            operator: "near".to_string(),
            operand: id,
            negated: false,
        }
    }

    pub fn id(id: String, negated: bool) -> Narrow {
        Narrow {
            operator: "id".to_string(),
            operand: id,
            negated,
        }
    }

    pub fn public_streams(negated: bool) -> Narrow {
        Narrow {
            operator: "streams".to_string(),
            operand: "public".to_string(),
            negated,
        }
    }

    pub fn is(word: IsWords, negated: bool) -> Narrow {
        let operand = match word {
            IsWords::ALERTED => "alerted".to_string(),
            IsWords::MENTIONED => "mentioned".to_string(),
            IsWords::STARRED => "starred".to_string(),
            IsWords::UNREAD => "unread".to_string(),
            IsWords::PRIVATE => "private".to_string(),
        };
        Narrow {
            operator: "is".to_string(),
            operand,
            negated,
        }
    }

    pub fn has(word: HasWords, negated: bool) -> Narrow {
        let operand = match word {
            HasWords::ATTACHMENT => "attachment".to_string(),
            HasWords::LINK => "link".to_string(),
            HasWords::IMAGE => "image".to_string(),
        };
        Narrow {
            operator: "has".to_string(),
            operand,
            negated,
        }
    }

    pub fn group_pm_with(users: Vec<String>, negated: bool) -> Narrow {
        Narrow {
            operator: "group-pm-with".to_string(),
            operand: users.join(","),
            negated,
        }
    }
}

pub enum IsWords {
    ALERTED,
    MENTIONED,
    STARRED,
    UNREAD,
    PRIVATE,
}

pub enum HasWords {
    LINK,
    IMAGE,
    ATTACHMENT,
}

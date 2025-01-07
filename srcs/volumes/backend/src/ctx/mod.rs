mod error;

use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn demo1_ctx() -> Self {
        Ctx { user_id: 1000 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self {
                user_id: user_id.clone(),
            })
        }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

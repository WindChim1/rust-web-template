use common::{AppError, AppResult};
use salvo::{Depot, FlowCtrl, Request, Response, handler};

use crate::jwt::{CLAIMS, JWTONCELOCK};

#[handler]
async fn auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> AppResult<()> {
    let jwt_auth_util = JWTONCELOCK
        .get()
        .ok_or(AppError::Other("auth 获取失败".to_string()))?;
    let token = jwt_auth_util.extract_token(req)?;
    let claims = jwt_auth_util.verify_acc_token(&token)?;
    depot.insert(CLAIMS, claims);
    if ctrl.has_next() {
        ctrl.call_next(req, depot, res).await;
    }
    Ok(())
}

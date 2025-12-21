use common::AppResult;
use salvo::{Depot, FlowCtrl, Request, Response, handler};

use crate::jwt::{CLAIMS, JWTTool};

#[handler]
pub async fn auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> AppResult<()> {
    let jwt_auth_util = JWTTool::get()?;
    let token = jwt_auth_util.extract_token(req)?;
    let claims = jwt_auth_util.verify_acc_token(&token)?;
    depot.insert(CLAIMS, claims);
    if ctrl.has_next() {
        ctrl.call_next(req, depot, res).await;
    }
    Ok(())
}

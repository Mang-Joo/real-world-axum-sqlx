use std::sync::Arc;

use crate::app_state;
use crate::app_state::AppState;
use crate::user::application::repository::find_by_user_name;
use crate::user::domain::user::User;

pub async fn get_profile(
    app_state: Arc<AppState>,
    user_id: Option<i64>,
    user_name: String,
) -> app_state::Result<User> {
    let user = find_by_user_name(&user_name, &app_state.pool).await?;

    

    todo!()
}
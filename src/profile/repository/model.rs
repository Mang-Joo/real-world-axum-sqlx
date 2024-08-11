use chrono::NaiveDateTime;

pub struct UserFollowEntity {
    id: i64,
    follower_id: i64,
    following_id: i64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted: bool,
}

drop table user_follow;

create table user_follow
(
    id           BIGSERIAL primary key,
    follower_id  bigint    not null,
    following_id bigint    not null,
    created_at   timestamp not null,
    updated_at   time      null,
    deleted      boolean   null,
    constraint user_follow_users_id_fk
        foreign key (follower_id) references users (id),
    constraint user_follow_users_id_fk_2
        foreign key (following_id) references users (id)
);
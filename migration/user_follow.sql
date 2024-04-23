drop table user_follow;

create table user_follow
(
    id           bigint auto_increment
        primary key,
    follower_id  bigint     not null,
    following_id bigint     not null,
    created_at   datetime   not null,
    updated_at   datetime   null,
    deleted      tinyint(1) null,
    constraint user_follow_users_id_fk
        foreign key (follower_id) references users (id),
    constraint user_follow_users_id_fk_2
        foreign key (following_id) references users (id)
);


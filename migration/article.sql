drop table tag;

create table tag
(
    tag_name varchar(50) not null,
    constraint tag_pk
        primary key (tag_name)
);

create table article
(
    id          bigint auto_increment primary key,
    user_id     bigint               not null,
    title       varchar(100)         not null,
    slug        varchar(150)         not null,
    description varchar(500)         not null,
    body        varchar(500)         not null,
    created_at  datetime             not null,
    updated_at  datetime             not null,
    deleted     tinyint(1) default 0 not null,
    constraint article_pk
        unique (slug),
    constraint article_users_id_fk
        foreign key (user_id) references users (id)
);



create table article_tag
(
    id         bigint auto_increment,
    article_id bigint      not null,
    tag_name   varchar(50) not null,
    constraint article_tag_pk
        primary key (id),
    constraint article_tag_article_id_fk
        foreign key (article_id) references article (id),
    constraint article_tag_tag_tag_name_fk
        foreign key (tag_name) references tag (tag_name)
);

create table article_favorite
(
    id               bigint auto_increment,
    article_id       bigint not null,
    favorite_user_id bigint not null,
    constraint article_favorite_pk
        primary key (id),
    constraint article_favorite_article_id_fk
        foreign key (article_id) references article (id),
    constraint article_favorite_users_id_fk
        foreign key (favorite_user_id) references users (id)
);


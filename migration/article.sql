drop table tag;

create table tag
(
    tag_name varchar(50) not null,
    constraint tag_pk
        primary key (tag_name)
);

create table article
(
    id          bigint auto_increment,
    user_id     bigint                null,
    title       varchar(100)          null,
    slug        varchar(150)          null,
    description varchar(500)          null,
    body        varchar(500)          null,
    created_at  datetime              null,
    updated_at  datetime              null,
    deleted     boolean default false null,
    constraint article_pk
        primary key (id),
    constraint article_users_id_fk
        foreign key (user_id) references users (id)
);

create table article_tag
(
    id         bigint auto_increment,
    article_id bigint      null,
    tag_name   varchar(50) null,
    constraint article_tag_pk
        primary key (id),
    constraint article_tag_article_id_fk
        foreign key (article_id) references article (id),
    constraint article_tag_tag_tag_name_fk
        foreign key (tag_name) references tag (tag_name)
);


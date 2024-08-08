DROP TABLE users;

create table users
(
    id                BIGSERIAL PRIMARY KEY,
    username          varchar(100)            not null,
    email             VARCHAR(50)             not null,
    password          varchar(500)            not null,
    bio               varchar(500)            null,
    image             varchar(50)             null,
    registration_date timestamp default now() not null,
    modified_date     timestamp default now() not null,
    deleted           bool      default false not null
);

create index users_email_index
    on users (email);

create index users_username_index
    on users (username);
DROP TABLE users;

create table users
(
    id                BIGSERIAL PRIMARY KEY,
    user_name         varchar(100) not null,
    email             VARCHAR(50)  not null,
    password          varchar(500) not null,
    bio               varchar(500) null,
    image             varchar(50) null,
    registration_date timestamp    not null,
    modified_date     timestamp    not null,
    deleted           bool         not null
);

create index users_email_index
    on users (email);

create index users_username_index
    on users (user_name);
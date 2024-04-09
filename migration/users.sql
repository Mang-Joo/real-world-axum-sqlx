create table users
(
    id        BIGINT auto_increment,
    email     VARCHAR(50)  not null,
    user_name varchar(100) not null,
    bio       varchar(500) null,
    image     varchar(50)  null,
    password  varchar(500) not null,
    constraint users_pk
        primary key (id)
);

create index users_email_index
    on users (email);


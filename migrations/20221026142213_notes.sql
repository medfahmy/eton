create table notes (
    id serial primary key,
    title varchar(64) not null,
    content text not null
);

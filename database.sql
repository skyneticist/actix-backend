drop table if exists projects;

drop table if exists books;

drop table if exists albums;

drop table if exists videos;

create table project_list (
    id serial primary key,
    title varchar(150) not null
);

create table project_details (
    id serial primary key,
    title varchar(150) not null,
    in_progress boolean not null,
    project_id integer not null
);

create table projects (
    id serial primary key,
    title varchar(250) not null,
    authors varchar(300) not null,
    in_progress boolean not null,
    project_id integer not null,
    category varchar(100) not null,
    external_url varchar(250),
    favorite boolean not null,
    monetized boolean not null
); 

create table books (
    id serial primary key,
    title varchar(150) not null,
    author varchar(150) not null,
    release_date varchar(150) not null,
    publisher varchar(150) not null
);

create table albums (
    id serial primary key,
    title varchar(150) not null,
    artist varchar(150) not null,
    release_date varchar(150) not null,
    label varchar(150) not null
);

create table videos (
    id serial primary key,
    title varchar(150) not null,
    video_url varchar(250) not null,
    favorite boolean not null,
    channel varchar(250),
    category varchar(150) not null,
    video_rank integer
);

insert into
    project_details (title, in_progress, project_id)
values
    ('ROM COM', true, 1),
    ('LG OLED AUTODIM', false, 2),
    ('CAT LASER DS4 CONTROL', true, 3);

insert into
    books (title, author, release_date, publisher)
values
    (
        'Zen and the Art of Motorcycle Maintenance',
        'Michael Persig',
        '1979',
        'unknown'
    );

insert into
    albums (title, artist, release_date, label)
values
    (
        'Flying Microtonal Banana',
        'King Gizzard & The Lizard Wizard',
        '2018',
        'notsure'
    );

insert into
    project_list (title)
values
    ('ROM COM'),
    ('LG OLED AUTODIM'),
    ('CAT LASER DS4 CONTROL');

insert into
    albums (
        title,
        artist,
        release_date,
        label
    )
values
    (
        'Deloused in the Comatorium',
        'The Mars Volta',
        '2003',
        'hmm'
    );

-- run this cat command to run this sql script against running psql docker container
-- (generic form) cat ./query.sql | docker exec -i <container-name> psql -U <user> -d <database> 
-- my specific command:
-- cat ./database.sql | docker exec -i actix-backend_postgres_1 psql -U actix -d actix
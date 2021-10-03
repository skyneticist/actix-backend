drop table if exists project_list;
drop table if exists project_details;

create table project_list (
    id serial primary key,
    title varchar(150) not null
);

create table project_details (
    id serial primary key,
    title varchar(150) not null, 
    in_progress boolean not null,
    list_id integer not null,
    foreign key (list_id) references project_list(id)
);

insert into project_list (title) values ('ROM COM'), ('LG OLED AUTODIM'), ('CAT LASER DS4 CONTROL');

insert into project_details (title, in_progress, list_id) values ('ROM COM', true, 1), ('LG OLED AUTODIM', false, 2), ('CAT LASER DS4 CONTROL', true, 3);
drop table if exists project_details;
drop table if exists project_list;

create table project_list (
    id serial primary key,
    title varchar(150) not null
);

create table project_details (
    id serial primary key,
    title varchar(150) not null, 
    in_progress boolean not null,
    project_id integer not null,
    foreign key (project_id) references project_list(id)
);

insert into project_list (title) values ('ROM COM'), ('LG OLED AUTODIM'), ('CAT LASER DS4 CONTROL');

insert into project_details (title, in_progress, project_id) values ('ROM COM', true, 1), ('LG OLED AUTODIM', false, 2), ('CAT LASER DS4 CONTROL', true, 3);


-- run this cat command to run this sql script against running psql docker container
-- (generic form) cat ./query.sql | docker exec -i <container-name> psql -U <user> -d <database> 

-- my specific command:
-- cat ./database.sql | docker exec -i actix-backend_postgres_1 psql -U actix -d actix

# script to quickly and cleanly start up backend with dockerized database
# ./run.sh

### db
## build and run psql container
./spinner.sh docker-compose down
./spinner.sh docker-compose build
./spinner.sh docker-compose up -d
echo 🎃 database is up

## wait for docker db to be up and healthy
echo waiting a bit on container
./spinner.sh sleep 4

## initialize our db environment
echo 🎃 populating DB!!
./spinner.sh cat ./database.sql | docker exec -i actix-backend_postgres_1 psql -U actix -d actix > sql_dump_info.txt
echo 🎃 database populated!


### actix
## build actix server executable
echo 🦀 building executable from Rust code...
./spinner.sh cargo build
echo 🎃 executable successfully built!

## run fresh actix-backend executable
echo 👻 running executable backend...
./target/debug/actix-backend.exe

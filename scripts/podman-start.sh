podman run -d --name oapth_mariadb -e MYSQL_DATABASE=oapth -e MYSQL_PASSWORD=oapth -e MYSQL_USER=oapth -e MYSQL_ROOT_PASSWORD=123456 -p 3306:3306 docker.io/library/mariadb:10
podman run -d --name oapth_mssql -e 'ACCEPT_EULA=Y' -e 'SA_PASSWORD=yourStrong(!)Password' -p 1433:1433 mcr.microsoft.com/mssql/server:2019-CU15-ubuntu-20.04
podman run -d --name oapth_mysql -e MYSQL_DATABASE=oapth -e MYSQL_PASSWORD=oapth -e MYSQL_USER=oapth -e MYSQL_ROOT_PASSWORD=123456 -p 3307:3306 docker.io/library/mysql:8
podman run -d --name oapth_pg -e POSTGRES_DB=oapth -e POSTGRES_PASSWORD=oapth -e POSTGRES_USER=oapth -p 5432:5432 docker.io/library/postgres:13

# Utils

# podman exec -it oapth_mariadb mysql -uoapth -poapth -Doapth
# podman exec -it oapth_mssql /opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P 'yourStrong(!)Password'
# podman exec -it oapth_mysql mysql -uoapth -poapth -Doapth
# podman exec -it oapth_pg psql -h localhost -U oapth
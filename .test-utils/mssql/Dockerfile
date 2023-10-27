FROM mcr.microsoft.com/mssql/server:2022-CU9-ubuntu-20.04

# Create a config directory
WORKDIR /etc

# Bundle config source
COPY mssql/entrypoint.sh /etc/entrypoint.sh
COPY mssql/configure-db.sh /etc/configure-db.sh
COPY mssql/setup.sql /etc/setup.sql

# Grant permissions for to our scripts to be executable
USER root
RUN chmod +x /etc/entrypoint.sh
RUN chmod +x /etc/configure-db.sh
RUN chown 10001 /etc/entrypoint.sh
RUN chown 10001 /etc/configure-db.sh
USER 10001

ENTRYPOINT ["/etc/entrypoint.sh"]
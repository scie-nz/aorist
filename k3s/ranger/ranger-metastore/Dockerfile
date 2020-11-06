FROM scienz/aorist-test-ranger:latest

FROM scienz/aorist-test-base-image:latest
COPY --from=0 /ranger/target/ranger-2.1.0-hive-plugin.tar.gz .
RUN tar xvf ranger-2.1.0-hive-plugin.tar.gz

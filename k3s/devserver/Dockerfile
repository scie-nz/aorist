FROM scienz/aorist-test-base-image:latest

WORKDIR /usr/bin
RUN apt-get update && apt-get install -y wget
RUN wget https://repo1.maven.org/maven2/io/prestosql/presto-cli/344/presto-cli-344-executable.jar
RUN chmod a+rwx presto-cli-344-executable.jar
RUN ln -s presto-cli-344-executable.jar presto-cli
RUN echo 'presto-cli --server presto-coordinator-headless:8080 --catalog hive --schema $1' > /usr/bin/presto
RUN chmod a+rwx /usr/bin/presto
RUN apt-get -y install openjdk-11-jre
RUN apt-get -y install less
RUN pip3 install alluxio google-cloud-storage

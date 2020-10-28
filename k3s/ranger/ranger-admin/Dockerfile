FROM scienz/aorist-test-ranger:latest

FROM scienz/aorist-test-base-image:latest
COPY --from=0 /ranger/target/ranger-2.1.0-admin.tar.gz .
RUN tar xvf ranger-2.1.0-admin.tar.gz

ENV HADOOP_VERSION=3.2.0
ENV METASTORE_VERSION=3.0.0
ENV HADOOP_HOME=/opt/hadoop-${HADOOP_VERSION}
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get -y install openjdk-8-jre mysql-client-core-8.0
# TODO: cloud sql proxy should be set up in separate docker image

RUN mkdir -p $HADOOP_HOME && chmod a+rw $HADOOP_HOME
RUN curl -L https://archive.apache.org/dist/hadoop/common/hadoop-${HADOOP_VERSION}/hadoop-${HADOOP_VERSION}.tar.gz | tar zxf -
RUN curl -L https://dev.mysql.com/get/Downloads/Connector-J/mysql-connector-java-8.0.19.tar.gz | tar zxf -

RUN mv hadoop-${HADOOP_VERSION}/* $HADOOP_HOME
ENV JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64
# RUN mv mysql-connector-java-8.0.19/mysql-connector-java-8.0.19.jar /usr/share/java/mysql-connector-java.jar
# RUN test -f "/usr/share/java/mysql-connector-java.jar"

RUN mkdir -p /usr/lib/ranger/
RUN mv ranger-2.1.0-admin /usr/lib/ranger/
# ENV HADOOP_CLASSPATH=$HADOOP_CLASSPATH:$HIVE_HOME/lib/mysql-connector-java-8.0.19.jar
# RUN rm -rf  mysql-connector-java-8.0.19
COPY entrypoint.sh /
WORKDIR /usr/lib/ranger/ranger-2.1.0-admin/
COPY install.properties .
RUN apt-get install -y bc lsb-release
COPY setup.sh .
COPY setup_kube.sh .
RUN ./setup.sh

EXPOSE 6080
ENTRYPOINT ["/entrypoint.sh"]

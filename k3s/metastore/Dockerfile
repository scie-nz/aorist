FROM scienz/aorist-test-base-image:latest
ENV HADOOP_VERSION=3.2.0
ENV METASTORE_VERSION=3.0.0
ENV HADOOP_HOME=/opt/hadoop-${HADOOP_VERSION}
ENV HIVE_HOME=/opt/apache-hive-metastore-${METASTORE_VERSION}-bin
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get -y install openjdk-8-jre mysql-client-core-8.0
# TODO: cloud sql proxy should be set up in separate docker image

RUN mkdir -p $HIVE_HOME && chmod a+rw $HIVE_HOME && mkdir -p $HIVE_HOME/lib \
  && chmod a+rw $HIVE_HOME/lib && mkdir -p $HADOOP_HOME && chmod a+rw $HADOOP_HOME
RUN curl -L ftp://ftp.funet.fi/pub/mirrors/apache.org/hive/hive-standalone-metastore-3.0.0/hive-standalone-metastore-3.0.0-bin.tar.gz | tar zxf -
RUN curl -L https://archive.apache.org/dist/hadoop/common/hadoop-${HADOOP_VERSION}/hadoop-${HADOOP_VERSION}.tar.gz | tar zxf -
RUN curl -L https://dev.mysql.com/get/Downloads/Connector-J/mysql-connector-java-8.0.19.tar.gz | tar zxf -
RUN cp mysql-connector-java-8.0.19/mysql-connector-java-8.0.19.jar ${HIVE_HOME}/lib/

RUN rm -rf  mysql-connector-java-8.0.19
RUN mv hadoop-${HADOOP_VERSION}/* $HADOOP_HOME
ENV JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64
ENV HADOOP_CLASSPATH=$HADOOP_CLASSPATH:$HIVE_HOME/lib/mysql-connector-java-8.0.19.jar

ENV METASTORE_AUX_JARS_PATH=/alluxio-2.4.0/client/alluxio-2.4.0-client.jar

WORKDIR apache-hive-metastore-3.0.0-bin

COPY entrypoint.sh /
EXPOSE 9083
ENTRYPOINT ["/entrypoint.sh"]

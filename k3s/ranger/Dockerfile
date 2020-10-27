FROM scienz/aorist-test-base-image:latest
run apt-get update && apt-get -y install openjdk-8-jdk
RUN apt-get update && apt-get -y install git
RUN git clone --depth 1 --branch release-ranger-2.1.0 https://github.com/apache/ranger
WORKDIR ranger
ENV JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/
RUN apt-get update && apt-get -y install maven
RUN mvn -DskipTests=false clean package

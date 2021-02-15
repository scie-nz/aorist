FROM scienz/aorist-test-base-image:latest
run apt-get update && apt-get -y install openjdk-8-jdk
RUN apt-get update && apt-get -y install git
RUN git clone --depth 1 --branch release-ranger-2.1.0 https://github.com/apache/ranger
WORKDIR ranger
ENV JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/
RUN apt-get update && apt-get -y install maven
RUN apt-get update && apt-get -y install python3
RUN apt-get update && apt-get -y install python3-pip
RUN pip3 install requests
RUN ln -s /usr/bin/python3 /usr/bin/python
RUN mvn -DskipTests=false clean package

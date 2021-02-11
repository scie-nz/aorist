FROM scienz/aorist-test-base-image:latest

WORKDIR /usr/bin
RUN apt-get update && apt-get install -y wget
RUN apt-get -y upgrade
RUN wget https://repo1.maven.org/maven2/io/prestosql/presto-cli/344/presto-cli-344-executable.jar
RUN chmod a+rwx presto-cli-344-executable.jar
RUN ln -s presto-cli-344-executable.jar presto-cli
RUN echo 'presto-cli --server presto-coordinator-headless:8080 --catalog hive --schema $1' > /usr/bin/presto
RUN chmod a+rwx /usr/bin/presto
RUN apt-get -y install openjdk-11-jre
RUN apt-get -y install less
RUN apt-get -y install python3-pip
RUN pip3 install alluxio google-cloud-storage
RUN apt-get -y install vim
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN apt-get -y install git unzip
RUN pip3 install astor black
WORKDIR /home
COPY entrypoint.sh .
RUN echo "export GIT_SSH_COMMAND='ssh -i /etc/deploy_key/ssh-privatekey'" >> /root/.bashrc
ENTRYPOINT ["/home/entrypoint.sh"]

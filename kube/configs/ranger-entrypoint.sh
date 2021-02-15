export JAVA_HOME=/usr/lib/jvm/zre-11-amd64
cd /presto-server-344
mv /presto-server-344/etc/ranger-2.1.0-presto-plugin.tar.gz .
tar xvf ranger-2.1.0-presto-plugin.tar.gz
cp etc/ranger-presto-install.properties ranger-2.1.0-presto-plugin/install.properties
cd ranger-2.1.0-presto-plugin
./enable-presto-plugin.sh
cd ..
../entrypoint.sh

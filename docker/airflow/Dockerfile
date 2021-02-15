# Apache Airflow image, works across amd64 and arm64.
# Build with:
#   docker buildx build --platform linux/amd64,linux/arm64 -t registry.gitlab.com/nickbp/k8s/airflow:$(date +%Y%m%d) .

FROM debian:buster

# Install prerequisites for installing base airflow package, with support for databases/sasl/ldap etc included.
# See also: https://airflow.apache.org/docs/apache-airflow/stable/installation.html
RUN apt-get update \
  && apt-get dist-upgrade -y \
  && apt-get install -y \
    curl \
    cython3 \
    freetds-dev \
    git \
    libffi-dev \
    libkrb5-dev \
    libldap2-dev \
    libmariadb-dev-compat \
    libpq-dev \
    libsasl2-dev \
    libssl-dev \
    python3-pip \
    unixodbc-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

ENV AIRFLOW_VERSION=2.0.0

# Install base airflow package:
# - Get constraints file for use in pip install calls.
# - Avoid pendulum 2.1.2, broken on arm: https://github.com/sdispater/pendulum/issues/504
# - Base image is currently python 3.7.x so get that version of the constraints
# See also (in addition to constraints.txt): https://github.com/apache/airflow/blob/2.0.0/setup.cfg
RUN curl -L "https://raw.githubusercontent.com/apache/airflow/constraints-${AIRFLOW_VERSION}/constraints-3.7.txt" | sed 's/pendulum==2.1.2/pendulum==2.1.1/g' > /tmp/airflow-constraints.txt

# Break out slow numpy build, used by base airflow (2400s arm64)
RUN pip3 install --no-cache-dir numpy --constraint /tmp/airflow-constraints.txt

# Break out slow pandas build, used by base airflow (7000s arm64)
RUN pip3 install --no-cache-dir pandas --constraint /tmp/airflow-constraints.txt

# Install base airflow package, as well as python-ldap for airflow ldap auth support (1090s arm64)
RUN pip3 install --no-cache-dir apache-airflow==${AIRFLOW_VERSION} python-ldap --constraint /tmp/airflow-constraints.txt

# Break out slow grpcio build (3571s arm64, 530s amd64)
RUN pip3 install --no-cache-dir grpcio --constraint /tmp/airflow-constraints.txt

# Break out 200MB+ download (100s amd64, 130s arm64)
RUN pip3 install --no-cache-dir pyspark --constraint /tmp/airflow-constraints.txt

# Install pyarrow and its many build dependencies. (2555s arm64)
# pyarrow is used by apache.beam, google(google-cloud-bigquery-storage), and papermill(requirements-hdfs.txt)
RUN curl -L -o arrow-keyring.deb https://apache.bintray.com/arrow/debian/apache-arrow-archive-keyring-latest-buster.deb \
  && apt-get install -y -V ./arrow-keyring.deb \
  && rm -fv arrow-keyring.deb \
  && apt-get update \
  && apt-get install -y --no-install-recommends cmake libarrow-dev libarrow-glib-dev libarrow-dataset-dev libarrow-python-dev libparquet-dev libparquet-glib-dev libbrotli-dev liblz4-dev libzstd-dev libbz2-dev libutf8proc-dev libsnappy-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives \
  && pip3 install --no-cache-dir pyarrow --constraint /tmp/airflow-constraints.txt

# Install nearly all airflow addon packages.
# Lists the packages individually so that we can omit individual packages that have issues.
# Broken down into chunks of roughly ten items, since even amd64 takes several minutes to install all of these, while emulated arm64 can take several hours.
# For the list of available packages, see "EXTRAS_PROVIDERS_PACKAGES": https://github.com/apache/airflow/blob/2.0.0/setup.py#L696

# 4222s arm64, 688s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
amazon,\
apache.atlas,\
apache.beam,\
apache.cassandra,\
apache.druid,\
apache.hdfs,\
apache.hive,\
apache.kylin,\
apache.livy\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 2378s arm64, 450s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
apache.pig,\
apache.pinot,\
apache.spark,\
apache.webhdfs,\
async,\
celery,\
cgroups,\
cloudant,\
cncf.kubernetes\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 563s arm64, 180s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
dask,\
databricks,\
datadog,\
dingding,\
discord,\
docker,\
elasticsearch,\
exasol,\
facebook,\
ftp\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 726s arm64, 56s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
github_enterprise,\
google,\
google_auth,\
grpc,\
hashicorp,\
http,\
imap,\
jdbc,\
jenkins,\
jira\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 534s arm64, 80s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
kerberos,\
ldap,\
microsoft.azure,\
microsoft.mssql,\
microsoft.winrm,\
mongo,\
mysql,\
odbc,\
openfaas\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 2057s arm64, 67s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
opsgenie,\
oracle,\
pagerduty,\
papermill,\
password,\
plexus,\
postgres,\
presto,\
qubole\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 2407s arm64, 55s amd64
RUN pip3 install --no-cache-dir apache-airflow[\
rabbitmq,\
redis,\
salesforce,\
samba,\
segment,\
sendgrid,\
sentry,\
sftp,\
singularity\
]==${AIRFLOW_VERSION}

# 154s arm64, 49s amd64
# Skips snowflake for now because:
# - snowflake-connector-python pulls in an old version of pyarrow that fails to build on arm64
# - the snowflake module's dependencies in airflow's setup.py are a mess anyway.
# In any case, the build fails with:
#   CMake Error at cmake_modules/SetupCxxFlags.cmake:338 (message):
#     Unsupported arch flag: -march=.
#   Call Stack (most recent call first):
#     CMakeLists.txt:100 (include)
# Possible fix could be setting PYARROW_CMAKE_OPTIONS="-DARROW_ARMV8_ARCH=armv8 -DCXX_SUPPORTS_ARMV8_ARCH=true"? But then there's still the issue of any conflicts with pyarrow 2.0.0.
# See also https://forums.developer.nvidia.com/t/building-apache-arrow-with-cuda-on-jetsons/158312/2
RUN pip3 install --no-cache-dir apache-airflow[\
slack,\
ssh,\
statsd,\
tableau,\
telegram,\
vertica,\
virtualenv,\
yandex,\
zendesk\
]==${AIRFLOW_VERSION} --constraint /tmp/airflow-constraints.txt

# 29865s total arm64

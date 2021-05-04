# Builds an image containing phabricator and prerequisites
# Build with:
#   docker buildx build --push --platform linux/amd64,linux/arm64 -t docker.io/scienz/phabricator:$(date +%Y%m%d) .
FROM debian:buster

# Install PHP requirements for running phabricator, along with extra tools that it uses
RUN apt-get update \
  && apt-get dist-upgrade -y \
  && apt-get install -y \
    php php-apcu php-cli php-curl php-fpm php-gd php-json php-ldap php-mbstring php-mysql php-zip \
    git mercurial subversion \
    imagemagick python3-pygments \
    mariadb-client procps gpg \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

# Download phabricator itself along with required arcanist
RUN cd /opt \
  && git clone --depth 1 https://github.com/phacility/arcanist.git \
  && git clone --depth 1 https://github.com/phacility/phabricator.git

FROM ubuntu:16.10

RUN apt-get update
RUN apt-get -y install apt-utils
RUN apt-get -y install curl aptitude build-essential
RUN apt-get -y install git

RUN apt-get -y install libmagickwand-dev

ADD run.sh /tmp/
CMD ["/bin/bash", "-c", "/tmp/run.sh"]

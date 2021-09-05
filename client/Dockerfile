FROM ubuntu:latest
RUN apt update && apt upgrade -y
RUN apt install -y nodejs
RUN apt install -y npm
RUN npm install --global -y yarn
COPY . ./client
WORKDIR ./client
RUN yarn && yarn build
VOLUME build:build
RUN cp -r build/* /var/www/html

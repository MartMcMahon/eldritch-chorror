FROM ubuntu:latest
RUN apt update && apt upgrade -y
RUN apt install -y nodejs
RUN apt install -y npm
RUN npm install --global -y yarn
RUN apt install -y nginx
COPY . ./client
WORKDIR ./client
RUN yarn && yarn build
EXPOSE 80
RUN cp -r build/* /var/www/html

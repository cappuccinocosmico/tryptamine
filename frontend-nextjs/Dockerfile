# BIG UPDATE: Changed the root image and source of the configuration files for deployment
FROM node:22.4.1-alpine3.20 as BUILD_IMAGE
WORKDIR /app
COPY ./package.json ./  
COPY ./package-lock.json ./    
# install dependencies
RUN npm install --force
COPY ./tsconfig.json ./
COPY . .
# build
RUN npm run build
FROM node:22.4.1-alpine3.20
WORKDIR /app
# copy from build image
COPY --from=BUILD_IMAGE /app/package.json ./package.json
COPY --from=BUILD_IMAGE /app/node_modules ./node_modules
COPY --from=BUILD_IMAGE /app/.next ./.next
COPY --from=BUILD_IMAGE /app/public ./public
COPY --from=BUILD_IMAGE /app/tsconfig.json ./
# COPY --from=BUILD_IMAGE postcss.config.js ./
# COPY --from=BUILD_IMAGE tailwind.config.js ./
COPY ./tailwind.config.js ./
COPY ./postcss.config.js ./
# FIXME: Bad Security, I am currently using a volume for this, this would push the .env.local file to our public dockerrepo
# COPY ./.env.local ./
COPY ./tsconfig.json ./

EXPOSE 3000

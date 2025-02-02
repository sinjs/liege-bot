FROM node:22-alpine AS builder

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

# FIXME: Bad workaround
ENV COREPACK_INTEGRITY_KEYS=0

RUN corepack enable
WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN pnpm install
COPY . .
RUN pnpm build

FROM nginx:alpine

WORKDIR /usr/share/nginx/
RUN rm -rf html && mkdir html

WORKDIR /

COPY ./nginx.conf /etc/nginx/
COPY --from=builder /app/dist /usr/share/nginx/html

ENTRYPOINT ["nginx", "-g", "daemon off;"]
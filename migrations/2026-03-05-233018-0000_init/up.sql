CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(100) NOT NULL UNIQUE,
    password    VARCHAR(255) NOT NULL,
    role        VARCHAR(20)  NOT NULL DEFAULT 'user',
    scopes      TEXT[]       NOT NULL DEFAULT ARRAY[]::TEXT[],
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE certificates (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL UNIQUE,
    type            VARCHAR(20)  NOT NULL, -- 'letsencrypt' | 'custom'
    certificate     TEXT         NOT NULL,
    private_key     TEXT         NOT NULL,
    dns_provider    VARCHAR(50),           -- for DNS-01 challenges
    expires_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE upstreams (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL UNIQUE,
    schema      VARCHAR(5)   NOT NULL DEFAULT 'http' CHECK (schema IN ('http', 'https')),
    host        VARCHAR(255) NOT NULL,
    port        INTEGER      NOT NULL CHECK (port BETWEEN 1 AND 65535),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE access_lists (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL UNIQUE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE access_list_rules (
    id              SERIAL PRIMARY KEY,
    access_list_id  INTEGER      NOT NULL REFERENCES access_lists(id) ON DELETE CASCADE,
    action          VARCHAR(5)   NOT NULL CHECK (action IN ('allow', 'deny')),
    address         VARCHAR(50)  NOT NULL, -- e.g. '192.168.1.0/24', '10.0.0.1', 'all'
    sort_order      INTEGER      NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE proxy_hosts (
    id                  SERIAL PRIMARY KEY,
    domain              VARCHAR(255)    NOT NULL UNIQUE,
    certificate_name    VARCHAR(255)    REFERENCES certificates(name),
    access_list_id      INTEGER         REFERENCES access_lists(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE TABLE proxy_host_upstreams (
    proxy_host_id   INTEGER NOT NULL REFERENCES proxy_hosts(id) ON DELETE CASCADE,
    upstream_id     INTEGER NOT NULL REFERENCES upstreams(id) ON DELETE RESTRICT,
    PRIMARY KEY (proxy_host_id, upstream_id)
);

CREATE INDEX idx_access_list_rules_list_order
    ON access_list_rules(access_list_id, sort_order);
-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_access_list_rules_list_order;

DROP TABLE IF EXISTS proxy_host_upstreams;

DROP TABLE IF EXISTS proxy_hosts;

DROP TABLE IF EXISTS access_list_rules;

DROP TABLE IF EXISTS access_lists;

DROP TABLE IF EXISTS upstreams;

DROP TABLE IF EXISTS certificates;

DROP TABLE IF EXISTS users;
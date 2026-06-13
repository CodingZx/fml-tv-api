CREATE EXTENSION pg_trgm;

CREATE OR REPLACE FUNCTION "split_12"(text)
  RETURNS "pg_catalog"."_text" AS $BODY$
declare
res text[];
begin
select regexp_split_to_array($1, '') into res;
for i in 1..length($1)-1 loop
    res := array_append(res, substring($1, i, 2));
end loop;
return res;
end;
$BODY$
LANGUAGE plpgsql IMMUTABLE STRICT COST 100;



CREATE TABLE "sys_account" (
    "id" uuid NOT NULL,
    "username" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
    "real_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
    "password" text COLLATE "pg_catalog"."default" NOT NULL,
    "status" bool NOT NULL,
    "super_admin" bool NOT NULL,
    "create_time" timestamp(6) NOT NULL,
    "create_user" uuid NOT NULL,
    "update_time" timestamp(6) NOT NULL,
    "update_user" uuid NOT NULL,
    "version" int4 NOT NULL,
    "deleted" bool NOT NULL DEFAULT false,
    "delete_time" timestamp(6) NOT NULL DEFAULT '2000-01-01 00:00:00'::timestamp without time zone,
    "delete_user" uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    "deleted_reason" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::text,
    CONSTRAINT "sys_account_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "sys_account_user_name_u_idx" ON "sys_account" USING btree (
    "username" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
) WHERE deleted = false;

COMMENT ON COLUMN "sys_account"."username" IS '账号';
COMMENT ON COLUMN "sys_account"."real_name" IS '姓名';
COMMENT ON COLUMN "sys_account"."password" IS '密码';
COMMENT ON COLUMN "sys_account"."status" IS '状态';
COMMENT ON COLUMN "sys_account"."super_admin" IS '是否超管';
COMMENT ON COLUMN "sys_account"."create_time" IS '创建时间';
COMMENT ON COLUMN "sys_account"."create_user" IS '创建人';
COMMENT ON COLUMN "sys_account"."update_time" IS '修改时间';
COMMENT ON COLUMN "sys_account"."update_user" IS '修改人';
COMMENT ON COLUMN "sys_account"."version" IS '版本号';
COMMENT ON COLUMN "sys_account"."deleted" IS '是否删除';
COMMENT ON COLUMN "sys_account"."delete_time" IS '删除时间';
COMMENT ON COLUMN "sys_account"."delete_user" IS '删除人';
COMMENT ON COLUMN "sys_account"."deleted_reason" IS '删除原因';

INSERT INTO "sys_account" ("id", "username", "real_name", "password", "status", "super_admin", "create_time", "create_user", "update_time", "update_user", "version", "deleted", "delete_time", "delete_user", "deleted_reason") VALUES ('00000000-0000-0000-0000-000000000001', 'admin', '超级管理员', '0L05IGCUa+fhxG9xbqJCt6761d292oInrGy7GVTjQWe+bpXGwaMrghYAznsX2awIKyS5KGncko5Ch2IZDmSv3p8JfYY1guRMK6dXesDq160IRsfLayxGsVrv6ECpndcsnLfHjrSuWrxGFJp7hM/XJiBfZHuHVAFz+in6QGc//sXJQJwJ0Q==', 't', 't', '2025-10-20 22:36:58.7595', '00000000-0000-0000-0000-000000000001', '2026-06-13 19:52:27.265056', '00000000-0000-0000-0000-000000000001', 3, 'f', '2000-01-01 00:00:00', '00000000-0000-0000-0000-000000000000', '');


CREATE UNLOGGED TABLE "sys_cache" (
  "id" uuid NOT NULL,
  "key" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "cache" text COLLATE "pg_catalog"."default" NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "expire_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_cache_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "sys_cache_u_idx" ON "sys_cache" USING btree (
  "key" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "sys_cache"."key" IS '缓存KEY';
COMMENT ON COLUMN "sys_cache"."cache" IS '缓存值';
COMMENT ON COLUMN "sys_cache"."create_time" IS '创建时间';
COMMENT ON COLUMN "sys_cache"."expire_time" IS '过期时间';


CREATE TABLE "sys_config" (
  "id" uuid NOT NULL,
  "key" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "conf_value" text COLLATE "pg_catalog"."default" NOT NULL,
  CONSTRAINT "sys_config_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "sys_config_u_idx" ON "sys_config" USING btree (
  "key" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "sys_config"."key" IS '配置KEY';
COMMENT ON COLUMN "sys_config"."conf_value" IS '配置值';


CREATE UNLOGGED TABLE "sys_counter" (
  "id" uuid NOT NULL,
  "key" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "counter" int8 NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "expire_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_counter_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "sys_counter_u_idx" ON "sys_counter" USING btree (
  "key" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "sys_counter"."key" IS '缓存KEY';
COMMENT ON COLUMN "sys_counter"."counter" IS '计数器';
COMMENT ON COLUMN "sys_counter"."create_time" IS '创建时间';
COMMENT ON COLUMN "sys_counter"."expire_time" IS '过期时间';



CREATE TABLE "sys_lock" (
  "id" uuid NOT NULL,
  "lock_key" text COLLATE "pg_catalog"."default" NOT NULL,
  "lock_value" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "expire_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_lock_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "sys_lock_u_idx" ON "sys_lock" USING btree (
  "lock_key" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "sys_lock"."lock_key" IS '锁Key';
COMMENT ON COLUMN "sys_lock"."lock_value" IS '锁值';
COMMENT ON COLUMN "sys_lock"."create_time" IS '创建时间';
COMMENT ON COLUMN "sys_lock"."expire_time" IS '过期时间';

CREATE TABLE "sys_login_log" (
  "id" uuid NOT NULL,
  "ip_addr" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "user_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "password" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_login_log_pkey" PRIMARY KEY ("id")
);

COMMENT ON COLUMN "sys_login_log"."ip_addr" IS '请求IP';
COMMENT ON COLUMN "sys_login_log"."user_name" IS '用户名';
COMMENT ON COLUMN "sys_login_log"."password" IS '密码';
COMMENT ON COLUMN "sys_login_log"."create_time" IS '创建时间';

CREATE TABLE "sys_oper_log" (
  "id" uuid NOT NULL,
  "title" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "business_type" int4 NOT NULL,
  "req_method" varchar(10) COLLATE "pg_catalog"."default" NOT NULL,
  "uri" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "exec_time" int8 NOT NULL,
  "req_ip" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "req_param" text COLLATE "pg_catalog"."default" NOT NULL,
  "success" bool NOT NULL,
  "err_msg" text COLLATE "pg_catalog"."default" NOT NULL,
  "oper_user" uuid,
  "create_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_oper_log_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "sys_oper_log_idx" ON "sys_oper_log" USING btree (
  "create_time" "pg_catalog"."timestamp_ops" DESC NULLS FIRST
);

COMMENT ON COLUMN "sys_oper_log"."title" IS '标题';
COMMENT ON COLUMN "sys_oper_log"."business_type" IS '业务类型';
COMMENT ON COLUMN "sys_oper_log"."req_method" IS '请求方式';
COMMENT ON COLUMN "sys_oper_log"."uri" IS '请求URI';
COMMENT ON COLUMN "sys_oper_log"."exec_time" IS '执行时间(ms)';
COMMENT ON COLUMN "sys_oper_log"."req_ip" IS '请求IP';
COMMENT ON COLUMN "sys_oper_log"."req_param" IS '参数';
COMMENT ON COLUMN "sys_oper_log"."success" IS '是否执行成功';
COMMENT ON COLUMN "sys_oper_log"."err_msg" IS '错误信息';
COMMENT ON COLUMN "sys_oper_log"."oper_user" IS '操作人';
COMMENT ON COLUMN "sys_oper_log"."create_time" IS '创建时间';


CREATE TABLE "sys_queue_delay_msg" (
  "id" uuid NOT NULL,
  "queue" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "message" text COLLATE "pg_catalog"."default" NOT NULL,
  "delay_time" timestamp(6) NOT NULL,
  CONSTRAINT "sys_queue_delay_msg_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "sys_queue_delay_msg_idx" ON "sys_queue_delay_msg" USING btree (
  "delay_time" "pg_catalog"."timestamp_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "sys_queue_delay_msg"."queue" IS '队列名称';
COMMENT ON COLUMN "sys_queue_delay_msg"."message" IS '消息内容';
COMMENT ON COLUMN "sys_queue_delay_msg"."delay_time" IS '延迟时间';

CREATE TABLE "sys_queue_msg" (
  "id" uuid NOT NULL,
  "queue" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "message" text COLLATE "pg_catalog"."default" NOT NULL,
  "status" int4 NOT NULL,
  "publish_time" timestamp(6) NOT NULL,
  "process_time" timestamp(6) NOT NULL,
  "finish_time" timestamp(6) NOT NULL,
  "error_detail" text COLLATE "pg_catalog"."default" NOT NULL,
  CONSTRAINT "sys_queue_msg_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "sys_queue_msg_idx" ON "sys_queue_msg" USING btree (
  "queue" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST,
  "publish_time" "pg_catalog"."timestamp_ops" ASC NULLS LAST
) WHERE status = 0;

COMMENT ON COLUMN "sys_queue_msg"."queue" IS '队列名称';
COMMENT ON COLUMN "sys_queue_msg"."message" IS '消息内容';
COMMENT ON COLUMN "sys_queue_msg"."status" IS '状态';
COMMENT ON COLUMN "sys_queue_msg"."publish_time" IS '发布时间';
COMMENT ON COLUMN "sys_queue_msg"."process_time" IS '处理时间';
COMMENT ON COLUMN "sys_queue_msg"."finish_time" IS '完成时间';
COMMENT ON COLUMN "sys_queue_msg"."error_detail" IS '错误详情';


CREATE TABLE "tv_group" (
  "id" uuid NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "types" jsonb NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "create_user" uuid NOT NULL,
  "update_time" timestamp(6) NOT NULL,
  "update_user" uuid NOT NULL,
  "version" int4 NOT NULL,
  "sort_num" int4 NOT NULL DEFAULT 0,
  CONSTRAINT "tv_group_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "tv_group_name_u_idx" ON "tv_group" USING btree (
  "name" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "tv_group"."name" IS '名称';
COMMENT ON COLUMN "tv_group"."types" IS '绑定类型';
COMMENT ON COLUMN "tv_group"."create_time" IS '创建时间';
COMMENT ON COLUMN "tv_group"."create_user" IS '创建人';
COMMENT ON COLUMN "tv_group"."update_time" IS '修改时间';
COMMENT ON COLUMN "tv_group"."update_user" IS '修改人';
COMMENT ON COLUMN "tv_group"."version" IS '版本号';
COMMENT ON COLUMN "tv_group"."sort_num" IS '排序值';

CREATE TABLE "tv_type" (
  "id" uuid NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "sort_num" int4 NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "create_user" uuid NOT NULL,
  "update_time" timestamp(6) NOT NULL,
  "update_user" uuid NOT NULL,
  "version" int4 NOT NULL,
  "deleted" bool NOT NULL DEFAULT false,
  "delete_time" timestamp(6) NOT NULL DEFAULT '2000-01-01 00:00:00'::timestamp without time zone,
  "delete_user" uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
  "deleted_reason" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::text,
  CONSTRAINT "tv_type_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "tv_type_u_idx" ON "tv_type" USING btree (
  "name" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
) WHERE deleted = false;

COMMENT ON COLUMN "tv_type"."name" IS '名称';
COMMENT ON COLUMN "tv_type"."sort_num" IS '排序值';
COMMENT ON COLUMN "tv_type"."create_time" IS '创建时间';
COMMENT ON COLUMN "tv_type"."create_user" IS '创建人';
COMMENT ON COLUMN "tv_type"."update_time" IS '修改时间';
COMMENT ON COLUMN "tv_type"."update_user" IS '修改人';
COMMENT ON COLUMN "tv_type"."version" IS '版本号';
COMMENT ON COLUMN "tv_type"."deleted" IS '是否删除';
COMMENT ON COLUMN "tv_type"."delete_time" IS '删除时间';
COMMENT ON COLUMN "tv_type"."delete_user" IS '删除人';
COMMENT ON COLUMN "tv_type"."deleted_reason" IS '删除原因';

CREATE TABLE "tv_type_bind" (
  "id" uuid NOT NULL,
  "tv_type_id" uuid NOT NULL,
  "collect_type_id" uuid NOT NULL,
  "site_id" uuid NOT NULL,
  CONSTRAINT "tv_type_bind_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "tv_type_bind_site_idx" ON "tv_type_bind" USING btree (
  "site_id" "pg_catalog"."uuid_ops" ASC NULLS LAST
);

CREATE UNIQUE INDEX "tv_type_bind_u_idx" ON "tv_type_bind" USING btree (
  "tv_type_id" "pg_catalog"."uuid_ops" ASC NULLS LAST,
  "collect_type_id" "pg_catalog"."uuid_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "tv_type_bind"."tv_type_id" IS '类型ID';
COMMENT ON COLUMN "tv_type_bind"."collect_type_id" IS '采集站类型ID';
COMMENT ON COLUMN "tv_type_bind"."site_id" IS '采集站点ID';


CREATE TABLE "tv_vod" (
  "id" uuid NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "clear_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "vod_tag" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_class" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_actor" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_blurb" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_remarks" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_content" text COLLATE "pg_catalog"."default" NOT NULL,
  "episode_count" int4 NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "update_time" timestamp(6) NOT NULL,
  "collect_type" jsonb NOT NULL DEFAULT '[]'::jsonb,
  "collect_vod" jsonb NOT NULL,
  "show" bool NOT NULL DEFAULT true,
  "tv_type" jsonb NOT NULL DEFAULT '[]'::jsonb,
  CONSTRAINT "tv_vod_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "tv_vod_clear_name_u_idx" ON "tv_vod" USING btree (
  "clear_name" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

CREATE INDEX "tv_vod_collect_types_gin" ON "tv_vod" USING gin (
  "collect_type" "pg_catalog"."jsonb_ops"
);

CREATE INDEX "tv_vod_search_idx1" ON "tv_vod" USING gin (
  split_12(clear_name::text) COLLATE "pg_catalog"."default" "pg_catalog"."array_ops"
);

CREATE INDEX "tv_vod_search_idx2" ON "tv_vod" USING gin (
  "clear_name" COLLATE "pg_catalog"."default" "gin_trgm_ops"
);

CREATE INDEX "tv_vod_tv_types_gin" ON "tv_vod" USING gin (
  "tv_type" "pg_catalog"."jsonb_ops"
);

COMMENT ON COLUMN "tv_vod"."name" IS '名称';
COMMENT ON COLUMN "tv_vod"."clear_name" IS '清除空格的名称';
COMMENT ON COLUMN "tv_vod"."vod_tag" IS '标签';
COMMENT ON COLUMN "tv_vod"."vod_class" IS '分类';
COMMENT ON COLUMN "tv_vod"."vod_actor" IS '演员';
COMMENT ON COLUMN "tv_vod"."vod_blurb" IS '简介';
COMMENT ON COLUMN "tv_vod"."vod_remarks" IS '备注';
COMMENT ON COLUMN "tv_vod"."vod_content" IS '内容';
COMMENT ON COLUMN "tv_vod"."episode_count" IS '剧集数量';
COMMENT ON COLUMN "tv_vod"."create_time" IS '创建时间';
COMMENT ON COLUMN "tv_vod"."update_time" IS '修改时间';
COMMENT ON COLUMN "tv_vod"."collect_type" IS '关联采集类型';
COMMENT ON COLUMN "tv_vod"."collect_vod" IS '关联采集视频';
COMMENT ON COLUMN "tv_vod"."show" IS '是否显示';
COMMENT ON COLUMN "tv_vod"."tv_type" IS '关联视频类型';


CREATE TABLE "tv_vod_pic" (
  "id" uuid NOT NULL,
  "tv_vod_id" uuid NOT NULL,
  "pic" text COLLATE "pg_catalog"."default" NOT NULL,
  "status" bool NOT NULL,
  "site_id" uuid NOT NULL,
  "collect_vod_id" uuid NOT NULL,
  CONSTRAINT "tv_vod_pic_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "tv_vod_pic_idx" ON "tv_vod_pic" USING btree (
  "tv_vod_id" "pg_catalog"."uuid_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "tv_vod_pic"."tv_vod_id" IS '视频ID';
COMMENT ON COLUMN "tv_vod_pic"."pic" IS '封面';
COMMENT ON COLUMN "tv_vod_pic"."status" IS '是否有效';
COMMENT ON COLUMN "tv_vod_pic"."site_id" IS '站点ID';
COMMENT ON COLUMN "tv_vod_pic"."collect_vod_id" IS '采集视频ID';


CREATE TABLE "collect_site" (
  "id" uuid NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "main_page" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "req_url" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "full_status" int4 NOT NULL,
  "full_collect_time" timestamp(6) NOT NULL,
  "collect_status" int4 NOT NULL,
  "last_time" timestamp(6) NOT NULL,
  "status" bool NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "create_user" uuid NOT NULL,
  "update_time" timestamp(6) NOT NULL,
  "update_user" uuid NOT NULL,
  "version" int4 NOT NULL,
  "deleted" bool NOT NULL DEFAULT false,
  "delete_time" timestamp(6) NOT NULL DEFAULT '2000-01-01 00:00:00'::timestamp without time zone,
  "delete_user" uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
  "deleted_reason" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::text,
  "player" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::text,
  CONSTRAINT "collect_site_pkey" PRIMARY KEY ("id")
);

COMMENT ON COLUMN "collect_site"."name" IS '名称';
COMMENT ON COLUMN "collect_site"."main_page" IS '主页地址';
COMMENT ON COLUMN "collect_site"."req_url" IS '请求URL';
COMMENT ON COLUMN "collect_site"."full_status" IS '全量采集状态';
COMMENT ON COLUMN "collect_site"."full_collect_time" IS '全量采集时间';
COMMENT ON COLUMN "collect_site"."collect_status" IS '增量采集状态';
COMMENT ON COLUMN "collect_site"."last_time" IS '最后采集时间';
COMMENT ON COLUMN "collect_site"."status" IS '状态';
COMMENT ON COLUMN "collect_site"."create_time" IS '创建时间';
COMMENT ON COLUMN "collect_site"."create_user" IS '创建人';
COMMENT ON COLUMN "collect_site"."update_time" IS '修改时间';
COMMENT ON COLUMN "collect_site"."update_user" IS '修改人';
COMMENT ON COLUMN "collect_site"."version" IS '版本号';
COMMENT ON COLUMN "collect_site"."deleted" IS '是否删除';
COMMENT ON COLUMN "collect_site"."delete_time" IS '删除时间';
COMMENT ON COLUMN "collect_site"."delete_user" IS '删除人';
COMMENT ON COLUMN "collect_site"."deleted_reason" IS '删除原因';
COMMENT ON COLUMN "collect_site"."player" IS '播放器';

CREATE TABLE "collect_type" (
  "id" uuid NOT NULL,
  "site_id" uuid NOT NULL,
  "type_id" int4 NOT NULL,
  "type_name" varchar COLLATE "pg_catalog"."default" NOT NULL,
  "vod_count" int4 NOT NULL,
  "show" bool NOT NULL DEFAULT true,
  CONSTRAINT "collect_type_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "collect_type_u_idx" ON "collect_type" USING btree (
  "site_id" "pg_catalog"."uuid_ops" ASC NULLS LAST,
  "type_id" "pg_catalog"."int4_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "collect_type"."site_id" IS '采集站ID';
COMMENT ON COLUMN "collect_type"."type_id" IS '类型ID';
COMMENT ON COLUMN "collect_type"."type_name" IS '类型名称';
COMMENT ON COLUMN "collect_type"."vod_count" IS '视频数量';
COMMENT ON COLUMN "collect_type"."show" IS '是否显示';

CREATE TABLE "collect_vod" (
  "id" uuid NOT NULL,
  "site_id" uuid NOT NULL,
  "collect_type_id" uuid NOT NULL,
  "vod_id" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "vod_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "vod_pic" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_tag" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_class" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_actor" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_blurb" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_remarks" text COLLATE "pg_catalog"."default" NOT NULL,
  "vod_content" text COLLATE "pg_catalog"."default" NOT NULL,
  "create_time" timestamp(6) NOT NULL,
  "episode_count" int4 NOT NULL DEFAULT 0,
  CONSTRAINT "collect_vod_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "collect_vod_u_idx" ON "collect_vod" USING btree (
  "site_id" "pg_catalog"."uuid_ops" ASC NULLS LAST,
  "vod_id" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "collect_vod"."site_id" IS '站点ID';
COMMENT ON COLUMN "collect_vod"."collect_type_id" IS '分类ID';
COMMENT ON COLUMN "collect_vod"."vod_id" IS '原始视频ID';
COMMENT ON COLUMN "collect_vod"."vod_name" IS '视频名称';
COMMENT ON COLUMN "collect_vod"."vod_pic" IS '封面';
COMMENT ON COLUMN "collect_vod"."vod_tag" IS '标签';
COMMENT ON COLUMN "collect_vod"."vod_class" IS '分类';
COMMENT ON COLUMN "collect_vod"."vod_actor" IS '演员';
COMMENT ON COLUMN "collect_vod"."vod_blurb" IS '简介';
COMMENT ON COLUMN "collect_vod"."vod_remarks" IS '备注';
COMMENT ON COLUMN "collect_vod"."vod_content" IS '内容';
COMMENT ON COLUMN "collect_vod"."create_time" IS '创建时间';
COMMENT ON COLUMN "collect_vod"."episode_count" IS '剧集数量';

CREATE TABLE "collect_vod_episode" (
  "id" uuid NOT NULL,
  "site_id" uuid NOT NULL,
  "collect_vod_id" uuid NOT NULL,
  "line" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "url" text COLLATE "pg_catalog"."default" NOT NULL,
  "sort_num" int4 NOT NULL,
  CONSTRAINT "collect_vod_episode_pkey" PRIMARY KEY ("id")
);

CREATE INDEX "collect_vod_episode_idx" ON "collect_vod_episode" USING btree (
  "collect_vod_id" "pg_catalog"."uuid_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "collect_vod_episode"."site_id" IS '采集站ID';
COMMENT ON COLUMN "collect_vod_episode"."collect_vod_id" IS '视频ID';
COMMENT ON COLUMN "collect_vod_episode"."line" IS '线路';
COMMENT ON COLUMN "collect_vod_episode"."name" IS '分集名称';
COMMENT ON COLUMN "collect_vod_episode"."url" IS '播放url';
COMMENT ON COLUMN "collect_vod_episode"."sort_num" IS '排序';




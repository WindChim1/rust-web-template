-- ----------------------------
-- 操作日志记录
-- ----------------------------
DROP TABLE IF EXISTS sys_oper_log;

--  创建表结构
CREATE TABLE sys_oper_log (
  oper_id           SERIAL        NOT NULL PRIMARY KEY,
  title             VARCHAR(50)      DEFAULT '',
  business_type     SMALLINT         DEFAULT 0,
  method            VARCHAR(200)     DEFAULT '',
  request_method    VARCHAR(10)      DEFAULT '',
  operator_type     SMALLINT         DEFAULT 0,
  oper_name         VARCHAR(50)      DEFAULT '',
  oper_url          VARCHAR(255)     DEFAULT '',
  oper_ip           VARCHAR(128)     DEFAULT '',
  oper_location     VARCHAR(255)     DEFAULT '',
  oper_param        VARCHAR(2000)    DEFAULT '',
  json_result       VARCHAR(2000)    DEFAULT '',
  status            SMALLINT         DEFAULT 0,
  error_msg         VARCHAR(2000)    DEFAULT '',
  oper_time         TIMESTAMPTZ,
  cost_time         BIGINT           DEFAULT 0
);

--  独立添加表注释
COMMENT ON TABLE sys_oper_log IS '操作日志记录';

--  独立添加字段注释
COMMENT ON COLUMN sys_oper_log.oper_id IS '日志主键';
COMMENT ON COLUMN sys_oper_log.title IS '模块标题';
COMMENT ON COLUMN sys_oper_log.business_type IS '业务类型（0其它 1新增 2修改 3删除）';
COMMENT ON COLUMN sys_oper_log.method IS '方法名称';
COMMENT ON COLUMN sys_oper_log.request_method IS '请求方式';
COMMENT ON COLUMN sys_oper_log.operator_type IS '操作类别（0其它 1后台用户 2手机端用户）';
COMMENT ON COLUMN sys_oper_log.oper_name IS '操作人员';
COMMENT ON COLUMN sys_oper_log.oper_url IS '请求URL';
COMMENT ON COLUMN sys_oper_log.oper_ip IS '主机地址';
COMMENT ON COLUMN sys_oper_log.oper_location IS '操作地点';
COMMENT ON COLUMN sys_oper_log.oper_param IS '请求参数';
COMMENT ON COLUMN sys_oper_log.json_result IS '返回参数';
COMMENT ON COLUMN sys_oper_log.status IS '操作状态（0正常 1异常）';
COMMENT ON COLUMN sys_oper_log.error_msg IS '错误消息';
COMMENT ON COLUMN sys_oper_log.oper_time IS '操作时间';
COMMENT ON COLUMN sys_oper_log.cost_time IS '消耗时间';


-- 创建索引
CREATE INDEX idx_sys_oper_log_bt ON sys_oper_log (business_type);
CREATE INDEX idx_sys_oper_log_s  ON sys_oper_log (status);
CREATE INDEX idx_sys_oper_log_ot ON sys_oper_log (oper_time);

-- 删除字典类型表（如果存在）
DROP TABLE IF EXISTS sys_dict_type;

-- 创建字典类型表（不含内联注释）
CREATE TABLE sys_dict_type (
    dict_id          SERIAL PRIMARY KEY,
    dict_name        VARCHAR(100) DEFAULT '',
    dict_type        VARCHAR(100) DEFAULT '',
    status           CHAR(1) DEFAULT '0',
    create_by        VARCHAR(64) DEFAULT '',
    create_time      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by        VARCHAR(64) DEFAULT '',
    update_time      TIMESTAMPTZ,
    remark           VARCHAR(500),
    CONSTRAINT uk_dict_type UNIQUE (dict_type)
);

-- 为表和字段添加注释
COMMENT ON TABLE sys_dict_type IS '字典类型表';
COMMENT ON COLUMN sys_dict_type.dict_id IS '字典主键';
COMMENT ON COLUMN sys_dict_type.dict_name IS '字典名称';
COMMENT ON COLUMN sys_dict_type.dict_type IS '字典类型';
COMMENT ON COLUMN sys_dict_type.status IS '状态（0正常 1停用）';
COMMENT ON COLUMN sys_dict_type.create_by IS '创建者';
COMMENT ON COLUMN sys_dict_type.create_time IS '创建时间';
COMMENT ON COLUMN sys_dict_type.update_by IS '更新者';
COMMENT ON COLUMN sys_dict_type.update_time IS '更新时间';
COMMENT ON COLUMN sys_dict_type.remark IS '备注';


-- ----------------------------
-- 字典数据表
-- ----------------------------
DROP TABLE IF EXISTS sys_dict_data;

-- 创建字典数据表（不含内联注释）
CREATE TABLE sys_dict_data (
    dict_code        SERIAL PRIMARY KEY,
    dict_sort        INT DEFAULT 0,
    dict_label       VARCHAR(100) DEFAULT '',
    dict_value       VARCHAR(100) DEFAULT '',
    dict_type        VARCHAR(100) DEFAULT '',
    is_default       CHAR(1) DEFAULT 'N',
    status           CHAR(1) DEFAULT '0',
    create_by        VARCHAR(64) DEFAULT '',
    create_time      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by        VARCHAR(64) DEFAULT '',
    update_time      TIMESTAMPTZ,
    remark           VARCHAR(500)
);

-- 为表和字段添加注释
COMMENT ON TABLE sys_dict_data IS '字典数据表';
COMMENT ON COLUMN sys_dict_data.dict_code IS '字典编码';
COMMENT ON COLUMN sys_dict_data.dict_sort IS '字典排序';
COMMENT ON COLUMN sys_dict_data.dict_label IS '字典标签';
COMMENT ON COLUMN sys_dict_data.dict_value IS '字典键值';
COMMENT ON COLUMN sys_dict_data.dict_type IS '字典类型';
COMMENT ON COLUMN sys_dict_data.is_default IS '是否默认（Y是 N否）';
COMMENT ON COLUMN sys_dict_data.status IS '状态（0正常 1停用）';
COMMENT ON COLUMN sys_dict_data.create_by IS '创建者';
COMMENT ON COLUMN sys_dict_data.create_time IS '创建时间';
COMMENT ON COLUMN sys_dict_data.update_by IS '更新者';
COMMENT ON COLUMN sys_dict_data.update_time IS '更新时间';
COMMENT ON COLUMN sys_dict_data.remark IS '备注';


-- 删除菜单权限表（如果存在）
DROP TABLE IF EXISTS sys_menu;
-- 创建菜单权限表
CREATE TABLE sys_menu (
    menu_id           SERIAL PRIMARY KEY,
    menu_name         VARCHAR(50) NOT NULL,
    parent_id         BIGINT DEFAULT 0,
    order_num         INT DEFAULT 0,
    path              VARCHAR(200) DEFAULT '',
    query             VARCHAR(255),
    route_name        VARCHAR(50) DEFAULT '',
    menu_type         CHAR(1) DEFAULT '',
    status            CHAR(1) DEFAULT '0',
    icon              VARCHAR(100) DEFAULT '#',
    create_by         VARCHAR(64) DEFAULT '',
    create_time       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by         VARCHAR(64) DEFAULT '',
    update_time       TIMESTAMPTZ,
    remark            VARCHAR(500) DEFAULT ''
);

-- 添加表和字段注释
COMMENT ON TABLE sys_menu IS '菜单权限表';
COMMENT ON COLUMN sys_menu.menu_id IS '菜单ID';
COMMENT ON COLUMN sys_menu.menu_name IS '菜单名称';
COMMENT ON COLUMN sys_menu.parent_id IS '父菜单ID';
COMMENT ON COLUMN sys_menu.order_num IS '显示顺序';
COMMENT ON COLUMN sys_menu.path IS '路由地址';
COMMENT ON COLUMN sys_menu.query IS '路由参数';
COMMENT ON COLUMN sys_menu.route_name IS '路由名称';
COMMENT ON COLUMN sys_menu.menu_type IS '菜单类型（M目录 C菜单 F按钮）';
COMMENT ON COLUMN sys_menu.status IS '菜单状态（0正常 1停用）';
COMMENT ON COLUMN sys_menu.icon IS '菜单图标';
COMMENT ON COLUMN sys_menu.create_by IS '创建者';
COMMENT ON COLUMN sys_menu.create_time IS '创建时间';
COMMENT ON COLUMN sys_menu.update_by IS '更新者';
COMMENT ON COLUMN sys_menu.update_time IS '更新时间';
COMMENT ON COLUMN sys_menu.remark IS '备注';

-- 删除角色信息表（如果存在）
DROP TABLE IF EXISTS sys_role;
-- 创建角色信息表
CREATE TABLE sys_role (
    role_id              SERIAL PRIMARY KEY,
    role_name            VARCHAR(30) NOT NULL,
    role_key             VARCHAR(100) NOT NULL,
    role_sort            INT NOT NULL,
    data_scope           CHAR(1) DEFAULT '1',
    status               CHAR(1) NOT NULL,
    del_flag             CHAR(1) DEFAULT '0',
    create_by            VARCHAR(64) DEFAULT '',
    create_time          TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by            VARCHAR(64) DEFAULT '',
    update_time          TIMESTAMPTZ,
    remark               VARCHAR(500)
);

-- 添加表和字段注释
COMMENT ON TABLE sys_role IS '角色信息表';
COMMENT ON COLUMN sys_role.role_id IS '角色ID';
COMMENT ON COLUMN sys_role.role_name IS '角色名称';
COMMENT ON COLUMN sys_role.role_key IS '角色权限字符串';
COMMENT ON COLUMN sys_role.role_sort IS '显示顺序';
COMMENT ON COLUMN sys_role.data_scope IS '数据范围（1：全部数据权限 2：自定义数据权限 3：本级别数据权限 4：本级别及以下数据权限）';
COMMENT ON COLUMN sys_role.status IS '角色状态（0正常 1停用）';
COMMENT ON COLUMN sys_role.del_flag IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN sys_role.create_by IS '创建者';
COMMENT ON COLUMN sys_role.create_time IS '创建时间';
COMMENT ON COLUMN sys_role.update_by IS '更新者';
COMMENT ON COLUMN sys_role.update_time IS '更新时间';
COMMENT ON COLUMN sys_role.remark IS '备注';

insert into sys_role values('1', '超级管理员',  'admin',  1, 1, 1, '0', 'admin', now(), '', null, '超级管理员');
insert into sys_role values('2', '普通角色',    'common', 2, 2, 1, '0', 'admin', now(), '', null, '普通角色');



-- ----------------------------
-- 用户信息表
-- ----------------------------
--  删除表（若存在，避免冲突）
DROP TABLE IF EXISTS sys_user CASCADE;
CREATE TABLE sys_user (
    user_id           SERIAL PRIMARY KEY,
    user_name         VARCHAR(30) NOT NULL,
    nick_name         VARCHAR(30) NOT NULL,
    user_type         VARCHAR(2) DEFAULT '00',
    email             VARCHAR(50) DEFAULT '',
    phone_number       VARCHAR(11) DEFAULT '',
    avatar            VARCHAR(100) DEFAULT '',
    password          VARCHAR(100) DEFAULT '',
    status            CHAR(1) DEFAULT '0',
    del_flag          CHAR(1) DEFAULT '0',
    login_ip          VARCHAR(128) DEFAULT '',
    login_date        TIMESTAMPTZ,
    pwd_update_date   TIMESTAMPTZ,
    create_by         VARCHAR(64) DEFAULT '',
    create_time       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_by         VARCHAR(64) DEFAULT '',
    update_time       TIMESTAMPTZ,
    remark            VARCHAR(500)
);

COMMENT ON TABLE sys_user IS '用户信息表';
COMMENT ON COLUMN sys_user.user_id IS '用户ID';
COMMENT ON COLUMN sys_user.user_name IS '用户账号';
COMMENT ON COLUMN sys_user.nick_name IS '用户昵称';
COMMENT ON COLUMN sys_user.user_type IS '用户类型（00系统用户、01普通用户、02临时用户）';
COMMENT ON COLUMN sys_user.email IS '用户邮箱';
COMMENT ON COLUMN sys_user.phone_number IS '手机号码';
COMMENT ON COLUMN sys_user.avatar IS '头像地址';
COMMENT ON COLUMN sys_user.password IS '密码（存储加密后的值）';
COMMENT ON COLUMN sys_user.status IS '账号状态（0正常 1停用）';
COMMENT ON COLUMN sys_user.del_flag IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN sys_user.login_ip IS '最后登录IP地址';
COMMENT ON COLUMN sys_user.login_date IS '最后登录时间';
COMMENT ON COLUMN sys_user.pwd_update_date IS '密码最后更新时间';
COMMENT ON COLUMN sys_user.create_by IS '创建者（用户名）';
COMMENT ON COLUMN sys_user.create_time IS '记录创建时间';
COMMENT ON COLUMN sys_user.update_by IS '更新者（用户名）';
COMMENT ON COLUMN sys_user.update_time IS '记录更新时间';
COMMENT ON COLUMN sys_user.remark IS '备注信息';


-- 删除用户和角色关联表（如果存在）
DROP TABLE IF EXISTS sys_user_role;
-- 创建用户和角色关联表
CREATE TABLE sys_user_role (
    user_id   BIGINT NOT NULL,
    role_id   BIGINT NOT NULL,
    -- 联合主键（用户ID+角色ID唯一确定一条关联记录）
    PRIMARY KEY (user_id, role_id)
);
-- 添加表和字段注释
COMMENT ON TABLE sys_user_role IS '用户和角色关联表';
COMMENT ON COLUMN sys_user_role.user_id IS '用户ID';
COMMENT ON COLUMN sys_user_role.role_id IS '角色ID';


-- 删除角色和菜单关联表（如果存在）
DROP TABLE IF EXISTS sys_role_menu;
-- 创建角色和菜单关联表
CREATE TABLE sys_role_menu (
    role_id   BIGINT NOT NULL,  -- 角色ID，关联sys_role表的role_id
    menu_id   BIGINT NOT NULL,  -- 菜单ID，关联sys_menu表的menu_id
    -- 联合主键：确保一个角色与一个菜单只能有一条关联记录
    PRIMARY KEY (role_id, menu_id)
);
-- 添加表和字段注释
COMMENT ON TABLE sys_role_menu IS '角色和菜单关联表';
COMMENT ON COLUMN sys_role_menu.role_id IS '角色ID';
COMMENT ON COLUMN sys_role_menu.menu_id IS '菜单ID';

-- 删除操作日志记录表（如果存在）
DROP TABLE IF EXISTS sys_oper_log;
-- 创建操作日志记录表
CREATE TABLE sys_oper_log (
    oper_id           SERIAL PRIMARY KEY,
    title             VARCHAR(50) DEFAULT '',
    business_type     INT DEFAULT 0,
    method            VARCHAR(200) DEFAULT '',
    request_method    VARCHAR(10) DEFAULT '',
    operator_type     INT DEFAULT 0,
    oper_name         VARCHAR(50) DEFAULT '',
    oper_url          VARCHAR(255) DEFAULT '',
    oper_ip           VARCHAR(128) DEFAULT '',
    oper_location     VARCHAR(255) DEFAULT '',
    oper_param        VARCHAR(2000) DEFAULT '',
    json_result       VARCHAR(2000) DEFAULT '',
    status            INT DEFAULT 0,
    error_msg         VARCHAR(2000) DEFAULT '',
    oper_time         TIMESTAMPTZ,
    cost_time         BIGINT DEFAULT 0
);

-- 添加索引（对应原表的 key 定义）
CREATE INDEX idx_sys_oper_log_bt ON sys_oper_log (business_type);
CREATE INDEX idx_sys_oper_log_s ON sys_oper_log (status);
CREATE INDEX idx_sys_oper_log_ot ON sys_oper_log (oper_time);

-- 添加表和字段注释
COMMENT ON TABLE sys_oper_log IS '操作日志记录';
COMMENT ON COLUMN sys_oper_log.oper_id IS '日志主键';
COMMENT ON COLUMN sys_oper_log.title IS '模块标题';
COMMENT ON COLUMN sys_oper_log.business_type IS '业务类型（0其它 1新增 2修改 3删除）';
COMMENT ON COLUMN sys_oper_log.method IS '方法名称';
COMMENT ON COLUMN sys_oper_log.request_method IS '请求方式';
COMMENT ON COLUMN sys_oper_log.operator_type IS '操作类别（0其它 1后台用户 2手机端用户）';
COMMENT ON COLUMN sys_oper_log.oper_name IS '操作人员';
COMMENT ON COLUMN sys_oper_log.oper_url IS '请求URL';
COMMENT ON COLUMN sys_oper_log.oper_ip IS '主机地址';
COMMENT ON COLUMN sys_oper_log.oper_location IS '操作地点';
COMMENT ON COLUMN sys_oper_log.oper_param IS '请求参数';
COMMENT ON COLUMN sys_oper_log.json_result IS '返回参数';
COMMENT ON COLUMN sys_oper_log.status IS '操作状态（0正常 1异常）';
COMMENT ON COLUMN sys_oper_log.error_msg IS '错误消息';
COMMENT ON COLUMN sys_oper_log.oper_time IS '操作时间';
COMMENT ON COLUMN sys_oper_log.cost_time IS '消耗时间（毫秒）';


DROP TABLE IF EXISTS sys_logininfor;
-- 创建系统访问记录表
CREATE TABLE sys_login_infor (
                                 info_id        SERIAL PRIMARY KEY,
                                 user_name      VARCHAR(50) DEFAULT '',
                                 ipaddr         VARCHAR(128) DEFAULT '',
                                 login_location VARCHAR(255) DEFAULT '',
                                 browser        VARCHAR(50) DEFAULT '',
                                 os             VARCHAR(50) DEFAULT '',
                                 status         CHAR(1) DEFAULT '0',
                                 msg            VARCHAR(255) DEFAULT '',
                                 login_time     TIMESTAMPTZ
);

-- 添加索引（对应原表的 key 定义）
CREATE INDEX idx_sys_login_infor_s ON sys_login_infor (status);
CREATE INDEX idx_sys_login_infor_lt ON sys_login_infor (login_time);

-- 添加表和字段注释
COMMENT ON TABLE sys_login_infor IS '系统访问记录';
COMMENT ON COLUMN sys_login_infor.info_id IS '访问ID';
COMMENT ON COLUMN sys_login_infor.user_name IS '用户账号';
COMMENT ON COLUMN sys_login_infor.ipaddr IS '登录IP地址';
COMMENT ON COLUMN sys_login_infor.login_location IS '登录地点';
COMMENT ON COLUMN sys_login_infor.browser IS '浏览器类型';
COMMENT ON COLUMN sys_login_infor.os IS '操作系统';
COMMENT ON COLUMN sys_login_infor.status IS '登录状态（0成功 1失败）';
COMMENT ON COLUMN sys_login_infor.msg IS '提示消息（如失败原因）';
COMMENT ON COLUMN sys_login_infor.login_time IS '访问时间';




-- 删除上传文件记录表（如果存在）
DROP TABLE IF EXISTS sys_upload_files;

-- 创建上传文件记录表
CREATE TABLE sys_upload_files (
    file_id           SERIAL PRIMARY KEY,
    original_name     VARCHAR(255) NOT NULL,
    stored_path       VARCHAR(500) NOT NULL,
    file_url          VARCHAR(500) NOT NULL,
    file_size         BIGINT DEFAULT 0,
    file_status       VARCHAR(20) NOT NULL DEFAULT 'pending',
    uploader_name     VARCHAR(64) DEFAULT '',
    upload_time       TIMESTAMPTZ(3) DEFAULT CURRENT_TIMESTAMP(3),
    remark            VARCHAR(500)
);
-- 创建唯一索引（对应原表的唯一索引）
CREATE UNIQUE INDEX idx_file_url ON sys_upload_files (file_url) ;


-- 添加表和字段注释
COMMENT ON TABLE sys_upload_files IS '上传文件记录表';
COMMENT ON COLUMN sys_upload_files.file_id IS '文件ID (主键)';
COMMENT ON COLUMN sys_upload_files.original_name IS '原始文件名';
COMMENT ON COLUMN sys_upload_files.stored_path IS '文件存储相对路径 (格式: YYYYMM/username/uuid.ext)';
COMMENT ON COLUMN sys_upload_files.file_url IS '可供前端访问的URL (格式: /uploads/YYYYMM/username/uuid.ext)';
COMMENT ON COLUMN sys_upload_files.file_size IS '文件大小 (字节)';
COMMENT ON COLUMN sys_upload_files.file_status IS '文件状态 (pending, active, deprecated)';
COMMENT ON COLUMN sys_upload_files.uploader_name IS '上传者用户名';
COMMENT ON COLUMN sys_upload_files.upload_time IS '上传时间';
COMMENT ON COLUMN sys_upload_files.remark IS '备注';

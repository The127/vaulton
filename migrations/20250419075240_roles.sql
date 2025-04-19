create table roles (
    id uuid primary key default gen_random_uuid(),
    name text not null unique,
    description text,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);

create table user_roles (
    user_id uuid references users(id) on delete cascade,
    role_id uuid references roles(id) on delete cascade,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp,
    primary key (user_id, role_id)
);

-- indexes
create index idx_roles_name on roles (name);
create index idx_user_roles_user_id on user_roles (user_id);
create index idx_user_roles_role_id on user_roles (role_id);

-- triggers
create trigger set_roles_timestamps
    before insert on roles
    for each row
execute function set_created_at_column();

create trigger update_roles_updated_at
    before update on roles
    for each row
execute function update_updated_at_column();

create trigger set_user_roles_timestamps
    before insert on user_roles
    for each row
execute function set_created_at_column();

create trigger update_user_roles_updated_at
    before update on user_roles
    for each row
execute function update_updated_at_column();

-- default roles
insert into roles (name, description)
values ('admin', 'Full system access'),
       ('user', 'Standard user access');
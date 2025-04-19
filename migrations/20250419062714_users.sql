create table users (
    id uuid primary key default gen_random_uuid(),
    username text not null unique,
    password_hash text not null,
    email text not null unique,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);

-- indexes
create index idx_users_username on users (username);
create index idx_users_email on users (email);

-- triggers
create trigger set_users_timestamps
    before insert on users
    for each row
execute function set_created_at_column();

create trigger update_users_updated_at
    before update on users
    for each row
execute function update_updated_at_column();
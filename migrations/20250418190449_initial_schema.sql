-- migrations/20250418190449_initial_schema.sql

-- enable uuid generation
create extension if not exists "uuid-ossp";

-- timestamp management functions
create or replace function update_updated_at_column()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language 'plpgsql';

create or replace function set_created_at_column()
returns trigger as $$
begin
    new.created_at = now();
    new.updated_at = now();
    return new;
end;
$$ language 'plpgsql';

-- clients table to store oidc client applications
create table clients (
    id uuid primary key default uuid_generate_v4(),
    client_id varchar(255) unique not null,     -- public client identifier
    client_secret_hash bytea,                   -- bcrypt hash stored as raw bytes
    redirect_uris text[],                       -- array of allowed redirect uris
    scopes text[],                             -- array of allowed scopes
    created_at timestamptz not null,            -- removed default, will be set by trigger
    updated_at timestamptz not null             -- removed default, will be set by trigger
);

-- Trigger for new records
create trigger set_clients_timestamps
    before insert on clients
    for each row
    execute function set_created_at_column();

-- Trigger for updated records
create trigger update_clients_updated_at
    before update on clients
    for each row
    execute function update_updated_at_column();

create index idx_clients_client_id on clients(client_id);
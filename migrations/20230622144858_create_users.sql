-- Add migration script here
create table if not exists users (
  id uuid primary key,
  name varchar(50) not null,
  email varchar(50) not null,
  created_at timestamp with time zone default now(),
  updated_at timestamp with time zone default now()
);

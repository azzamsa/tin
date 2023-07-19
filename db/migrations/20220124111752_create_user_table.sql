-- Add migration script here

/* `user` is reserved keyword in postgres.
   using "user" with double quotes requires string escaping all the time.
   `user_` is the current best option.
*/
create table if not exists user_ (
   id UUID primary key,

   created_at timestamp with time zone not null,
   updated_at timestamp with time zone not null,

   name text not null unique,
   full_name text
);

create table property_status_history (
    id serial primary key,
    property_id uuid not null references properties (id),
    status_id integer not null references property_statuses (id),
    changed_at timestamp default current_timestamp not null
);
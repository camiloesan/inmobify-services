create table states (
    id serial primary key,
    "name" varchar(24) not null unique
);

insert into states (name) values
    ('Aguascalientes'),
    ('Baja California'),
    ('Baja California Sur'),
    ('Campeche'),
    ('Chiapas'),
    ('Chihuahua'),
    ('Ciudad de México'),
    ('Coahuila'),
    ('Colima'),
    ('Durango'),
    ('Estado de México'),
    ('Guanajuato'),
    ('Guerrero'),
    ('Hidalgo'),
    ('Jalisco'),
    ('Michoacán'),
    ('Morelos'),
    ('Nayarit'),
    ('Nuevo León'),
    ('Oaxaca'),
    ('Puebla'),
    ('Querétaro'),
    ('Quintana Roo'),
    ('San Luis Potosí'),
    ('Sinaloa'),
    ('Sonora'),
    ('Tabasco'),
    ('Tamaulipas'),
    ('Tlaxcala'),
    ('Veracruz'),
    ('Yucatán'),
    ('Zacatecas');
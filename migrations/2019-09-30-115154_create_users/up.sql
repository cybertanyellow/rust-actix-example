CREATE TABLE users (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(122) NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  updated_by VARCHAR(36) NOT NULL
);

insert into users (id, first_name, last_name, email, password, created_by, updated_by) values 
('00000000-0000-0000-0000-000000000000', 'admin', 'user', 'admin@admin.com', 'f9e90c1b7955972ed0b3b70aa4dd4d9989a37a0021cebed929988ab9d8efb50c', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
('1802d2f8-1a18-43c1-9c58-1c3f7100c842', 'test', 'user', 'test@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000');

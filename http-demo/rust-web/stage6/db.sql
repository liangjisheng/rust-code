drop table if exists course;

create table course (
    id bigint primary key AUTO_INCREMENT,
    teacher_id int not null,
    name varchar(140) not null,
    time TIMESTAMP default now()
);

insert into course (teacher_id, name, time) values(1, 'First course', '2022-01-17 05:40:00');
insert into course (teacher_id, name, time) values(1, 'Second course', '2022-01-18 05:45:00');
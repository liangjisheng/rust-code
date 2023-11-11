drop table if exists course;

create table `course` (
    `id` bigint NOT NULL AUTO_INCREMENT,
    `teacher_id` int(0) NOT NULL,
    `name` varchar(140) NOT NULL,
    `time` timestamp(0) NULL DEFAULT CURRENT_TIMESTAMP(0),
    `description` varchar(2000) DEFAULT NULL,
    `format` varchar(30) DEFAULT NULL,
    `structure` varchar(200) DEFAULT NULL,
    `duration` varchar(30) DEFAULT NULL,
    `price` int(0) DEFAULT NULL,
    `language` varchar(30) DEFAULT NULL,
    `level` varchar(30) DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Records of course
-- ----------------------------
INSERT INTO `course` VALUES (1, 1, 'First course', '2022-01-17 05:40:00', NULL, NULL, NULL, NULL, NULL, NULL, NULL);
INSERT INTO `course` VALUES (2, 1, 'Second course', '2022-01-18 05:45:00', NULL, NULL, NULL, NULL, NULL, NULL, NULL);
INSERT INTO `course` VALUES (4, 1, 'Test course', '2023-03-01 21:14:52', 'This is a course', NULL, NULL, NULL, NULL, 'English', 'Beginner');

DROP TABLE IF EXISTS `teacher`;

CREATE TABLE `teacher` (
    `id` bigint NOT NULL AUTO_INCREMENT,
    `name` varchar(255) NOT NULL,
    `picture_url` varchar(255) NOT NULL,
    `profile` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ----------------------------
-- Records of teacher
-- ----------------------------
INSERT INTO `teacher` VALUES (1, 'alice', 'www.baidu.com', 'test');
INSERT INTO `teacher` VALUES (2, 'alice', 'www.baidu.com', 'test');
INSERT INTO `teacher` VALUES (3, 'alice', 'www.baidu.com', 'test');
INSERT INTO `teacher` VALUES (4, 'alice', 'www.baidu.com', 'test');

SET FOREIGN_KEY_CHECKS = 1;

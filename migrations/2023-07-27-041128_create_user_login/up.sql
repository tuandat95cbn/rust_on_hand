-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE party (
  party_id                    UUID NOT NULL default uuid_generate_v1(),
  party_type_id               VARCHAR(60),
  external_id                 VARCHAR(60),
  description                 TEXT,
  status_id                   VARCHAR(60),
  created_date                TIMESTAMP   NULL,
  created_by_user_login       VARCHAR(255),
  last_modified_date          TIMESTAMP   NULL,
  last_modified_by_user_login VARCHAR(255),
  is_unread                   BOOLEAN,
  updated_at          TIMESTAMP   ,
  created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  party_code                  VARCHAR(255),
  CONSTRAINT pk_party PRIMARY KEY (party_id)
);
CREATE TABLE person (
  party_id                    UUID              NOT NULL,
  first_name                  VARCHAR(100),
  middle_name                 VARCHAR(100),
  last_name                   VARCHAR(100),
  gender                      CHARACTER(1),
  birth_date                  DATE,

  updated_at          TIMESTAMP   ,
  created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT pk_person PRIMARY KEY (party_id),
  CONSTRAINT person_party FOREIGN KEY (party_id) REFERENCES party (party_id)
);

CREATE TABLE user_login (
  user_login_id            VARCHAR(255)  NOT NULL,
  current_password         VARCHAR(60),
  otp_secret               VARCHAR(60),
  client_token             VARCHAR(512),
  password_hint            TEXT,
  is_system                BOOLEAN,
  enabled                  BOOLEAN,
  has_logged_out           BOOLEAN,
  require_password_change  BOOLEAN,
  disabled_date_time       TIMESTAMP     NULL,
  successive_failed_logins INTEGER,
  updated_at          TIMESTAMP   ,
  created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  otp_resend_number        INT DEFAULT 0 NULL,
  party_id                 UUID,
  CONSTRAINT pk_user_login PRIMARY KEY (user_login_id),
  CONSTRAINT user_party FOREIGN KEY (party_id) REFERENCES party (party_id)
);
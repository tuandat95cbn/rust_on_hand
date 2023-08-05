-- Your SQL goes here

CREATE TABLE status(
  status_id VARCHAR(100)             NOT NULL,
  status_code                  VARCHAR(100),
  description                 VARCHAR(100),
  updated_at          TIMESTAMP   ,
  created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT pk_status PRIMARY KEY (status_id)
);
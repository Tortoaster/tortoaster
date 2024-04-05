CREATE TABLE projects
(
    id            VARCHAR(128) PRIMARY KEY,
    name          VARCHAR(32)              NOT NULL,
    description   TEXT                     NOT NULL,
    thumbnail_url VARCHAR                  NOT NULL,
    project_url   VARCHAR,
    date_posted   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE comments
(
    id          SERIAL PRIMARY KEY,
    project_id  VARCHAR(128) REFERENCES projects ON DELETE CASCADE NOT NULL,
    name        VARCHAR(32)                                        NOT NULL,
    email       VARCHAR(64)                                        NOT NULL,
    message     VARCHAR(256)                                       NOT NULL,
    date_posted TIMESTAMP WITH TIME ZONE                           NOT NULL DEFAULT now()
);

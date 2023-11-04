create table
    metric_categories (
        id varchar(32) PRIMARY KEY,
        name text NOT NULL,
        description text NOT NULL DEFAULT ''::text
    );

create table
    metric_types (
        id varchar(32) PRIMARY KEY,
        name text NOT NULL,
        description text NOT NULL DEFAULT ''::text,
        metric_category_id varchar(32) REFERENCES metric_categories (id) NOT NULL
    );

create table
    metric_sensors (
        id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID (),
        metric_type_id varchar(32) REFERENCES metric_types (id) NOT NULL,
        metric_category_id varchar(32) REFERENCES metric_categories (id) NOT NULL,
        agent_id UUID REFERENCES agents (id) NOT NULL
    );

create table
    metric_readings (
        metric_sensor_id UUID REFERENCES metric_sensors (id) NOT NULL,
        time timestamp default CURRENT_TIMESTAMP NOT NULL,
        value decimal default 0 NOT NULL,
        PRIMARY KEY (metric_sensor_id, time)
    );
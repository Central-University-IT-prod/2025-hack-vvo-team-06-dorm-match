CREATE TABLE rooms (
    id UUID PRIMARY KEY,
    number VARCHAR NOT NULL,
    description TEXT NOT NULL,
    photo_url VARCHAR,
    capacity INTEGER NOT NULL,
    current_occupants INTEGER NOT NULL,
    faculty_restriction VARCHAR,
    course_restriction INTEGER,
    sex_restriction VARCHAR NOT NULL,
    status VARCHAR NOT NULL
);

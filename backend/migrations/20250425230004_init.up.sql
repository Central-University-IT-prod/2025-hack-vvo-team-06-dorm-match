-- Создание ENUM типов для users и student_profiles
CREATE TYPE user_role AS ENUM ('student', 'admin');
CREATE TYPE user_status AS ENUM ('pending', 'verified', 'rejected');
CREATE TYPE wake_type AS ENUM ('early_bird', 'night_owl', 'flexible');
CREATE TYPE mbti_type AS ENUM (
    'intj', 'intp', 'entj', 'entp',
    'infj', 'infp', 'enfj', 'enfp',
    'istj', 'isfj', 'estj', 'esfj',
    'istp', 'isfp', 'estp', 'esfp'
);
CREATE TYPE user_sex AS ENUM ('male', 'female');

-- Создание таблицы users
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL,
    status user_status NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Создание таблицы student_profiles
CREATE TABLE student_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    faculty VARCHAR(100) NOT NULL,
    course INTEGER NOT NULL,
    gender user_sex NOT NULL,
    age INTEGER NOT NULL,
    wake_hours wake_type NOT NULL,
    hobbies JSONB NOT NULL,
    mbti mbti_type,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Создание таблицы applications
CREATE TABLE applications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    room_id UUID NOT NULL,
    status VARCHAR NOT NULL,
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

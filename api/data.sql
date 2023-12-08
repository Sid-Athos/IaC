create schema coding_games;

SET search_path TO database_n.coding_games;

create sequence invite_id
    maxvalue 999999999999
    cache 10;

alter sequence invite_id owner to admin_sid;

create sequence invite_id_auto
    maxvalue 99999
    cache 10;

alter sequence invite_id_auto owner to admin_sid;

create sequence invite_id_seq
    maxvalue 99999
    cache 10;

alter sequence invite_id_seq owner to admin_sid;

create sequence invite_id_seq1
    maxvalue 99999
    cache 10;

alter sequence invite_id_seq1 owner to admin_sid;

create sequence invite_id_seq2
    maxvalue 99999
    cache 10;

alter sequence invite_id_seq2 owner to admin_sid;

create sequence ws_session_id_seq
    maxvalue 10000000
    cache 1000;

alter sequence ws_session_id_seq owner to admin_sid;

create sequence game_history_id_seq
    maxvalue 1000000
    cache 100;

alter sequence game_history_id_seq owner to admin_sid;

create sequence game_members_history_id_seq
    maxvalue 10000000
    cache 100;

alter sequence game_members_history_id_seq owner to admin_sid;

create sequence game_move_history_id_seq
    maxvalue 1000000
    cache 100;

alter sequence game_move_history_id_seq owner to admin_sid;

create table if not exists "user"
(
    email      varchar(256)          not null
        constraint uq_email
            unique,
    password   varchar(256)          not null,
    pseudo     varchar(256)          not null
        constraint uq_pseudo
            unique,
    id         integer generated always as identity
        constraint id
            primary key,
    admin      boolean default false not null,
    experience integer default 0     not null,
    level      integer default 1     not null
);

alter table "user"
    owner to admin_sid;

create table if not exists session
(
    id      integer generated always as identity
        constraint session_pk
            primary key,
    token   varchar(256) not null
        constraint uq_token
            unique,
    user_id integer      not null
        constraint session_user__fk
            references "user"
);

alter table session
    owner to admin_sid;

create table if not exists game
(
    id          integer generated always as identity
        constraint game_pk
            primary key,
    name        varchar(256)                                not null,
    min_players integer      default 0                      not null,
    max_players integer      default 0                      not null,
    description text,
    language    varchar(255) default '/'::character varying not null,
    code        text,
    user_id     integer                                     not null
        constraint game_user_id_fk
            references "user",
    tag         varchar(256)                                not null
        unique,
    constraint game_pk2
        unique (name, user_id, language)
);

alter table game
    owner to admin_sid;

create table if not exists invite
(
    id           integer generated always as identity (maxvalue 99999 cache 10)
        constraint invite_pk
            primary key,
    lobby_id     integer not null,
    from_user_id integer not null,
    to_user_id   integer not null
);

alter table invite
    owner to admin_sid;

alter sequence invite_id_seq1 owned by invite.id;

alter sequence invite_id_seq2 owned by invite.id;

create table if not exists friend_list
(
    id           serial
        constraint friend_list_pk
            primary key,
    applicant_id integer               not null
        constraint friend_list_user_id_fk
            references "user",
    recipient_id integer               not null
        constraint friend_list_user_id_fk_2
            references "user",
    accepted     boolean default false not null,
    constraint unique_request
        unique (recipient_id, applicant_id),
    constraint unique_request_2
        unique (applicant_id, recipient_id),
    constraint check_name
        check (applicant_id <> recipient_id)
);

alter table friend_list
    owner to admin_sid;

create unique index if not exists friend_list_id_uindex
    on friend_list (id);

create table if not exists ranking
(
    id       serial,
    user_id  integer              not null
        constraint ranking_user_id_fk
            references "user",
    game_id  integer              not null
        constraint ranking_game_id_fk
            references game,
    rank     integer default 1000 not null,
    nb_games integer default 0    not null,
    constraint ranking_pk
        primary key (user_id, game_id)
);

alter table ranking
    owner to admin_sid;

create table if not exists ws_session
(
    id       integer generated always as identity (maxvalue 10000000 cache 1000)
        constraint ws_session_pk
            primary key,
    lobby_id integer not null,
    user_id  integer not null
);

alter table ws_session
    owner to admin_sid;

alter sequence ws_session_id_seq owned by ws_session.id;

create table if not exists game_history
(
    id         integer generated always as identity (maxvalue 1000000 cache 100)
        constraint game_history_pk
            primary key,
    date_time  timestamp default now() not null,
    nb_players integer                 not null,
    game_id    integer                 not null
        constraint game_history_game_id_fk
            references game
);

alter table game_history
    owner to admin_sid;

alter sequence game_history_id_seq owned by game_history.id;

create table if not exists lobby
(
    id                   integer generated always as identity
        constraint lobby_pk
            primary key,
    code                 varchar(256)          not null,
    game_id              integer               not null
        constraint lobby_game_null_fk
            references game,
    private              boolean default false not null,
    is_launched          boolean default false not null,
    game_history_id      integer
        constraint lobby_game_history_id_fk
            references game_history,
    from_move_history_id integer
);

alter table lobby
    owner to admin_sid;

create table if not exists lobby_member
(
    id       integer generated always as identity
        constraint lobby_member_pk
            primary key,
    lobby_id integer               not null
        constraint lobby_member_lobby_null_fk
            references lobby,
    user_id  integer               not null
        constraint user_id_unique_pk
            unique
        constraint lobby_member_user_null_fk
            references "user",
    is_host  boolean default false not null,
    player   integer default 0     not null
);

alter table lobby_member
    owner to admin_sid;

create table if not exists game_members_history
(
    id              integer generated always as identity (maxvalue 10000000 cache 100)
        constraint game_members_history_pk
            primary key,
    user_id         integer not null
        constraint game_members_history_user_id_fk
            references "user",
    game_history_id integer not null
        constraint game_members_history_game_history_id_fk
            references game_history,
    player          integer not null
);

alter table game_members_history
    owner to admin_sid;

alter sequence game_members_history_id_seq owned by game_members_history.id;

create table if not exists game_move_history
(
    id              integer generated always as identity (maxvalue 1000000 cache 100)
        constraint game_move_history_pk
            primary key,
    player          integer not null,
    game_state      text    not null,
    action          text    not null,
    action_number   integer not null,
    game_history_id integer not null
        constraint game_move_history_game_history_id_fk
            references game_history
);

alter table game_move_history
    owner to admin_sid;

alter sequence game_move_history_id_seq owned by game_move_history.id;


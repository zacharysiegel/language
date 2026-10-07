create table course (
    title text not null
)
;

create table level (
    level_code text not null constraint level_pk primary key,
    position integer not null constraint level_position_unique unique,
    title text not null
)
;

create table unit (
    unit_id text not null constraint unit_pk primary key,
    level_code text not null constraint unit_level_fk references level (level_code),
    ordinal integer not null constraint unit_ordinal_unique unique,
    title text not null
)
;

create table unit_objective (
    unit_id text not null constraint unit_objective_unit_fk references unit (unit_id),
    position integer not null,
    objective text not null,
    constraint unit_objective_pk primary key (unit_id, position)
)
;

create table vocabulary_item (
    unit_id text not null constraint vocabulary_item_unit_fk references unit (unit_id),
    position integer not null,
    pt text not null,
    en text not null,
    literal text,
    part_of_speech text,
    note text,
    clip_key text constraint vocabulary_item_clip_fk references clip (clip_key),
    example_pt text not null,
    example_en text not null,
    example_literal text,
    example_clip_key text constraint vocabulary_item_example_clip_fk references clip (clip_key),
    constraint vocabulary_item_pk primary key (unit_id, position)
)
;

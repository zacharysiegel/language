create table culture_note (
    title text not null,
    body text not null
)
;

create table dialogue (
    position integer not null constraint dialogue_pk primary key,
    dialogue_id text not null,
    title text not null,
    context text not null
)
;

create table dialogue_line (
    dialogue_position integer not null constraint dialogue_line_dialogue_fk references dialogue (position),
    position integer not null,
    speaker text not null,
    speaker_gender text not null,
    pt text not null,
    en text not null,
    literal text,
    clip_key text constraint dialogue_line_clip_fk references clip (clip_key),
    constraint dialogue_line_pk primary key (dialogue_position, position)
)
;

create table grammar_note (
    position integer not null constraint grammar_note_pk primary key,
    title text not null,
    explanation text not null
)
;

create table grammar_table_header (
    note_position integer not null constraint grammar_table_header_note_fk references grammar_note (position),
    position integer not null,
    header_text text not null,
    constraint grammar_table_header_pk primary key (note_position, position)
)
;

create table grammar_table_cell (
    note_position integer not null constraint grammar_table_cell_note_fk references grammar_note (position),
    row_position integer not null,
    column_position integer not null,
    cell_text text not null,
    constraint grammar_table_cell_pk primary key (note_position, row_position, column_position)
)
;

create table grammar_example (
    note_position integer not null constraint grammar_example_note_fk references grammar_note (position),
    position integer not null,
    pt text not null,
    en text not null,
    literal text,
    clip_key text constraint grammar_example_clip_fk references clip (clip_key),
    constraint grammar_example_pk primary key (note_position, position)
)
;

create table reading (
    position integer not null constraint reading_pk primary key,
    reading_id text not null,
    title text not null
)
;

create table reading_paragraph (
    reading_position integer not null constraint reading_paragraph_reading_fk references reading (position),
    position integer not null,
    pt text not null,
    en text not null,
    literal text,
    clip_key text constraint reading_paragraph_clip_fk references clip (clip_key),
    constraint reading_paragraph_pk primary key (reading_position, position)
)
;

create table exercise (
    exercise_id integer not null constraint exercise_pk primary key,
    reading_position integer constraint exercise_reading_fk references reading (position),
    position integer not null,
    exercise_kind text not null,
    prompt text,
    correct_choice_index integer,
    hint_en text,
    explanation text,
    clip_key text constraint exercise_clip_fk references clip (clip_key)
)
;

create table exercise_choice (
    exercise_id integer not null constraint exercise_choice_exercise_fk references exercise (exercise_id),
    position integer not null,
    choice_text text not null,
    constraint exercise_choice_pk primary key (exercise_id, position)
)
;

create table exercise_answer (
    exercise_id integer not null constraint exercise_answer_exercise_fk references exercise (exercise_id),
    position integer not null,
    answer_text text not null,
    constraint exercise_answer_pk primary key (exercise_id, position)
)
;

create table exercise_word (
    exercise_id integer not null constraint exercise_word_exercise_fk references exercise (exercise_id),
    position integer not null,
    word_text text not null,
    is_distractor integer not null,
    constraint exercise_word_pk primary key (exercise_id, position)
)
;

create table exercise_pair (
    exercise_id integer not null constraint exercise_pair_exercise_fk references exercise (exercise_id),
    position integer not null,
    pt text not null,
    en text not null,
    literal text,
    clip_key text constraint exercise_pair_clip_fk references clip (clip_key),
    constraint exercise_pair_pk primary key (exercise_id, position)
)
;

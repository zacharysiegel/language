create table clip (
    clip_key text not null constraint clip_pk primary key,
    persona text not null,
    spoken_text text not null
)
;

create table clip_audio (
    clip_key text not null constraint clip_audio_clip_fk references clip (clip_key),
    engine_code text not null,
    audio_sha256 text not null,
    audio_size_bytes integer not null,
    constraint clip_audio_pk primary key (clip_key, engine_code)
)
;

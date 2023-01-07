INSERT INTO instrument (name) VALUES ('bass');
INSERT INTO instrument (name) VALUES ('drum');
INSERT INTO instrument (name) VALUES ('electricguitar');
INSERT INTO instrument (name) VALUES ('acousticguitar');
INSERT INTO instrument (name) VALUES ('singing');
INSERT INTO instrument (name) VALUES ('piano');
INSERT INTO instrument (name) VALUES ('violin');

INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'bass'), id, 'bass' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'drum'), id, 'drum' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'electricguitar'), id, 'electric guitar' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'acousticguitar'), id, 'acoustic guitar' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'singing'), id, 'singing' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'piano'), id, 'piano' FROM language WHERE name = 'English';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'violin'), id, 'violin' FROM language WHERE name = 'English';

INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'bass'), id, 'guitare basse' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'drum'), id, 'batterie' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'electricguitar'), id, 'guitare électrique' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'acousticguitar'), id, 'guitare accoustique' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'singing'), id, 'chant' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'piano'), id, 'piano' FROM language WHERE name = 'French';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'violin'), id, 'violon' FROM language WHERE name = 'French';

INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'bass'), id, 'バス' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'drum'), id, 'ドラム' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'electricguitar'), id, 'エレクトリックギター' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'acousticguitar'), id, 'アコースティックギター' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'singing'), id, 'ボーカル' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'piano'), id, 'ピアノ' FROM language WHERE name = 'Japanese';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'violin'), id, 'バイオリン' FROM language WHERE name = 'Japanese';

INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'bass'), id, 'Bassgitarre' FROM language WHERE name = 'German';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'drum'), id, 'Trommel' FROM language WHERE name = 'German';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'electricguitar'), id, 'E-Gitarre' FROM language WHERE name = 'German';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'acousticguitar'), id, 'Gitarre' FROM language WHERE name = 'German';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'singing'), id, 'Gesang' FROM language WHERE name = 'German';
INSERT INTO instrument_local (instrument_id, language_id, name) SELECT (SELECT id FROM instrument WHERE name = 'piano'), id, 'Klavier' FROM language WHERE name = 'German';

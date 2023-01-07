INSERT INTO language (name) VALUES
    ('English'),
    ('French'),
    ('Japanese'),
    ('German');

INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'English'), id, 'English' FROM language WHERE name = 'English';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'French'), id, 'French' FROM language WHERE name = 'English';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'Japanese'), id, 'Japanese' FROM language WHERE name = 'English';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'German'), id, 'German' FROM language WHERE name = 'English';

INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'English'), id, 'Anglais' FROM language WHERE name = 'French';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'French'), id, 'Français' FROM language WHERE name = 'French';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'Japanese'), id, 'Japonais' FROM language WHERE name = 'French';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'German'), id, 'Allemand' FROM language WHERE name = 'French';

INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'English'), id, '英語' FROM language WHERE name = 'Japanese';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'French'), id, 'フランス語' FROM language WHERE name = 'Japanese';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'Japanese'), id, '日本語' FROM language WHERE name = 'Japanese';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'German'), id, 'ドイツ語' FROM language WHERE name = 'Japanese';

INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'English'), id, 'Englisch' FROM language WHERE name = 'German';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'French'), id, 'Französisch' FROM language WHERE name = 'German';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'Japanese'), id, 'Japanisch' FROM language WHERE name = 'German';
INSERT INTO language_local (language_id, local_id, name)
SELECT (SELECT id FROM language WHERE name = 'German'), id, 'Deutsch' FROM language WHERE name = 'German';




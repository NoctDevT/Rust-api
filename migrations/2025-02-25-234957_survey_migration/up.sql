CREATE TABLE IF NOT EXISTS questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    survey_id UUID NOT NULL REFERENCES surveys(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL
);

ALTER TABLE responses 
ADD COLUMN question_id UUID NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE responses 
ADD CONSTRAINT fk_question FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE;


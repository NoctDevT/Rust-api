DELETE FROM questions WHERE survey_id IN (
    '550e8400-e29b-41d4-a716-446655440000',
    '123e4567-e89b-12d3-a456-426614174000'
);
DELETE FROM surveys WHERE id IN (
    '550e8400-e29b-41d4-a716-446655440000',
    '123e4567-e89b-12d3-a456-426614174000'
);

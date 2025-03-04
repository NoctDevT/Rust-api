INSERT INTO surveys (id, title, created_at) VALUES
    ('550e8400-e29b-41d4-a716-446655440000', 'Customer Satisfaction', NOW()),
    ('123e4567-e89b-12d3-a456-426614174000', 'Product Feedback', NOW());

INSERT INTO questions (id, survey_id, question_text) VALUES
    ('a1b2c3d4-e89b-12d3-a456-426614174001', '550e8400-e29b-41d4-a716-446655440000', 'How satisfied are you?'),
    ('a1b2c3d4-e89b-12d3-a456-426614174002', '550e8400-e29b-41d4-a716-446655440000', 'Would you recommend us?'),
    ('b2c3d4e5-e89b-12d3-a456-426614174003', '123e4567-e89b-12d3-a456-426614174000', 'What do you like about our product?'),
    ('b2c3d4e5-e89b-12d3-a456-426614174004', '123e4567-e89b-12d3-a456-426614174000', 'What improvements do you suggest?');

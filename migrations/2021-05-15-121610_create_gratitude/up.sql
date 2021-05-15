CREATE TABLE "gratitude" (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  item VARCHAR(255) NOT NULL
);

INSERT INTO gratitude (
    created_at,
    item
) VALUES
(
    '2021-05-12 12:00:00',
    'A gratitude list'
),
(
    '2021-05-12 12:00:00',
    'Yay'
);

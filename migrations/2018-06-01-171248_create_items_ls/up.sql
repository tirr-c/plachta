CREATE TYPE item_type_ls AS ENUM (
  'material', 'disposable', 'attack', 'recover',
  'assist', 'explore', 'craft', 'core',
  'sub', 'weapon', 'armor', 'accessory',
  'important', 'book'
  -- '소재', '소모품', '공격', '회복',
  -- '보조', '탐색', '조합', '코어',
  -- '서브', '무기', '방어구', '장식품',
  -- '중요', '서적'
);

CREATE TYPE item_category_ls AS ENUM (
  'plant', 'magic_grass', 'honeycomb', 'fruit',
  'mushroom', 'foodstuff', 'animal', 'flower',
  'insect', 'fish', 'puniball', 'dragon',
  'scent', 'water', 'oil', 'gas',
  'paper', 'fuel', 'powder', 'wood',
  'thread', 'cloth', 'clay', 'sand',
  'ore', 'gem', 'metal', 'medicine_ingredient',
  'poison_ingredient', 'mysterious', 'ericsil', 'activator',
  'anima', 'counteractive', 'magic_tool', 'bomb',
  'medicine', 'sweet', 'food', 'weapon_material',
  'armor_material', 'weapon_core', 'weapon_parts', 'metal_wand',
  'alchemic_gun', 'bow', 'wand', 'sword',
  'book', 'armor', 'accessory', 'collecting_tool',
  'important'
  -- '식물류', '마법의 풀', '벌집', '과일',
  -- '버섯', '식재', '동물 소재', '꽃',
  -- '곤충', '어패류', '푸니푸니 구슬', '용 소재',
  -- '향기', '물', '기름', '기체',
  -- '종이', '연료', '화약', '목재',
  -- '실 소재', '직물', '점토', '모래',
  -- '광석', '보석', '금속', '약 재료',
  -- '독 재료', '신비의 힘', '에릭실', '활성',
  -- '아니마', '중화제', '마법의 도구', '폭탄',
  -- '약품', '과자', '식품', '무기 소재',
  -- '방어구 소재', '무기 코어', '무기 파츠', '금속 지팡이',
  -- '연금총', '활', '지팡이', '검',
  -- '책', '방어구', '장식품', '채집 도구',
  -- '중요'
);

CREATE TABLE items_ls (
  id          serial PRIMARY KEY,
  name        varchar(60) NOT NULL UNIQUE,
  lv          integer NOT NULL,
  ty          item_type_ls NOT NULL,
  base_price  integer,
  is_catalyst boolean NOT NULL DEFAULT 'f',

  CONSTRAINT valid_level CHECK (lv > 0 AND lv <= 50),
  CONSTRAINT valid_price CHECK (base_price IS NULL OR base_price > 0)
);

CREATE TABLE category_map_ls (
  id        serial PRIMARY KEY,
  item_id   integer NOT NULL REFERENCES items_ls (id),
  category  item_category_ls NOT NULL,

  UNIQUE (item_id, category)
);

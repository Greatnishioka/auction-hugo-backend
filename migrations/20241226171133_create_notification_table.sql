-- Add migration script here

CREATE TABLE productions (

    product_id SERIAL PRIMARY KEY,                  
    product_title TEXT NOT NULL,                   
    product_image_url TEXT NOT NULL,               
    product_price INT NOT NULL,            
    product_openprice INT NOT NULL,           
    product_tags JSON NOT NULL,                    
    product_text JSON NOT NULL,                    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    product_thresholds JSON NOT NULL,              
    product_sold_status INT NOT NULL               
);



-- CREATE TABLE notifications (
--     notification_id SERIAL PRIMARY KEY UNIQUE NOT NULL,
--     notification_status Number NOT NULL,
--     notification_title TEXT NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- 入力例
-- CREATE TABLE notifications (
--    notification_id 1,
--    notification_status 1, // 1 = 落札, 2 = 運営からのメッセージ
--    notification_title TEXT NOT NULL, // 通知タイトル
--    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );
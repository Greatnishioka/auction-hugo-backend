-- Add migration script here

CREATE TABLE productions_renew (

    product_id SERIAL PRIMARY KEY,                  
    product_title TEXT NOT NULL,                   
    product_image_url TEXT NOT NULL,               
    product_price INT NOT NULL,            
    product_openprice INT NOT NULL,
    product_progress INT NOT NULL,           
    product_tags JSON NOT NULL,                    
    product_text JSON NOT NULL,                    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMP,
    product_thresholds JSON NOT NULL,              
    product_sold_status INT NOT NULL
              
);
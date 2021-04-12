WITH eb AS (
    SELECT * FROM electricity_bids
    WHERE
            postal_code_lower < $1 AND
            postal_code_upper > $1 AND
            true = $2
),

     gb AS (
         SELECT * FROM gas_bids
         WHERE
                 postal_code_lower < $1 AND
                 postal_code_upper > $1 AND
                 true = $3
     ),

     hb AS (
         SELECT * FROM heat_bids
         WHERE
                 postal_code_lower < $1 AND
                 postal_code_upper > $1 AND
                 true = $4
     ),

     ib AS (
         SELECT * FROM internet_bids
         WHERE
                 postal_code_lower < $1 AND
                 postal_code_upper > $1 AND
                 true = $5
     )

SELECT jsonb_strip_nulls(jsonb_agg(res)) as json
FROM (
         SELECT bids.id,
                electricity_bid,
                gas_bid,
                heat_bid,
                internet_bid,
                ARRAY(SELECT db FROM discount_bids db WHERE bid_id = bids.id) AS discounts
         FROM bids
                  LEFT JOIN eb electricity_bid on bids.id = electricity_bid.id
                  LEFT JOIN gb gas_bid on bids.id = gas_bid.id
                  LEFT JOIN hb heat_bid on bids.id = heat_bid.id
                  LEFT JOIN ib internet_bid on bids.id = internet_bid.id
         WHERE electricity_bid.id IS NOT NULL
            OR gas_bid.id IS NOT NULL
            OR heat_bid.id IS NOT NULL
            OR internet_bid.id IS NOT NULL
     ) as res
## GraphQL server setup
reference for graphql server: https://oliverjumpertz.com/how-to-build-a-powerful-graphql-api-with-rust/

- run `cargo run` and launch `localhost:8080`.

- Execute the query:
  ```
  query Query {
      root
  }
  ```

- For inserting:
  ```
  mutation {
    unsafeInsert(
      leaf: {
      	address: [78, 171, 15, 165, 90, 108, 113, 192, 194, 59, 208, 230, 124, 6, 104, 43, 76, 231, 138, 127],
        lowCoin: 0,
        highCoin: 10
      }) {
      leaf,
      root
    }
  }
  ```

- For sending:
  ```
  mutation {
    unsafeSend(
      leaf: {
        address: [78, 171, 15, 165, 90, 108, 113, 192, 194, 59, 208, 230, 124, 6, 104, 43, 76, 231, 138, 127],
        lowCoin: 0,
        highCoin: 10
      },
      key: 0,
      highestCoinToSend: 5,
      recipient: [218, 30, 169, 125, 76, 109, 114, 49, 37, 178, 110, 74, 20, 65, 195, 153, 208, 234, 11, 228],
      sig: [
        228, 164, 161,  86, 150,  56,  88,  85,
        172,  99, 204,  40,  86, 122, 129,  60,
        124, 240,  72, 143, 236, 153,  69, 172,
        173,  44, 142, 217, 241, 167, 164, 117,
        42, 163, 171, 214, 152, 150, 246,
        73, 188,  76, 150,  86,  73, 212,
        91,  81, 144, 132,  31, 102, 178,
        232, 126, 139, 171, 194, 239, 165,
        105, 149, 199, 178
      ]
      pubkey: [
        4, 218, 175, 173, 110, 253, 117,  16, 114, 241,  71,
      131, 180, 234,   4,  43,  74, 138, 113, 157,  84,  58,
      188,  53, 189,  44, 126, 239, 253,  71, 143, 158,  58,
      218, 248,  50, 123, 163,  50, 254,  36, 202,   2, 172,
      56,   1, 162, 132, 252, 234, 157, 196, 160, 124, 109,
      212, 115, 244,   0, 188,  61, 175,  87,  28,  79,
    ]
    )
  }
  ```

- Now query the root again to see the update. Also try `localhost:8080/health`.

## Postgres DB setup
- Install postgresql locally.
- Run `psql`.
- Create a user `dev` to manage db.
- Create arcpay database which will host all our tables: `create databse arcpay`.
- Create table to store continuously updating merkle tree ("state merkle tree" vs "contract state merkle tree" which is attached to the root stored in contract):
  ```sql
  CREATE TABLE state_tree (
    leaf bigint,
    owner integer,
    coin_low bigint,
    coin_high bigint,
    index integer,
    PRIMARY KEY (index),
    UNIQUE (leaf)
  );
  ```
- Insert into table:
  ```sql
  INSERT INTO state_tree (leaf, owner, coin_low, coin_high, index)
  VALUES (123456789, 1, 1000, 5000, 10);
  ```

## RabbitMQ setup
- Install rabbitMQ locally.
- Create a local rabbitMQ server with default settings:
  - Should be accessible at `amqp://guest:guest@localhost:5672`.
  - You can open `http://localhost:15672/` and use `guest` as username & password.

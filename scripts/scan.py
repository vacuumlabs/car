

import os
import sys
import psycopg2
from psycopg2.extras import DictCursor
import requests
import time
import dataclasses
from typing import List

def connection():
    return psycopg2.connect(os.environ["DATABASE_URL"])


@dataclasses.dataclass
class Task:
    id: int
    address: bytes
    last_block: int
    tags: List[int]
    services: List[int]
    query: str
    chain: int
    enabled: bool

"""
Logic
CREATE TEMP TABLE temp_address (chain integer, hash bytea); -- create temp table
INSERT INTO temp_address VALUES (1, decode('ba123453', 'hex'));
INSERT INTO address (hash, chain) SELECT hash, chain from temp_address WHERE hash not in (SELECT hash from address)

INSERT INTO "tasks" ("last_block", "tags", "services", "query", "address")
VALUES (0, ARRAY[1], ARRAY[1], 'test', decode('1234', 'hex'));
"""


tasks = []
with connection() as conn:
    with conn.cursor(cursor_factory=DictCursor) as cursor:
        cursor.execute("SELECT * FROM tasks WHERE enabled is TRUE ORDER BY id")
        if cursor.rowcount == 0:
            sys.exit(0)

        for task in cursor.fetchall():
            t = Task(**task)
            t.address = t.address.tobytes()
            tasks.append(t)

        
for task in tasks:
    print(task)
    running = True
    last_block = 0
    while running:
        with connection() as conn:
            with conn.cursor() as cursor:

                cursor.execute("CREATE TEMP TABLE temp_address (chain integer, hash bytea);")
                url = task.query.format(start=task.last_block)
                print(url)
                time.sleep(0.2)
                r = requests.get(url)
                data = r.json()
                
                # We have all records
                if data["result"] is None or len(data["result"]) <= 1 or (last_block == task.last_block and last_block > 0):
                    cursor.execute("UPDATE tasks SET enabled = FALSE, last_block = %s WHERE id = %s", (task.id, task.last_block))
                    running = False
                    continue

                last_block = task.last_block

                # Collect new addresses
                for record in data["result"]:
                    address_from = bytes.fromhex(record["from"][2:])
                    address_to = bytes.fromhex(record["to"][2:])
                    task.last_block = max(int(record["blockNumber"]), task.last_block)

                    cursor.execute(
                        "INSERT INTO temp_address VALUES (%s, %s), (%s, %s)",
                        (task.chain, address_from, task.chain, address_to)
                    )

                # Insert new addresses
                cursor.execute("""
                    INSERT INTO 
                        address (hash, chain)
                    (
                        SELECT
                            DISTINCT T.hash, T.chain
                        FROM
                            temp_address T
                            LEFT JOIN address A
                                ON A.hash = T.hash AND A.chain = T.chain
                        WHERE 
                            A.id IS NULL
                    );
                """)

        with connection() as conn:
            with conn.cursor() as cursor:

                for record in data["result"]:
                    address_from = bytes.fromhex(record["from"][2:])
                    address_to = bytes.fromhex(record["to"][2:])
                    hash = bytes.fromhex(record["hash"][2:])

                    try:
                        cursor.execute("""
                            INSERT INTO 
                                transaction (chain, hash, amount, "from", "to")
                                VALUES (
                                    %s, %s, %s,
                                    ARRAY(SELECT id FROM address WHERE chain = %s AND hash = %s), 
                                    ARRAY(SELECT id FROM address WHERE chain = %s AND hash = %s)
                                )
                        """, (task.chain, hash, int(record["value"]),  task.chain, address_from, task.chain, address_to))
                        conn.commit()

                    except Exception as e:
                        conn.commit()
                        print(e)

                cursor.execute("UPDATE tasks SET last_block = %s WHERE id = %s", (task.last_block, task.id))
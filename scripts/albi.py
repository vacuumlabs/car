import json
import dataclasses
import psycopg2

import os
import re
from typing import List


address_regexp = re.compile(r'(\s+:\w*)?0x(?P<address>[a-fA-F0-9]+)')

@dataclasses.dataclass
class Chain:
    name: str
    addresses: List[str]
    api_key: str
    id: int

chains = [
    Chain(name="Ethereum", addresses=[], api_key='', id=1),
    Chain(name="Polygon (Matic)", addresses=[], api_key='', id=2),
]


def connection():
    return psycopg2.connect(os.environ["DATABASE_URL"])


def get_tags(cursor):
    tags = {}
    cursor.execute("SELECT title, id FROM tag;")
    for tag in cursor.fetchall():
        tags[tag[0]] = tag[1]
    return tags

def get_services(cursor):
    services = {}
    cursor.execute("SELECT title, id FROM service;")
    for service in cursor.fetchall():
        services[service[0]] = service[1]
    return services

                
data = {}
with open("dapps.json") as fp:
    data = json.load(fp)


new_tags = {}
new_services = {}
for dapp in data["dapps"]:
    for chain in chains:
        if chain.name in dapp["chains"]:
            new_tags[dapp["category"]] = True
            new_services[dapp["title"]] = True            

new_services = [s for s in new_services.keys() if s]
new_tags = [t for t in new_tags.keys() if t]

with connection() as conn:
    with conn.cursor() as cursor:        
        tags = get_tags(cursor)
        for tag in new_tags:
            if tag not in tags:
                cursor.execute("INSERT INTO tag (title) VALUES(%s)", (tag,))
        tags = get_tags(cursor)

    with conn.cursor() as cursor:
        services = get_services(cursor)
        for service in new_services:
            if service not in services:
                cursor.execute("INSERT INTO service (title) VALUES(%s)", (service,))
        services = get_services(cursor)




with connection() as conn:
    with conn.cursor() as cursor:
        cursor.execute("CREATE TEMP TABLE temp_address (chain integer, hash bytea);")

        for dapp in data["dapps"]:
            for chain in chains:
                if chain.name in dapp["chains"]:
                    for address in dapp["addresses"]:
                        if match := address_regexp.match(address):        
                            hex_address = match.group("address")
                            try:
                                cursor.execute("INSERT INTO temp_address VALUES (%s, %s)", (chain.id, bytes.fromhex(hex_address)))

                            except Exception as e:
                                print(e)

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
        for dapp in data["dapps"]:
            for chain in chains:
                if chain.name in dapp["chains"]:
                    for address in dapp["addresses"]:
                        if match := address_regexp.match(address):        
                            hex_address = match.group("address")

                            try:
                                cursor.execute("""
                                    -- Update existing address
                                    UPDATE 
                                            address A
                                        SET
                                            tags = array_unique(A.tags || %s),
                                            services = array_unique(A.services || %s)
                                        WHERE A.chain = %s and A.hash = %s

                                    """, (tags[dapp["category"]], services[dapp["title"]], chain.id, bytes.fromhex(hex_address)))

                            except Exception as e:
                                print(e)
                                #os.exit(1)
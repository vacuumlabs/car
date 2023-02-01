import codecs
import dataclasses
import os
import re
from pathlib import Path
from typing import Dict, List

import psycopg2
import requests
from dataclass_wizard import YAMLWizard

address_regexp = re.compile(r'(arbitrum:)?0x(?P<address>[a-fA-F0-9]+)')


@dataclasses.dataclass
class Chain:
    name: str
    addresses: List[str]
    api_key: str
    id: int

@dataclasses.dataclass
class Config(YAMLWizard):
    chains: List[Chain]

config = Config.from_yaml_file("config.yaml")
conn = psycopg2.connect(os.environ["DATABASE_URL"])

r = requests.get("https://api.llama.fi/protocols")
services_json = r.json()

new_tags = {}
for service in services_json:
    new_tags[service["category"]] = True
    new_tags[service["gecko_id"]] = True
new_tags = [t for t in new_tags.keys() if t]

new_services = {}
for service in services_json:
    new_services[service["name"]] = True
new_services = [s for s in new_services.keys() if s]

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


tags = {}
services = {}
with conn:
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


    with conn.cursor() as cursor:
        for service in services_json:
            if isinstance(service["address"], str):
                if match := address_regexp.match(service["address"]):        
                    hex_address = match.group("address")

                    for chain in config.chains:
                        for address in chain.addresses:
                            query = address.format(address=f'0x{hex_address}', token=chain.api_key, start="{start}")
                            add_tags = []
                            add_services = []
                            if service["gecko_id"] in tags:
                                add_tags.append(tags[service["gecko_id"]])

                            if service["category"] in tags:
                                add_tags.append(tags[service["category"]])

                            if service["name"] in services:
                                add_services.append(services[service["name"]])

                            
                            cursor.execute("""
            INSERT INTO tasks (address, last_block, tags, services, query, chain, enabled)
            VALUES (decode(%s, 'hex'), %s, %s, %s, %s, %s, %s);
                            """, (hex_address, 0, add_tags, add_services, query, chain.id, True))

    with conn.cursor() as cursor:
        cursor.execute("""
        -- Insert address what is not present
        INSERT INTO
                address (chain, hash, services, tags)
        (
            SELECT 
                DISTINCT ON (T.chain, T.address)
                T.chain, T.address, T.services, T.tags 
            FROM
                tasks T
                LEFT JOIN address A
                ON
                    T.address = A.hash AND T.chain = A.chain

            WHERE A.id is NULL
        );
        """)
        cursor.execute("""
        -- Update existing address
        UPDATE 
                address A
            SET
                tags = array_unique(A.tags || T.tags),
                services = array_unique(A.services || T.services)
            FROM (
                SELECT address, chain, tags, services FROM tasks
            ) as T
            WHERE A.chain = T.chain and A.hash = T.address

        """)


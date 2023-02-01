import os
import psycopg2
from psycopg2.extensions import parse_dsn

def get_cursor():
    #return psycopg2.connect(**parse_dsn(os.environ["DATABASE_URL"]))
    return psycopg2.connect(os.environ["DATABASE_URL"])
    
    

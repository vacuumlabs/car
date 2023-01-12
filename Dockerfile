FROM rust:1.65


COPY ./car .
COPY ./migration .

CMD ["sh", "-c", "./migration up && ./car"]
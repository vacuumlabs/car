FROM debian:bookworm

COPY ./car .
COPY ./migration .

CMD ["sh", "-c", "./migration up && echo Running... &&  ./car"]
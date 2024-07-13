# Continuebee Spring Server Implementation
This project a deployable spring server for continuebee

---

## Endpoints

---
### POST
### user/create
###  Parameters:
Content-Type: application/json
```
{ 
  "timestamp": 1719881098285,
  "pubKey": "028f73a2dfa092d3a676310d8625b085f2f60702498803bd124cd475287bbfa126",
  "hash": "password",
  "signature": "64724c3de2ce61bdb10945e3b53aacc8f5fab5913127158a1ac7ec08d9e996a31449953fef9c5640a440a643992d9b9232e3eda05c5ef0feac638f823d7ebd5a"
}

signature is of message timestamp + pubkey + hash ie: 1719881098285028f73a2dfa092d3a676310d8625b085f2f60702498803bd124cd475287bbfa126password
```
### Response:
HTTP/1.1 202 accepted

Content-Type: application/json
```
{
    "userUUID": "3e00c9fd-6588-4b34-8e0c-daf40a8adeec"
}
```
HTTP/1.1 400 bad request

Content-Type: application/json
```
{
    "status: 400,
    "message": "Invalid request parameters provided",
    "errorDetails": [
        "Invalid Key Format"
    ]
}
```

---
### GET
### user/{{uuid}}?timestamp={{timestamp}}&hash={{password}}&signature={{signature of (timestamp + uuid + password)}}
### Parameters:
URL Parameter
```
uuid = 3e00c9fd-6588-4b34-8e0c-daf40a8adeec
```

Query Parameters
```
timestamp = 1719881098285
hash = password
signature = 64724c3de2ce61bdb10945e3b53aacc8f5fab5913127158a1ac7ec08d9e996a31449953fef9c5640a440a643992d9b9232e3eda05c5ef0feac638f823d7ebd5a

signature is of message timestamp + uuid + hash
```
### Response:
HTTP/1.1 200 OK

Content-Type: application/json
```
{
    "status": 202,
    "message": "3e00c9fd-6588-4b34-8e0c-daf40a8adeec"
}
```

HTTP/1.1 400 bad request

Content-Type: application/json
```
{
    "status: 400,
    "message": "Invalid request parameters provided",
    "errorDetails": [
        "Timestamp only valid within {{VALID_WINDOW_IN_SECONDS}}";
        "Now: {{now timestamp}}"
        "Provided: {{message timestamp}}",
        "Signature verification failed"
    ]
}
```

---
### PUT
### user/update-hash
###  Parameters:
Content-Type: application/json
```
{ 
  "timestamp": 1719881098285,
  "userUUID": "028f73a2dfa092d3a676310d8625b085f2f60702498803bd124cd475287bbfa126",
  "hash": "password",
  "newHash": "newPassword"
  "signature": "81fd959d0e5deb35e960ddc816b90be3cf179795cc234757c36d3776559909e05d852e8d95a79dca8a28b83b198c8c4f8f4acb6337a7cdab4f7b3144c02150aa"
}

signature is of message timestamp + userUUID + hash + newHash
```
### Response:
HTTP/1.1 202 Accepted

Content-Type: application/json
```
{
    "userUUID": "3e00c9fd-6588-4b34-8e0c-daf40a8adeec"
}
```
HTTP/1.1 400 bad request

Content-Type: application/json
```
{
    "status: 400,
    "message": "Invalid request parameters provided",
    "errorDetails": [
        "Timestamp only valid within {{VALID_WINDOW_IN_SECONDS}}";
        "Now: {{now timestamp}}"
        "Provided: {{message timestamp}}",
        "Signature verification failed"
    ]
}
```

---

### DELETE
### user/delete
###  Parameters:
Content-Type: application/json
```
{ 
  "timestamp": 1719881098285,
  "userUUID": "028f73a2dfa092d3a676310d8625b085f2f60702498803bd124cd475287bbfa126",
  "hash": "password",
  "signature": "7455581755ebce4a07fe7bb1a8825dd21b946e9e7de87456a0d450e1877c98fae849714bdd986810732b8157a91e5d5e81233a33ae51fe238713ee18fce7b91d"
}

signature is of message timestamp + userUUID + hash
```
### Response:
HTTP/1.1 200 Ok

Content-Type: application/json
```
{
    true
}
```
HTTP/1.1 400 bad request

Content-Type: application/json
```
{
    "status: 400,
    "message": "Invalid request parameters provided",
    "errorDetails": [
        "Timestamp only valid within {{VALID_WINDOW_IN_SECONDS}}";
        "Now: {{now timestamp}}"
        "Provided: {{message timestamp}}",
        "Signature verification failed"
    ]
}
```

---

## Technologies Used
This project is built using the following technologies:

- [Spring Boot](https://spring.io/projects/spring-boot): An open-source Java-based framework used to create stand-alone applications that are easy to deploy.
- [Docker](https://www.docker.com/): An open-source platform that automates the deployment, scaling, and management of applications by containerization.
- [Gradle](https://gradle.org/): An open-source build automation system used to define project configurations for Java.
- [Liquibase](https://www.liquibase.org/): An open-source library for tracking, managing, and applying database schema changes.
- [MyBatis](https://mybatis.org/): A Java-based open-source persistence framework that provides a simplified, minimalistic approach for integrating SQL databases.
- [Postgres](https://www.postgresql.org/): An open-source object-relational database management system (ORDBMS).

---
## Installation
To install this project:

With docker & docker-compose installed run the following command from the root project folder: `docker compose up -d`




# SQL

Structured Query Language (구조적 질의어)

SQL은 추상적으로 말하자면 DB를 사용하는 데에 쓰이는 언어다.

예를 들어 특정 테이블에서 데이터를 가져오는 SQL 문장은 다음과 같다.

```
# MySQL(MariaDB), Oracle DB, MS SQL 동일
SELECT * FROM table_name
```

그렇다면 SQL이란 것은 구체적으로 말했을 땐 무엇인가?

SQL은 DB 중에서도 관계형 DB에 사용하는 쿼리 언어다.

Relational Database라 하며, 줄여서 RDB라 하기도 한다.

RDB는 여러 DB 종류 중에서도 가장 널리 쓰이는 유형이며,
그래서 DB를 크게 두 분류로 나눴을 때 SQL, NoSQL로 나누기도 한다.

NoSQL은 SQL을 무조건 배제한다고 하여 NoSQL인 것은 아니다.

NoSQL은 'Not Only SQL'이라는 뜻이며,
SQL 외에도 다른 유형의 DB까지 포함해서 부르는 말이지,
반드시 해당 DB에서 SQL 기능을 배제해야만 NoSQL로 분류되는 것은 아니다.

RDB가 아니라면 SQL이 아닌, 각 DB 종류에 맞는 쿼리어를 써야만 한다.

그렇다면 SQL끼리의 문법은 모두 동일한가? 그렇지는 않다.

현재 사람들 사이에서 널리 쓰이는 RDB의 종류는 다음과 같다.

* MySQL(MariaDB)
* Oracle DB
* MS SQL
* PostgreSQL
* etc.

위에 언급한 SELECT 문법에서 알 수 있겠지만,
SQL 간의 문법은 대체로는 비슷한 편이다.

하지만 구체적으로 무슨 DB를 쓰느냐에 따라서 세부사항은 조금씩 달라진다.

일반적으로 사람들에게 SQL을 가르칠 때에는
가장 널리 쓰이는 MySQL 문법을 기준으로 가르치는 경우가 많다.

MariaDB는 라이센스 등의 문제 때문에 따로 나온 DB지만,
문법 자체는 MySQL과 거의 동일하여 사실상 MySQL이라고 봐도 된다.

그런데 기업에서는 Oracle DB를 사용하는 경우도 많아서,
강의를 할 때 Oracle DB와의 차이를 같이 가르치거나,
아니면 아예 Oracle DB를 기준으로 가르치는 경우도 많다.

MySQL이나 MariaDB는 기업보단 개인이나 소규모 집단에서 사용하는 경우가 많다.

MS SQL, PostgreSQL도 꽤 쓰이지만,
SQL 자체를 입문할 때 그 두 개를 기준으로 강의를 진행하는 경우는
그다지 많지 않았던 것 같다.

실제로 설치할 DB로 내가 추천하는 것은 MariaDB인데,
그 이유는 단순히 MySQL이나 Oracle DB는 용량도 크고 설치나 삭제 과정이 복잡하기 때문이다.
성능 저하도 다소 유발할 수 있어서, 컴퓨터에 설치하는 것이 좀 꺼려질 수도 있다.

그에 반해 MariaDB는 훨씬 더 가볍다.
그래서 문법 기준으로 보면 사실상 MySQL을 추천한다고 보면 된다.

게다가 개인용으로 쓴다고 가정했을 때 MariaDB가 더 적합하다.
나중에 다른 DB를 쓴다고 하더라도,
MySQL은 애초에 문법이 같기 때문에 걱정할 필요가 없고,
Oracle DB, MS SQL, PostgreSQL 같은 경우에도
일단 기본적으로는 SQL이라 약간의 차이만 익히면 금세 적응할 수 있다.


# 2과목 1장 (Subject 2 - Chapter 1)

* **2과목: SQL 기본 및 활용**
  * **1장: SQL 기본**
    * 1절: 관계형 데이터베이스 개요
    * 2절: DDL
    * 3절: DML
    * 4절: TCL
    * 5절: WHERE 절
    * 6절: FUNCTION
    * 7절: GROUP BY, HAVING 절
    * 8절: ORDER BY 절
    * 9절: 조인

## RDB 개요

관계형 데이터베이스 (RDB, Relational Database)

* DB
  * 여러 사람이 공유하여 사용할 목적으로 체계화해 통합, 관리하는 데이터 집합
* DBMS
  * DB를 관리하기 위한 소프트웨어
  * (ex: MySQL, OracleDB, MS SQL)
* 계층형 DB
  * 자료구조: 트리형태
  * 표현관계: 1대N 관계
* 네트워크 DB
  * 자료구조: 오너-멤버 형태
  * 표현관계: 1대N, M대N
* 관계형 DB
  * 자료구조: 릴레이션
  * 릴레이션으로 집합 연산, 관계 연산

관계형 DB가 많이 사용되고, DB란 말 자체가 사실상 RDB를 가리키는 경우도 많음.

## RDB 구성

* 릴레이션 -> 테이블
* 릴레이션의 조인 연산을 통해 합집합, 교집합, 차집합 등을 생성 가능
* 릴레이션을 사용한 집합 연산, 관계 연산 가능 -> 다양한 형태로 데이터 조회
* SQL 사용 -> 누구나 쉽게 DB 조회 가능

## 집합연산 종류

* 합집합(Union)
  * 릴레이션 2개 -> 1개
  * 중복행은 1번만 조회
* 차집합(Difference)
  * 본래 릴레이션에는 있으나 다른 릴레이션엔 없는 것 조회
* 교집합(Intersection)
  * 릴레이션 2개 안에 공통된 것을 조회
* 곱집합(Catesian Product)
  * 각 릴레이션의 모든 데이터를 조합하여 연산

## 관계연산 종류

* 선택 연산(Selection)
  * 릴레이션에서 조건에 맞는 행만 조회
* 투영 연산(Projection)
  * 릴레이션에서 조건에 맞는 속성만 조회
* 결합 연산(Join)
  * 여러 릴레이션의 공통된 속성을 사용하여 새로운 릴레이션 생성
* 나누기 연산(Division)
  * 나누는 릴레이션의 모든 속성과 관련 있는 기준 릴레이션의 속성들을 반환

## 테이블

테이블 구조: 기본키/행(튜플)/열(속성)/외래키

외래키: 관계연산 중 결합연산을 수행하기 위해 사용되는 키

## SQL 종류

* DDL (Data Definition Language)
  * 관계형 DB의 구조를 정의
  * CREATE: DB, 테이블 생성
  * ALTER: 테이블 수정
  * DROP: DB, 테이블 삭제
  * RENAME: 테이블 이름 변경
  * TRUNCATE: 테이블 초기화
* DML (Data Manipulation Language)
  * 테이블에서 데이터를 입력, 수정, 삭제, 조회함
  * SELECT: DB에서 데이터 검색
  * INSERT: 테이블에 데이터 추가
  * UPDATE: 테이블 내 데이터 수정
  * DELETE: 테이블에서 데이터 삭제
* DCL (Data Control Language)
  * DB 사용자에게 권한을 부여 및 회수함
  * GRANT: 권한 부여
  * REVOKE: 권한 회수
* TCL (Transaction Control Language)
  * 트랜잭션을 제어하는 명령어
  * COMMIT: 현재 처리한 작업 확정
  * ROLLBACK: 현재 처리한 작업 취소
  * SAVEPOINT: ROLLBACK 시 해당 시점으로 복구

## 트랜잭션의 4가지 특징 (ACID)

* Atomicity (원자성)
  * 연산 전부가 실행되든지, 아니면 전혀 실행되지 않아야 함
  * 즉 일부만 실행된다거나 해서는 안 됨
* Consistency (일관성)
  * 트랜잭션의 수행 결과로 DB 상태가 모순되지 않아야 함
* Isolation (고립성)
  * 트랜잭션 실행 중 결과는 다른 트랜잭션이 알 수 없음
* Durability (영속성)
  * 트랜잭션 실행 후 결과는 영구적으로 보장됨

## SQL 실행 순서

* Parsing
  * SQL문 문법 확인 및 구문 분석
  * 분석된 SQL문은 캐시에 저장
* Execution
  * 파싱한 결과대로 SQL 실행
* Fetch
  * 데이터를 읽어서 전송

## WHERE 절

## GROUP BY, HAVING 절

## ORDER BY 절

## 조인


# 2과목: 파일 시스템 관련 명령어

* **2과목: 리눅스 운영 및 관리**
  * 1항목: 파일 시스템 관련 명령어
    * 권한 및 그룹 설정
    * 파일 시스템의 관리
  * 2항목: Shell
    * 개념 및 종류
    * 환경 설정
  * 3항목: 프로세스 관리
    * 개념 및 유형
    * 프로세스 유틸리티
  * 4항목: 에디터
    * 에디터의 종류
    * 에디터 활용
  * 5항목: 소프트웨어 설치
    * 개념 및 사용법
    * 소프트웨어 설치 및 삭제
  * 6항목: 장치 설정
    * 주변장치 연결 및 설정
    * 주변장치 활용

## 파일 시스템 종류

* NFS
* EXT2
* EXT3
* EXT4
* ReiserFS
* JFS
* XFS
* NTFS
* ISO 9660
* UDF

## fdisk

파일 시스템 관리에 사용하는 명령어

fdisk [옵션] [장치명]

옵션 종류:

* -l
* -s 파티션명
* -v
* -t

fdisk 내부 명령어:

* q: 저장하지 않고 종료
* p: 현재 디스크 정보 출력
* d: 파티션 삭제
* n: 파티션 생성
* t: 파티션 속성 변경

## mkfs

파티션 포맷 및 파일 시스템 구축

```
# 예시
mkfs -t ext4 /dev/sdb1
```

## Shell 개념 및 종류

Shell:

* 대화식 인터페이스를 통해 사용자의 명령어를 커널에 전달하고 이후 결과를 받아 다시 사용자에게 전달
* 로그인할 때 실행되어 사용자별로 사용 환경 설정이 가능함. 즉 사용자마다 사용하는 셸이 다를 수 있음.
* 강력한 스크립팅이 가능하고, 프로세스의 포그라운드 및 백그라운드 설정이 가능.

Shell 종류:

* 본셸 계열
  * /bin/sh (본셸)
  * /bin/ksh (콘셸)
  * /bin/bash (배쉬셸)
  * /bin/zsh (지셸)
* C셸 계열
  * /bin/csh (C셸)
  * /bin/tcsh (TC셸)

## 주요 환경 변수 (무조건 대문자)

* PATH
* HOME
* HOSTNAME
* USER
* DISPLAY
* PS1
* PS2
* PWD
* SHELL
* TERM
* TMOUT
* LANG
* PRINT
* MAIL

## 환경 변수 설정 관련 명령어

* export: 환경 변수 리스트 확인
* export 변수명=변수값: 환경 변수 변경 또는 등록
* export 변수명=$변수명:변수값: 기존 변수값에 변수값 추가
* echo $변수명: 변수값 확인
* unset 변수명: 변수 정의 해제

## 프로세스 관련 명령어

* ps
* pstree
* jobs
* bg/fg
* kill
* killall
* nice
* renice
* top
* nohup
* tail
* crontab

## 에디터 종류

* vi
* vim
* nvim
* emacs
* pico
* gedit
* xedit

## 소프트웨어 설치

* Debian 계열 (Debian, Ubuntu 등)
  * 패키지 툴: dbkg, apt-get, aptitude
* Red-Hat 계열 (Fedora, CentOS, RHEL 등)
  * 패키지 툴: rpm, yum
* OpenSUSE 계열
  * 패키지 툴: yaST, zypper

## 장치 관련 명령어

* 프린터 관련
  * BSD 계열
    * lpd, lpc, lpq, lpr, lpm
  * System V 계열
    * lp, lpstat, cancel
* 사운드카드 관련
  * alsactl
* 스캐너 관련
  * sane-find-scanner
  * scanimage
  * scanadf

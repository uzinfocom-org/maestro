<header>
<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/public/favicons/pinned.svg" alt="logo" height="100" align="left">
<h1 style="display: inline">Maestro</h1>

UIC GW dan osongina autentifikatsiya qiluvchi cli dastur

[![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/maestro?style=flat-square&logo=github)](https://github.com/uzinfocom-org/maestro)
[![Test CI](https://github.com/uzinfocom-org/maestro/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/maestro/actions/workflows/test.yml)
</header>

## Dastur haqida

Ba'zan GUI intefeysiga ega emas qurilmalar uchun UIC-GW dan autentifikatsiya qilish judayam qiyin. Shuning uchun
osongina konfiguratsion http klient orqali autentifikatsiya qilish tizimini yozib chiqdik.

## O'rnatish

Eng oxirgi reliz artifaktdan shu yerdan ko'chirib olishingiz kerak bo'ladi:
https://github.com/uzinfocom-org/maestro/releases. Keyin esa:

```shell
# Yordam uchun
./maestro --help

# Config faylni yaratib olish
./maestro create

# Config bilan amalga oshirish
./maestro config <config file>

# Qo'lda terilgan holda
./maestro credentials <url> <username> <password>
```

## Litsenziya

Ushbu kutubxona MIT litsenziyasi ostida tarqatiladi. Batafsil ma'lumot uchun [LICENSE](./license) ko'zdan kechiring!

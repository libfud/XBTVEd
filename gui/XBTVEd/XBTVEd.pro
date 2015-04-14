#-------------------------------------------------
#
# Project created by QtCreator 2015-04-14T01:53:20
#
#-------------------------------------------------

QT       += core gui
CONFIG   += c++11

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = XBTVEd
TEMPLATE = app


SOURCES += main.cpp\
        mainwindow.cpp \
    xbtved.cpp

HEADERS  += mainwindow.h \
    xbtved.h

FORMS    += mainwindow.ui

win32:CONFIG(release, debug|release): LIBS += -L$$PWD/../../target/debug/release/ -lXBTVEd-68102e438aa431ea
else:win32:CONFIG(debug, debug|release): LIBS += -L$$PWD/../../target/debug/debug/ -lXBTVEd-68102e438aa431ea
else:unix: LIBS += -L$$PWD/../../target/debug/ -lXBTVEd-68102e438aa431ea

INCLUDEPATH += $$PWD/../../target/debug
DEPENDPATH += $$PWD/../../target/debug

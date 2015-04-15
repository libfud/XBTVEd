#include "mainwindow.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    Q_INIT_RESOURCE(icons);
    Q_INIT_RESOURCE(font);

    QApplication app(argc, argv);
    app.setApplicationName("XBTVEd");

    MainWindow w;
    w.show();

    return app.exec();
}

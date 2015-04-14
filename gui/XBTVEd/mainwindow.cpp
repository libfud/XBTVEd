#include <QWidget>
#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    createActions();
    createMenus();
    createToolBars();
    xbtveditor = new App::XBTVEditor;
    setCentralWidget(xbtveditor);

    connect(xbtveditor->modified(), SIGNAL()
}

MainWindow::~MainWindow()
{
    delete ui;
    delete xbtveditor;
}

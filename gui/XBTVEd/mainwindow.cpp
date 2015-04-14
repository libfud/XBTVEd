#include <QWidget>
#include <QObject>
#include <QFileDialog>
#include <QCloseEvent>
#include <QLabel>
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
    std::string temp = xbtveditor->getSchedule();
    //QLabel temporary = new QLabel::(temp.c_str())
    //templabel = temporary
    //setCentralWidget(templabel);
}

MainWindow::~MainWindow()
{
    delete ui;
    delete xbtveditor;
}

void MainWindow::closeEvent(QCloseEvent *event)
{
    if (maybeSave()) {
        xbtveditor->saveAll();
        event->accept();
    } else {
        event->accept();
    }
}

void MainWindow::newFile()
{
    xbtveditor->newSchedule();
}

void MainWindow::open()
{
    QString fileName = QFileDialog::getOpenFileName(this);
    if (!fileName.isEmpty()) {
        xbtveditor->loadFile(fileName);
    }
}

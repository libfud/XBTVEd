#include <QWidget>
#include <QObject>
#include <QFileDialog>
#include <QMessageBox>
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
    //std::string temp = xbtveditor->getSchedule();
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

bool MainWindow::saveAs()
{
    QString fileName = QFileDialog::getSaveFileName(this);
    if (!fileName.isEmpty()) {
        return xbtveditor->saveAs(fileName);
    } else {
        return false;
    }
}

bool MainWindow::save()
{
    return xbtveditor->saveFile();
}

bool MainWindow::saveAll()
{
    return xbtveditor->saveAll();
}

void MainWindow::undo()
{
    xbtveditor->unDo();
}

void MainWindow::redo()
{
    xbtveditor->reDo();
}

void MainWindow::about()
{
    QMessageBox::about(this, tr("About XBTVEd"),
                       tr("<b>XBTVEd</b> is an editor to create schedules for Kodi."));
}

void MainWindow::buffersModified()
{
    setWindowModified(xbtveditor->anyBufModified());
}

void MainWindow::createActions()
{
    newAct = new QAction(QIcon(":/images/new.png"), tr("&New"), this);
    newAct->setShortcuts(QKeySequence::New);
    newAct->setStatusTip(tr("Create a new schedule"));
    connect(newAct, SIGNAL(triggered()), this, SLOT(newFile()));

    openAct = new QAction(QIcon(":/images/open.png"), tr("&Open..."), this);
    openAct->setShortcuts(QKeySequence::Open);
    openAct->setStatusTip(tr("Open an existing file"));
    connect(openAct, SIGNAL(triggered()), this, SLOT(open()));

    saveAct = new QAction(QIcon(":/images/save.png"), tr("&Save"), this);
    saveAct->setShortcuts(QKeySequence::Save);
    saveAct->setStatusTip(tr("Save this schedule."));
    connect(saveAct, SIGNAL(triggered()), this, SLOT(save()));

    exitAct = new QAction(QIcon(":/images/exit.png"), tr("&Exit"), this);
    exitAct->setShortcut(QKeySequence::Quit);
    connect(exitAct, SIGNAL(triggered()), this, SLOT(quit()));

    saveAsAct = new QAction(QIcon(":/images/saveas.png"), tr("&Save as..."), this);
    saveAsAct->setStatusTip(tr("Save this schedule as another file"));
    connect(saveAsAct, SIGNAL(triggered()), this, SLOT(saveAs()));

    saveAllAct = new QAction(tr("&Save all"), this);
    saveAllAct->setStatusTip(tr("Save all modified buffers"));
    connect(saveAllAct, SIGNAL(triggered()), this, SLOT(saveAll()));

    undoAct = new QAction(QIcon(":/images/undo.png"), tr("&Undo"), this);
    undoAct->setShortcuts(QKeySequence::Undo);
    connect(undoAct, SIGNAL(triggered()), this, SLOT(undo()));

    redoAct = new QAction(QIcon(":/images/redo.png"), tr("&Redo"), this);
    redoAct->setShortcut(QKeySequence::Redo);
    connect(redoAct, SIGNAL(triggered()), this, SLOT(redo()));

    aboutAct = new QAction(tr("&About"), this);
    connect(aboutAct, SIGNAL(triggered()), this, SLOT(about()));

    aboutQtAct = new QAction(tr("About &Qt"), this);
    connect(aboutQtAct, SIGNAL(triggered()), qApp, SLOT(aboutQt()));
}

void MainWindow::createMenus()
{
    fileMenu = menuBar()->addMenu(tr("&File"));
    fileMenu->addAction(newAct);
    fileMenu->addAction(openAct);
    fileMenu->addAction(saveAct);
    fileMenu->addAction(saveAsAct);
    fileMenu->addAction(saveAllAct);
    fileMenu->addSeparator();
    fileMenu->addAction(exitAct);

    editMenu = menuBar()->addMenu(tr("&Edit"));
    editMenu->addAction(undoAct);
    editMenu->addAction(redoAct);

    schedMenu = menuBar()->addMenu(tr("&Schedule"));
    progMenu = menuBar()->addMenu(tr("Program"));
    instrMenu = menuBar()->addMenu(tr("&Instructions"));

    menuBar()->addSeparator();

    helpMenu = menuBar()->addMenu(tr("&Help"));
    helpMenu->addAction(aboutAct);
    helpMenu->addAction(aboutQtAct);
}

void MainWindow::createToolBars()
{
    fileToolBar = addToolBar(tr("File"));
    fileToolBar->addAction(newAct);
    fileToolBar->addAction(openAct);
    fileToolBar->addAction(saveAct);

    editToolBar = addToolBar(tr("Edit"));
    editToolBar->addAction(undoAct);
    editToolBar->addAction(redoAct);
}

bool MainWindow::maybeSave()
{
    if (xbtveditor->anyBufModified()) {
        QMessageBox::StandardButton ret;
        ret = QMessageBox::warning(this, tr("XBTVEd"),
                                   tr("Buffers have been modified.\n"
                                      "Do you want to save changes?"),
                                   QMessageBox::Save | QMessageBox::Discard | QMessageBox::Cancel);
        if (ret == QMessageBox::Save) {
            return save();
        } else if (ret == QMessageBox::Cancel) {
            return false;
        }
    }
    return true;
}

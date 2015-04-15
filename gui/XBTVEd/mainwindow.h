#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QLabel>
#include "xbtved.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();

protected:
    void closeEvent(QCloseEvent *event);

//private slots:
//    void newFile();
//    void open();
//    bool saveBuffer();
//    bool saveAs();
//    bool saveAll();
//    void undo();
//    void redo();
//    void about();
//    void buffersModified();

private:
    Ui::MainWindow *ui;
    void createActions();
    void createMenus();
    void createToolBars();
//    bool maybeSave();

//    App::XBTVEditor *xbtveditor;
//    QLabel *templabel;

    QMenu *fileMenu;
    QMenu *editMenu;
    QMenu *schedMenu;
    QMenu *progMenu;
    QMenu *instrMenu;
    QMenu *helpMenu;
    QToolBar *fileToolBar;
    QToolBar *editToolBar;
    QToolBar *schedToolBar;
    QToolBar *progToolBar;
    QToolBar *instrToolBar;
    QAction *newAct;
/*
    QAction *openAct;
    QAction *saveAct;
    QAction *saveAsAct;
    QAction *saveAllAct;
    QAction *exitAct;
    QAction *undoAct;
    QAction *redoAct;
    QAction *aboutAct;
    QAction *aboutQtAct;
    */
};

#endif // MAINWINDOW_H

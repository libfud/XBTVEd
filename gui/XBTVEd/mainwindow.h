#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
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

private slots:
    void newFile();
    void open();
    bool save();
    bool saveAs();
    bool saveAll();
    void about();
    void bufModified();
    void anyBufModified();

private:
    Ui::MainWindow *ui;
    void createActions();
    void createMenus();
    void createToolBars();
    bool maybeSave();
    void loadFile(const QString(&fileName));
    void saveFile(const QString(&fileName));

    App::XBTVEditor *xbtveditor;

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
    QAction *openAct;
    QAction *saveAct;
    QAction *saveAsAct;
    QAction *saveAllAct;
    QAction *exitAct;
    QAction *cutAct;
    QAction *copyAct;
    QAction *pastAct;
    QAction *undoAct;
    QAction *redoAct;
    QAction *docAct;
    QAction *aboutAct;
    QAction *aboutQtAct;
};

#endif // MAINWINDOW_H

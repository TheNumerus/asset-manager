#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();

private slots:
    void on_sourceButton_clicked();
    void on_watchButton_clicked();
    void on_actionOpen_triggered();

    void on_actionSave_triggered();

    void on_actionSave_as_triggered();

    void on_addFiletypeButton_clicked();

    void on_actionAbout_triggered();

    void on_extensionTabs_tabCloseRequested(int index);

private:
    Ui::MainWindow *ui;
    bool watching;
};
#endif // MAINWINDOW_H

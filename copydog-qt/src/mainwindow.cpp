#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QtDebug>
#include <QDir>
#include "copydog.h"

MainWindow::MainWindow(QWidget *parent): QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);
}

MainWindow::~MainWindow() {
    delete ui;
}


void MainWindow::on_openButton_clicked() {
    auto filename = QFileDialog::getOpenFileName(this, tr("Open Config File"), QDir::homePath(), tr("TOML config file (*.toml)"));
    if (filename == nullptr) {
        return;
    }
    auto new_name = "Copydog <" + filename + ">";
    setWindowTitle(new_name);
    ui->listWidget->addItem(filename);
}

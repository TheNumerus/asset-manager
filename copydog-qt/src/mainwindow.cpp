#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QtDebug>
#include <QDir>
#include <QFile>
#include <QMessageBox>

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

    // check for valid format
    QFileInfo fi(filename);
    if (fi.completeSuffix() != "toml") {
        QMessageBox box;
        box.setText("Config must be a TOML file.");
        box.exec();
        return;
    }

    QFile file(filename);
    if (!file.open(QIODevice::ReadWrite | QIODevice::Text)) {
        QMessageBox box;
        box.setText("Error while opening config file.");
        box.exec();
        return;
    }

    ui->listWidget->clear();

    QTextStream in(&file);
    while (!in.atEnd()) {
        QString line = in.readLine();
        ui->listWidget->addItem(line);
    }

    auto new_name = "Copydog <" + filename + ">";
    setWindowTitle(new_name);
}

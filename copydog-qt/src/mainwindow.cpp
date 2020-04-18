#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QtDebug>
#include <QDir>
#include <QFile>
#include <QMessageBox>

#include <iostream>

#include "filetypesetting.h"
#include "copydog.h"
#include "../../toml11/toml.hpp"

void notYetImplementedBox() {
    QMessageBox box;
    box.setText("Not yet implemented.");
    box.setIcon(QMessageBox::Icon::Warning);
    box.exec();
}

MainWindow::MainWindow(QWidget *parent): QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);
    watching = false;
    ui->sourceLineEdit->setText(QDir::homePath());
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::on_sourceButton_clicked() {
    QString path = QFileDialog::getExistingDirectory(this, tr("Open Source Folder"), QDir::homePath());
    if (path == nullptr) {
        return;
    }
    ui->sourceLineEdit->setText(path);
}

void MainWindow::on_watchButton_clicked() {
    if (watching) {
        ui->logList->addItem("Watch stopped");
        ui->watchButton->setText("Watch");
        ui->watchButton->setIcon(QIcon::fromTheme("media-playback-start"));
    } else {
        ui->logList->addItem("Watch started");
        ui->watchButton->setText("Stop");
        ui->watchButton->setIcon(QIcon::fromTheme("media-playback-stop"));
    }
    watching = !watching;
}

void MainWindow::on_actionOpen_triggered() {
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

    ui->logList->clear();

    // TODO check validity of config
    toml::value toml_data = toml::parse(filename.toStdString());


    std::string source;
    try {
        source = toml::find<std::string>(toml_data, "source");
    }  catch (std::out_of_range&) {
        QMessageBox box;
        box.setText("Config file does not have source parameter.");
        box.exec();
        return;
    }

    //remove old data
    while (auto item = ui->extensionList->takeAt(0)) {
        delete item->widget();
    }

    //fill window with data

    ui->sourceLineEdit->setText(QString::fromStdString(source));

    toml::table data = toml::get<toml::table>(toml_data);
    // iterate over data
    for (std::pair<toml::key, toml::value> value: data) {
        if (value.first != "source") {
            auto filetype = new FiletypeSetting(QString::fromStdString(value.first));
            ui->extensionList->addWidget(filetype);
        }
    }

    auto new_name = "Copydog <" + fi.fileName() + ">";
    setWindowTitle(new_name);
}

void MainWindow::on_actionSave_triggered() {
    notYetImplementedBox();
}

void MainWindow::on_actionSave_as_triggered() {
    notYetImplementedBox();
}

void MainWindow::on_addFiletypeButton_clicked() {
    notYetImplementedBox();
}

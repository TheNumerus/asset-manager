#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QtDebug>
#include <QDir>
#include <QFile>
#include <QMessageBox>
#include <QCloseEvent>
#include <QSettings>

#include <iostream>

#include "filetypesetting.h"
#include "copydog.h"
#include "aboutwindow.h"

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
    // save window position
    QSettings qs;
    qs.setValue("pos", QVariant(pos()));
    qs.setValue("size", QVariant(size()));
    qs.sync();

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
        qDebug() << QString::fromStdString(generate_toml());
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
    ui->extensionTabs->clear();

    //fill window with data
    ui->sourceLineEdit->setText(QString::fromStdString(source));

    toml::table data = toml::get<toml::table>(toml_data);
    // iterate over data
    for (std::pair<toml::key, toml::value> value: data) {
        if (value.first != "source") {
            auto extension = QString::fromStdString(value.first);
            auto target = toml::find<std::string>(value.second, "target");
            auto filetype = new FiletypeSetting(ui->extensionTabs, extension);
            filetype->set_target_path(QString::fromStdString(target));
            ui->extensionTabs->addTab(filetype, extension);
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
    auto filetype = new FiletypeSetting(ui->extensionTabs, "ext");
    auto index = ui->extensionTabs->addTab(filetype, "ext");
    ui->extensionTabs->setCurrentIndex(index);
}

void MainWindow::on_actionAbout_triggered() {
    AboutWindow *aw = new AboutWindow(this);
    aw->show();
}

void MainWindow::on_extensionTabs_tabCloseRequested(int index) {
    ui->extensionTabs->removeTab(index);
}

std::string MainWindow::generate_toml() {
    // add source line
    toml::value data{{"source", ui->sourceLineEdit->text().toStdString()}};

    for (FiletypeSetting* fs: ui->extensionTabs->findChildren<FiletypeSetting*>()) {
        toml::value v{{"target",
            fs->get_target_path().toStdString()
        }};

        data.as_table().insert(std::pair<std::string, toml::value>(fs->get_extension().toStdString(), v));
    }

    std::string s = toml::format(data);

    copydog::print_input(s.data());
    return s;
}

void MainWindow::closeEvent(QCloseEvent *event) {
    // TODO check for unsaved changes
    if (watching) {
        QMessageBox box;
        box.setText("Are you sure you want to close? Watching is still in progress.");
        box.setIcon(QMessageBox::Icon::Warning);
        box.setStandardButtons(QMessageBox::Ok | QMessageBox::Cancel);
        box.setDefaultButton(QMessageBox::Cancel);
        auto result = box.exec();
        if (result == QMessageBox::Cancel) {
            event->ignore();
        }
    }
}

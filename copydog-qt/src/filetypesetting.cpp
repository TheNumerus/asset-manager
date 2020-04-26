#include "filetypesetting.h"
#include "ui_filetypesetting.h"
#include "ignorefolderitem.h"

#include <QTabWidget>
#include <QDir>
#include <QFileDialog>

FiletypeSetting::FiletypeSetting(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::FiletypeSetting)
{
    ui->setupUi(this);
}

FiletypeSetting::FiletypeSetting(QWidget *parent, QString extension) : QWidget(parent), ui(new Ui::FiletypeSetting) {
    ui->setupUi(this);
    ui->extensionText->setText(extension);
}

QString FiletypeSetting::get_extension() const {
    return ui->extensionText->text();
}

void FiletypeSetting::set_target_path(const QString &path) {
    ui->pathText->setText(path);
}

QString FiletypeSetting::get_target_path() const {
    return ui->pathText->text();
}

void FiletypeSetting::add_ignore_folder(const QString & folder) {
    auto item = new IgnoreFolderItem(ui->ignoredFoldersWidget, folder);
    ui->ignoredFoldersWidget->layout()->addWidget(item);
}

FiletypeSetting::~FiletypeSetting() {
    delete ui;
}

void FiletypeSetting::on_extensionText_textEdited(const QString &arg1){
    // QTabWidget contains QStackedWidget
    auto tab = static_cast<QTabWidget*>(parentWidget()->parentWidget());
    tab->setTabText(tab->currentIndex(), arg1);
}

void FiletypeSetting::on_pathButton_clicked() {
    QString path;

    if (ui->pathText->text().length() == 0) {
        path = QFileDialog::getExistingDirectory(this, tr("Open Target Folder"), QDir::homePath());
    } else {
        path = QFileDialog::getExistingDirectory(this, tr("Open Target Folder"), ui->pathText->text());
    }

    if (path == nullptr) {
        return;
    }
    ui->pathText->setText(path);
}

#include "ignorefolderitem.h"
#include "ui_ignorefolderitem.h"

IgnoreFolderItem::IgnoreFolderItem(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::IgnoreFolderItem)
{
    ui->setupUi(this);
}

IgnoreFolderItem::IgnoreFolderItem(QWidget *parent, const QString &path):
    QWidget(parent),
    ui(new Ui::IgnoreFolderItem)
{
    ui->setupUi(this);
    ui->lineEdit->setText(path);
}

IgnoreFolderItem::~IgnoreFolderItem()
{
    delete ui;
}

#include "filetypesetting.h"
#include "ui_filetypesetting.h"

FiletypeSetting::FiletypeSetting(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::FiletypeSetting)
{
    ui->setupUi(this);
}

FiletypeSetting::FiletypeSetting(QString extension) :
    ui(new Ui::FiletypeSetting)
{
    ui->setupUi(this);
    ui->lineEdit->setText(extension);
}

FiletypeSetting::~FiletypeSetting()
{
    delete ui;
}

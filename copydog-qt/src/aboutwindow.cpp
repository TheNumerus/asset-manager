#include "aboutwindow.h"
#include "ui_aboutwindow.h"
#include "versions.h"
#include <QtSvg>
#include <QGraphicsSvgItem>

AboutWindow::AboutWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::AboutWindow)
{
    ui->setupUi(this);
    ui->appversionlabel->setText("Version " + app_version);
    ui->icon->load(QString(":/logo.svg"));
}

AboutWindow::~AboutWindow()
{
    delete ui;
}

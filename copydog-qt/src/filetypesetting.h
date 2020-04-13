#ifndef FILETYPESETTING_H
#define FILETYPESETTING_H

#include <QWidget>

namespace Ui {
class FiletypeSetting;
}

class FiletypeSetting : public QWidget
{
    Q_OBJECT

public:
    explicit FiletypeSetting(QWidget *parent = nullptr);
    FiletypeSetting(QString extension);
    ~FiletypeSetting();

private:
    Ui::FiletypeSetting *ui;
};

#endif // FILETYPESETTING_H

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
    FiletypeSetting(QWidget *parent, QString extension);

    QString get_extension() const;

    void set_target_path(const QString &);
    QString get_target_path() const;

    void add_ignore_folder(const QString &);

    ~FiletypeSetting();

private slots:
    void on_extensionText_textEdited(const QString &arg1);
    void on_pathButton_clicked();

private:
    Ui::FiletypeSetting *ui;
};

#endif // FILETYPESETTING_H

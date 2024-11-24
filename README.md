# messy-buster

`messy-buster` は、指定されたディレクトリ内のファイルを拡張子と日付ごとに整理するツールです。  
ファイルを自動的にフォルダに移動し、フォルダツリーを作成します。

## インストール

まず、Rustがインストールされていることを確認してください。Rustがインストールされていない場合は、[公式サイト](https://www.rust-lang.org/)からインストールしてください。

次に、リポジトリをクローンし、依存関係をインストールします。

```sh
git clone https://github.com/yourusername/messy-buster.git
cd messy-buster
cargo build --release

<build-path>/messy-buster -d <path_your_directory> --save
```

## 使い方

### 基本的な使い方

指定されたディレクトリ内のファイルを整理します。

```sh
messy-buster -d <directory>
```

### オプション

- `-d, --directory <DIRECTORY>`: ベースディレクトリを設定します。
- `--save`: ディレクトリを config.toml に保存します。保存のみを行い、ファイルの移動は行いません。
- `--dry-run`: ファイルの移動をシミュレートします。実際にはファイルを移動せず、フォルダツリーを表示します。
- `--show-config`: 現在の config.toml の base_dir を表示します。

#### 例

ディレクトリを指定してファイルを整理
```sh
messy-buster -d /path/to/your/directory
```

ディレクトリを config.toml に保存

```sh
messy-buster -d /path/to/your/directory --save
```

ファイルの移動をシミュレート

```sh
messy-buster -d /path/to/your/directory --dry-run
```

現在の base_dir を表示

```sh
messy-buster --show-config
```


## 設定ファイル

`config.toml` ファイルを使用して、ベースディレクトリを設定できます。

```toml
base_dir = "/path/to/your/directory"
```

## ライセンス

このプロジェクトは [MIT ライセンス](LICENSE) のもとで公開されています。

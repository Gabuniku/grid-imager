# Grid-Imager

複数の画像をグリッド状に配置するツールです．


<img width="500" height="500" alt="test" src="https://github.com/user-attachments/assets/81fae207-4f36-4814-9d1c-347b42743177" />


こんなのが作れます．


---

## 使い方

`grid-imager-cli [OPTION] [File]...`

例 : `grid-imager-cli -o grid_image.png -s 200 -c 2 a.png b.png c.png d.png --stroke 5`

---

## オプション

| 引数                    | 名前                  | 概要              | 初期値              |
|-----------------------|---------------------|-----------------|------------------|
| `-d` ,  `--directory` | `<ImagesDirectory>` | 読み込む画像のあるディレクトリ |                  |
| `-o` ,  `--output`    | `<output>`          | 出力ファイル名         | `grid_image.png` |
| `-s` ,  `--size`      | `<size>`            | 1コマの大きさ [px]    | `200`            |
| `-c` ,  `--columns`   | `<columns>`         | 列数              |                  |
| `--stroke`            | `<stroke>`          | グリッド線の幅 [px]    | `0`              |
| `-r` ,  `--red`       | `<red>`             | グリッド線の赤色の値      | `0`              |
| `-g` ,  `--green`     | `<green>`           | グリッド線の緑色の値      | `0`              |
| `-b` ,  `--blue`      | `<blue>`            | グリッド線の青色の値      | `0`              |
| `-a` ,  `--alpha`     | `<alpha>`           | グリッド線のアルファ値     | `255`            |
| `-h` ,  `--help`      |                     | ヘルプを表示          |                  |
| `-V` ,  `--version`   |                     | バージョンを表示        |                  |

## インストール
[リリースページ](https://github.com/Gabuniku/grid-imager/releases)からビルド済みのバイナリをダウンロード.


もしくはcargo環境ならば
`cargo install --path .`
でインストールできます．

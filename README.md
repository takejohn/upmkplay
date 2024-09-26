# upmkplay - Upload Misskey Play
Misskey PlayのスクリプトをアップロードするCLIアプリ

# Usage
## CLI
```sh
upmkplay [OPTIONS] [script]
```

### Arguments
* `[script]` - AiScript(.is)のファイル名

### Options
* `-c`, `-config <filename>` 設定ファイル名を指定(デフォルト: `upmkplay.config.json`)
* `-w`, `--watch` ウォッチモードを有効化(AiScriptファイルが更新されると自動でアップロード)

## Config
`--config`オプションで指定されたJSONの設定ファイル(デフォルトでは`upmkplay.config.json`)を読み込みます。

各キーで以下の値を指定します。
* `"url"` **(必須)** - PlayページのURL
* `"token"` **(必須)** - APIのアクセストークン
* `"title"` - Playのタイトル
* `"summary"` - Playの説明
* `"visibility"` - Playの公開範囲。`"public"`(パブリック)または`"private"`(非公開)

例:
```json
{
    "url": "https://example.com/play/abcde12345",
    "token": "12345678abcdefghABCDEFGH12345678",
    "title": "Test Play",
    "summary": "Hello, world!",
    "visibility": "public"
}
```

## アクセストークンの取得方法
1. PlayのホストとなるMisskeyインスタンスにアクセスします。
1. 「設定」を開き、「API」から「アクセストークンの発行」をクリックします。
1. 適当な名前を入力し、「権限」一覧から「Playを操作する」をオンにして、チェックマークをクリックします。
1. 「確認コード」に表示された英数字列がアクセストークンです。

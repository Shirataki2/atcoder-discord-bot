# AtCoder Discord Bot (AtCorder)

AtCoderのAC履歴から自分がACしたものをDiscordのチャンネルに送信します．

## Usage

### 1. Botを導入する

https://discord.com/api/oauth2/authorize?client_id=801783771526856704&permissions=126016&scope=bot

から導入ができます

### 2. 通知を受け取るチャンネルで`^start` コマンドを実行

※濫用防止のため`メッセージ管理`権限のあるユーザーのみが実行可能です


### 3. `^register <your_atcoder_id>` で自分のAtCoder IDとDiscordのユーザーデータを紐づける

この地点ではACをとっても送信されません．

### 4. `^subscribe`でAC情報を受け取る

内部的にAtCoder Problems のAPIを利用しています．APIの更新頻度によって変わりますが概ね2-5分程度遅れて通知が送信されます．


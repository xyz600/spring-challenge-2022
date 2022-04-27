# spring-challenge-2022

## ルール（日本語訳）

### 目的

モンスターの攻撃から自分の基地を守り、相手より長生きしよう。

### ルール

両プレイヤーは3体のヒーローからなるチームを操作します。
両チームはマップの反対側の隅にある拠点付近からスタートします。
ゲーム中、マップの端には定期的にモンスターが出現します。
モンスターが自分の拠点に到達すると、ダメージを受けます。
自分の拠点がダメージを受けすぎると負けとなります。

ありがたいことに、ヒーローはモンスターを退治することができます。

### マップ

ゲームは、座標X=0, Y=0を左上のピクセル、X=17630, Y=9000を右下のピクセルとする長方形のマップ上で行われます。
各拠点は破壊される前に最大3ポイントのダメージを受けることができます。

### ヒーロー

毎ターン、あなたは各ヒーローにコマンドを与えなければなりません。以下のコマンドのいずれかを実行することができます。

* WAIT（待機）：ヒーローはその場にとどまります。
* MOVE（移動）：移動の後にマップの座標を入力すると、ヒーローはその地点に向かって最大800ユニット前進します。

ヒーローの移動フェイズ後、800ユニット以内にいるモンスターは2ポイントのダメージを受けます。

### モンスター

すべてのモンスターは、一定の体力を持って登場します。ターン終了時に、モンスターの体力が0以下になった場合、そのモンスターはゲームから除外されます。
モンスターはランダムに出現し、移動方向もランダムです。
モンスターは常に1ターンに400ユニットの速度で一直線に進みます。
ターン終了時に拠点から5000ユニット以内にモンスターがいた場合、その拠点をターゲットにします。
拠点を狙った場合、モンスターはその拠点に向かって直進し、マップから離れることができなくなります。
ターン終了時に拠点から300ユニット以内にモンスターがいた場合、そのターンに殺されていない限り、そのモンスターは消滅し、拠点に1ポイントのダメージを与える。

### 勝敗

* 勝利条件
  * 相手の基礎体力が0になった。
  * 220ターン経過した時点で、あなたの基礎体力が相手より多いこと。
* 敗北条件
  * あなたのベースの体力が0になった。
  * あなたのプログラムが有効なコマンドを時間内に提供しなかった。

### 🐞デバッグのヒント

* エンティティにカーソルを合わせると、そのエンティティに関する詳細な情報が表示されます。
* コマンドの後にテキストを追加すると、そのテキストがヒーローの上に表示されます。
* ビューアの歯車のアイコンを押すと、追加の表示オプションにアクセスできます。
* キーボードでアクションを操作する：スペースで再生/一時停止、矢印で1フレームずつ進む

## 参加記

(1日目)

### wood2 league

* 脅威になる敵のうち、一番近くにいるキャラを殴り続ける
* 一度ターゲットにしたキャラは、消えるまで覚え続ける

### 覚えておくべきこと

* 一見直線状に来てないように見える monster も threat 判定喰らうことがある
  * ルールでいうところの「ターン終了時に拠点から5000ユニット以内にモンスターがいた場合、その拠点をターゲットにします」が働いているせいだとわかった
  * 割と threat_state が便利だとわかりつつある
* 何か時間経過と共に敵の体力が増加し続けているように見える
  * 一人で殴り続けても無理があるから、複数人で協調して殴らないといけないように見える？
* 敵キャラクターが何ターンで自軍につくかは正確に計算したいところ
  * 明らかに間に合わないなら、技を使うなり応援を頼むなりできそう

### wood1 league

* 序盤で敵を押し付けられるよりも、終盤で押し付けられる方がとても辛い
* 序盤は自分の陣地近辺でマナを生成することに徹して、後半で2人がかりで蜘蛛を wind し続ける戦略を試してみたい

(2日目)

### 強そうな戦略

* 序盤はマナをたくさん貯める
  * 何をするにもマナは必要
  * 適当なターン数で分岐
* 割り当ては、距離の総和が一番小さいのを選ぶ
* (防御) monster の到着までに間に合わなければ `Wind` を使う
  * 到着までに狩れるかどうかの判定を入れる
    * 毎ターン距離400詰める
    * 300 unit圏内に入れば攻撃
    * 相手が自陣 300 unit 圏内に入ったターンに倒されない
* (攻撃) 相手の陣地に入らない monster を相手陣地に入れる wind をする
  * 角度とタイミングが重要
    * 攻撃2人 : 水平 or 垂直から interleave 周期で投げ続ける
    * 防御1人 : 細かいことはせず、切る＋ギリギリになったら wind
      * 防御魔法かけられたら防御手薄だと厳しそうだけど、同じことはこっちの攻撃にも言えるので、速く攻撃ができればよい
  * 1回 wind したらもう二度としない
    * あんまり付きまとっても、HPが削られてしまうので

### wood1 league 最終戦略

* 序盤80ターンはマナを貯め続ける
* 攻撃2人、防御1人
* 補助情報
  * 陣地が左右で異なることで座標の取り扱いが異なるのが面倒だったので、常に左にいるように点対称に変換してから渡すことにした

### bronze

* 全ルール解法
  * control と defense

### ルールの確認

* スペルは、hero id が小さい順に適用されていく
* 複数の control を同時に受けたら、平均値に落ちる
* 同時に wind を受けたら、全部効果が適用される
* shield は、shield の重ね掛けも無効化する
* shield がかかったキャラに spell を打っても、マナを消費してしまう
* ライフがタイの場合は、保持していた最大マナの過多で勝敗が決まる
* コマンドの順番が決まっている
  * `Control` -> `Shield` -> `Move hero` -> 攻撃＋マナ回復判定 -> `Wind` -> `Move monster` -> `shield countdown` -> monster appear / disappear
  * hero と monster の動きは同じじゃなかったのか…要修正
* 敵のHP増加速度
  * 初期 HP は 10
  * 5ターンに1回モンスターが spawn されて、spawn の毎に HP が 0.5 ずつ上昇していく
* shield が張られていない状態で control されると、そのターン SHIELD も打てなくなる
* 相手陣地内にいる敵 hero は、自分には見えない
  * visualizer では見えるので、混乱しがち

### 強そうな戦略

* 敵キャラが複数いた時、一番マナが溜まる位置を見つけてそこに動く
* 自陣で wind した時、相手の陣地の端を狙って飛ばす
* 相手ディフェンスを吹き飛ばしたり control したりして、攻撃を阻害する
* 相手陣地に 侵入することがわかっている monster に defense をかける
  * 端にいるほど良いけど、どうなんだろう
* wind する時、相手の淵ぎりぎりを狙う
  * 同時に wind されることを防ぎたい
* defense 側は、相手が wind してくると思ったら自分も wind をしないと負け確になる
  * さっさと wind をするという選択もありかもしれない
* 相手の防御を control で外に出す
* attack は1人パス、1人シュート決めるのがよさそう
* CONTROL 難しいと思ってたけど、 action の優先順位が一番高いのが強い原因っぽく見える
  * 極端な話、SHIELD 張らなくてもずっと CONTROL できるならもうそれでいいじゃん

### バグ修正

* defense の際、wind する条件などを一番近い target に限定していたが、全員に対して確認しないといけない

### 対戦環境の構築

* コドゲのサーバーが重すぎるので、現状の AI に対する勝率が知りたいと思っても色々難しい
* という訳で、時間もあるので対戦環境を作ることにする
  * コドゲが用意してくれた対戦環境を動かしてみようとするも、上手くいかず…
  * ちょっと必要になるまであきらめて、AIの改善自体をやっていく
  * MCTS とかどうやるのかあんまり想像ついてないんだよな
* ちょっと自分で作ってみたい気持ちに駆られてきたので、自分で作ってみる
  * ゲームのルールのついでに、ゲームのルールに基づいた適当な判定ルーチンも実装したいので

### 次に作る戦略まとめ

* 序盤：マナをたくさん貯める
  * 情報量が高くなるように分散したい
  * 複数キャラを同時に叩くことで、効率的にマナを集められる位置取りを探す
    * 直近1手の行動で、最もマナが大きくなる位置を探索
      * monster を2重ループして、その中点に移動する
* 終盤：マナが K 溜まったら移行する
  * 1人ディフェンス
    * 自陣の前線ギリギリで、以下をやる
      * 対策なしだとゴールされてしまうが対応方法が存在する monster のうち、猶予ターンが最も少ないものに以下をする
        * SHIELD が張られていない場合、WIND
          * 動きの量を最小限に
        * SHIELD が張られている敵には、ゴールに近い順に攻撃対応
          * 前線に近い位置ギリギリで殴り続ける
  * 1人ミッドフィルター
    * 相手の陣地の直前に居座って、以下をやる（優先度順）
      * monster がいない場合は、とにかく動いて monster を探す
      * monster を WIND で陣地の端へ飛ばす
  * 1人フォワード
    * 相手の陣地内に入って、以下をやる（優先度順）
      * 勝ちが確定する行動をとる
        * SHIELD を張る
          * base から距離 300 + 800T 以内にいる、HP が 2NT より多い monster に対して
            * ディフェンスの数（= 相手陣地内にいる相手の数）を N とする
            * 1ターンに減らせる monster の HP は最大4
            * 相手が即時攻撃を仕掛けられる前提でも間に合わない
        * SHIELD が張られた monster の保護命令
          * CONTROL を無限連打が成功したと仮定
          * その場で wind が成功したと仮定
      * base までの距離が 4300 以内 だったら、相手陣地内にいる monster に SHIELD をかける
        * 勝ち確とまでは言わんけど、相手を十分引き付けられるし、相手がスルーした結果勝ち確になることもある
      * 相手陣地 monster を飛ばさず、相手を WIND できたら行う
  * 共通
    * マナがない場合は、マナを貯める
    * 相手が hero に SHIELD を張る戦略を採用しているかを覚えておく
    * 原理的に相手が追い付けるか否かの判定
      * 相手陣地内にいる各 hero に対して、最善で追いついて殴り続けた時にどうなるか


### 相性の悪さ

* unclassify
  * TOBIASA
  * Linksawaking
* 三位一体＋完全防御
  * xdcomrade
* 完全防御 => 単身攻撃型
  * PiterYeh
* wind シュート型
  * Samsonov
  * Ch3oul
  * brian__fr
* wind 防御型
  * awnion
* 手前防御型
  * TongoHiti
* 単身速攻型
  * Emerg
  * __NAGATO__
  * XilasZ
* 戦略移行型(中盤を mid 3人、終盤を attacker 3人)
  * Tatasterix
  * Blodis
  * Littleyounes
  * Neumann
  * Eddgja
* 三位一体型
  * mchl12
* 完全防御型
  * AbsorbantLight
  * Althea59
  * Twibby
  * Haitaka
  * stdas
  * Boulderdash
  * seb
  * jaberkro
* 対策済
  * Tatasterix
  * 完全防御型
  * mchl12


### 相手の戦略を分類する

* 背景
  * 特定の相手に馬鹿みたいに勝てる戦略を作ったら、他の大多数の人に負ける戦略になってしまった
  * 戦略設計としてどういう戦略にもある程度渡り合えるものでないと厳しいので、ありえる戦略について考えてみる
* 分類
  * 完全守備型
    * 自陣にキャラが全く現れない
  * 速攻型
    * 開始早々に相手 hero が自陣にいる

### ゲームの simulator の使いどころ

* 以下のような可能性をジャッジしたい
  * 1匹必ず虫を shoot できるか
  * 相手を確実に倒せるか否か
* 相手の手を適当に決め打った上で、探索などを通して詰みがあるかを確認できたらいい
* full simulator と partial simulator を区別したい
  * 実際の探索では内部状態が完全にはわからないので、取れる情報が限定的
  * パラメータ調整など評価に使える環境も込みで

### ゲームの探索の使いどころ

* 色んな動きを優先度つけてるけど、コスト関数をベースにして探索とかできたら嬉しい
* ただ、今回の問題では原理的に打てる手が無数にあるので、適切に絞るのが難しいと思う
  * 有効に思える動きをいくつか挙げておいて、その中から探索する
* 場面を絞ってもいいと思う
  * 例えば、自分の攻撃パターンを探索することに限定する等
* コスト関数として有効に見えるもの
  * 相手を殺したか否か
  * 相手の残り HP
  * monster を 到着させるのに必要なターン数
    * 相手の邪魔が入らない前提？
  * 残りマナ数
  * 敵 hero が自 hero を妨害するために必要な最短ターン数
* 攻撃に有効な動作
  * 特定の虫に近づく(半径)
    * HP を奪う
    * HP を温存
  * 敵 hero を wind で遠ざける
  * 敵 hero を control で遠ざける
* 結局のところ、確実に虫が入れられるかの判定器が欲しいということになりそう
  * 相手の行動によらず詰んでいることを判定するのは結構厳しい？
    * 相手 hero が自分の hero の妨害抜きにして阻止できるか

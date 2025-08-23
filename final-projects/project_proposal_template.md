# Project Proposal: 
GhostCouncil

## 1. Overview <!-- プロジェクトの概要を数行で記載 -->
提案に対して登録されたメンバーからどのくらい承認受けたか、承認した人は秘匿にして集計できる

## 2. Objectives <!-- プロジェクトの背景・目的・スコープ等を記載 -->
DAO における投票・承認では「誰が承認したか」が明示されるのが通常だが、これはプライバシーや社会的リスクを伴う。
一方で、ガバナンスの透明性には「参加人数」「しきい値到達」が不可欠。
そこで Ghost Council は 「参加メンバーは公開」「誰が承認したかは秘匿」「何人承認したかは透明」 を両立させる。

## 3. Deliverables <!-- プロジェクトにおける成果物の想定を記載 -->
アプリに簡単に取り込める開発キットとデモ用のサイバーでかっこいいuiのweb app

## 4. Team <!-- プロジェクトメンバーとそれぞれの役割(e.g.,どの部分を担当するか)を記載 -->

| Member | Role |
|-------:|:-----|
| kaga       |  zk, ui design    |

## 5. Design & Architecture <!-- 全体設計や細部のアーキテクチャーを具体的に記載(成果物が実装の場合のみ) -->
スマートコントラクト
	•	GhostGovernor.sol: 提案・承認・実行を管理
	•	Verifier.sol: ZK証明の検証モジュール
	•	ZK 回路
	•	Circom 回路 daoApprove.circom（Merkle inclusion + nullifier 生成）
	•	Groth16 証明のセットアップファイル（zkey, vkey）
	•	フロントエンドアプリ
	•	提案一覧・承認UI（サイバー風デザイン）
	•	リアルタイム承認人数表示（イベント購読）
	•	「ACCESS GRANTED」等のサイバーパンク演出
	•	サブグラフ（The Graph）
	•	承認イベントのインデックス化、履歴検索

## 6. Reference Materials <!-- 参考にした資料・リンク等を記載 -->
https://arxiv.org/abs/2501.05626

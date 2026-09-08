/**
 * @fileoverview ヘルプ＆アプリ情報モーダルコンポーネントモジュール。
 *
 * アプリケーションの概要説明、バージョン表記、および各種機能・ショートカットキーの
 * 操作ガイドを直感的に閲覧できるモーダルダイアログを提供します。
 */

import React from 'react';
import { motion } from 'framer-motion';
import { 
  X, 
  HelpCircle, 
  Clock, 
  Calendar as CalendarIcon, 
  Pin, 
  Sun, 
  Move, 
  Keyboard, 
  Info,
  Sparkles
} from 'lucide-react';

/**
 * ヘルプモーダルのプロパティ型定義。
 * @typedef {Object} HelpModalProps
 * @property {string} version - アプリケーションのバージョン文字列 (例: "1.4.0")
 * @property {function(): void} onClose - モーダルダイアログを閉じるコールバック関数
 */

/**
 * 操作ガイドカードのデータ定義
 */
const GUIDE_ITEMS = [
  {
    icon: Clock,
    title: '時計機能',
    description: 'デジタル表示（力強いImpact系フォント）とアナログ表示（滑らかなスイープ運針）を切り替え可能。12H/24H表記や秒表示のON/OFFもワンクリックで変更できます。'
  },
  {
    icon: CalendarIcon,
    title: 'カレンダー機能',
    description: '日本の祝日（振替休日・国民の休日）に対応。月の長さに影響されない安定した6週間レイアウトを採用し、年間カレンダー表示や祝日カスタマイズも備えています。'
  },
  {
    icon: Pin,
    title: '最前面ピン留め',
    description: 'タイトルバー右上のピンアイコンで、他のウィンドウに隠れず常に最前面に表示（Always-On-Top）できます。作業中も常に時刻を確認可能です。'
  },
  {
    icon: Sun,
    title: '透過 & ダークモード',
    description: '太陽アイコンで背景透過モード（グラスモーフィズム）へ切り替え。月アイコンでライト/ダークテーマを瞬時に切り替えられ、壁紙に美しく調和します。'
  },
  {
    icon: Move,
    title: '自由なウィンドウ配置',
    description: 'ウィンドウのほぼ全域をドラッグしてデスクトップの好みの位置へ配置できます。マルチモニター環境でも前回の終了位置を記憶して自動復元します。'
  },
  {
    icon: Keyboard,
    title: 'ショートカット & トレイ',
    description: '「Esc」キーでアプリを素早く終了できます。タスクバーのシステムトレイアイコンを右クリックして最前面固定や位置リセットを行うことも可能です。'
  }
];

/**
 * ヘルプ＆アプリ情報モーダルコンポーネント。
 *
 * ## 特徴
 * - アプリのバージョン番号およびコンセプトを分かりやすく提示。
 * - アイコン付きのカードレイアウトで各機能を整理して解説。
 * - 外側クリック、閉じるボタン（✕）、またはキー操作でスムーズに終了可能。
 *
 * @param {HelpModalProps} props - コンポーネントのプロパティ
 * @returns {JSX.Element} HelpModal コンポーネント
 */
export default function HelpModal({ version, onClose }) {
  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.95 }}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-md select-none"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-white dark:bg-slate-900 w-full max-w-3xl max-h-[85vh] rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/20 dark:border-slate-800">
        {/* ヘッダーエリア */}
        <div className="p-6 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between flex-shrink-0 bg-slate-50/50 dark:bg-slate-950/20">
          <div className="flex items-center gap-3">
            <div className="p-2.5 bg-blue-500/10 text-blue-500 rounded-2xl">
              <HelpCircle size={24} />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h2 className="text-2xl font-black text-slate-900 dark:text-white tracking-tight">
                  Clondar Pro
                </h2>
                <span className="px-2.5 py-0.5 bg-blue-50 dark:bg-blue-950/50 text-blue-600 dark:text-blue-400 font-mono text-xs font-bold rounded-full border border-blue-500/20">
                  {version ? `v${version}` : 'v1.0.0'}
                </span>
              </div>
              <p className="text-xs text-slate-400 dark:text-slate-500 mt-0.5">
                デスクトップ常駐型 ミニマル時計 ＆ カレンダーウィジェット
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors text-slate-400 hover:text-slate-600 dark:hover:text-slate-200"
            title="閉じる"
            aria-label="閉じる"
          >
            <X size={20} />
          </button>
        </div>

        {/* コンテンツエリア */}
        <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
          {/* アプリのコンセプト説明 */}
          <div className="p-4 rounded-2xl bg-gradient-to-r from-blue-500/5 via-indigo-500/5 to-purple-500/5 border border-blue-500/10 flex items-start gap-3.5">
            <div className="p-2 bg-blue-500/10 text-blue-500 rounded-xl flex-shrink-0 mt-0.5">
              <Sparkles size={18} />
            </div>
            <div className="text-xs leading-relaxed text-slate-600 dark:text-slate-300">
              <span className="font-bold text-slate-800 dark:text-slate-100">Clondar</span> は、Windows デスクトップに美しく溶け込む、透過・枠なし・影なしの極限までミニマルなデザインを追求したウィジェット型時計＆カレンダーアプリです。軽量なネイティブエンジン（Tauri v2）により低負荷で軽快に動作します。
            </div>
          </div>

          {/* 機能一覧グリッド */}
          <div>
            <h3 className="text-xs font-black text-slate-400 dark:text-slate-500 uppercase tracking-widest mb-3 flex items-center gap-1.5">
              <Info size={14} /> 主な機能と使い方
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3.5">
              {GUIDE_ITEMS.map(({ icon: ItemIcon, title, description }) => (
                <div
                  key={title}
                  className="p-4 rounded-2xl bg-slate-50 dark:bg-slate-800/50 border border-slate-100 dark:border-slate-800 flex flex-col gap-2 hover:border-slate-200 dark:hover:border-slate-700 transition-colors"
                >
                  <div className="flex items-center gap-2 text-blue-600 dark:text-blue-400">
                    <ItemIcon size={16} />
                    <span className="text-xs font-bold text-slate-800 dark:text-slate-200">
                      {title}
                    </span>
                  </div>
                  <p className="text-[11px] leading-relaxed text-slate-500 dark:text-slate-400">
                    {description}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* フッター */}
        <div className="px-6 py-4 border-t border-slate-100 dark:border-slate-800 flex items-center justify-between bg-slate-50/50 dark:bg-slate-950/20 text-[11px] text-slate-400 dark:text-slate-500 flex-shrink-0">
          <span>Clondar Pro © 2026</span>
          <button
            onClick={onClose}
            className="px-4 py-1.5 bg-blue-500 hover:bg-blue-600 text-white text-xs font-bold rounded-xl shadow-sm transition-all"
          >
            閉じる
          </button>
        </div>
      </div>
    </motion.div>
  );
}

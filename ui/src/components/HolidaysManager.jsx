import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { X, Calendar as CalendarIcon, AlertCircle, FileText, CheckCircle2, Trash2, Plus, Save } from 'lucide-react';
import { isTauri } from '../utils/tauri';
import { fallbackConfig } from '../utils/holidays';

export default function HolidaysManager({ onClose, onSaved }) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [stats, setStats] = useState([]);
  const [diffParts, setDiffParts] = useState([]);
  const [activeTab, setActiveTab] = useState('edit'); // デフォルトを 'edit' に
  const [holidaysConfig, setHolidaysConfig] = useState(null);
  const [rawJsonText, setRawJsonText] = useState('');
  const [saving, setSaving] = useState(false);

  // 追加用の状態
  const [newFixedDate, setNewFixedDate] = useState('');
  const [newFixedName, setNewFixedName] = useState('');

  const keywords = ['の日', '天皇誕生日', '振替休日', '国民の休日'];

  const loadData = async () => {
    try {
      setLoading(true);
      let currentJson = '';
      
      if (isTauri()) {
        const { invoke } = await import('@tauri-apps/api/core');
        currentJson = await invoke('load_holidays_json');
      } else {
        // Webブラウザ環境用のフォールバック読み込み
        try {
          const response = await fetch('/config/holidays.json');
          if (response.ok) {
            currentJson = await response.text();
          } else {
            currentJson = JSON.stringify(fallbackConfig, null, 2);
          }
        } catch (e) {
          currentJson = JSON.stringify(fallbackConfig, null, 2);
        }
      }

      setRawJsonText(currentJson);
      const parsedConfig = JSON.parse(currentJson);
      setHolidaysConfig(parsedConfig);

      const fallbackJson = JSON.stringify(fallbackConfig, null, 2);

      // 2. 統計の取得
      const calculatedStats = [];
      for (const word of keywords) {
        let count = 0;
        if (isTauri()) {
          const { invoke } = await import('@tauri-apps/api/core');
          count = await invoke('get_word_count', { text: currentJson, word });
        } else {
          const regex = new RegExp(word, 'gi');
          count = (currentJson.match(regex) || []).length;
        }
        calculatedStats.push({ word, count });
      }
      setStats(calculatedStats);

      // 3. 差分の取得
      if (isTauri()) {
        const { invoke } = await import('@tauri-apps/api/core');
        const diff = await invoke('get_holidays_diff', {
          oldText: fallbackJson,
          newText: currentJson,
        });
        setDiffParts(diff);
      } else {
        const lines = currentJson.split('\n');
        setDiffParts(lines.map(line => ({ diff_type: 'Unchanged', value: line })));
      }

      setError(null);
    } catch (err) {
      console.error('HolidaysManager data load error:', err);
      setError(err.message || 'データの取得中にエラーが発生しました');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  // 固定祝日の追加
  const handleAddFixedHoliday = (e) => {
    e.preventDefault();
    if (!newFixedDate || !newFixedName) return;

    if (!/^\d{2}-\d{2}$/.test(newFixedDate)) {
      alert("日付は MM-DD 形式 (例: 07-07) で入力してください。");
      return;
    }

    setHolidaysConfig(prev => {
      const nextFixed = { ...prev.fixed, [newFixedDate]: newFixedName };
      return { ...prev, fixed: nextFixed };
    });
    setNewFixedDate('');
    setNewFixedName('');
  };

  // 固定祝日の削除
  const handleRemoveFixedHoliday = (dateKey) => {
    setHolidaysConfig(prev => {
      const nextFixed = { ...prev.fixed };
      delete nextFixed[dateKey];
      return { ...prev, fixed: nextFixed };
    });
  };

  // データの保存
  const handleSave = async () => {
    try {
      setSaving(true);
      const jsonContent = JSON.stringify(holidaysConfig, null, 2);

      if (isTauri()) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('save_holidays_json', { jsonContent });
      } else {
        console.log("Mock Save (Browser environment):", jsonContent);
        localStorage.setItem('mock_holidays_json', jsonContent);
      }

      // キャッシュのリロード
      const { reloadHolidays } = await import('../utils/holidays');
      await reloadHolidays();

      if (onSaved) onSaved();
      onClose();
    } catch (err) {
      console.error("Failed to save holidays:", err);
      alert("保存に失敗しました: " + (err.message || err));
    } finally {
      setSaving(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.95 }}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-md"
    >
      <div className="bg-white dark:bg-slate-900 w-full max-w-5xl h-[480px] rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/20 dark:border-slate-800">
        {/* ヘッダー */}
        <div className="p-6 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-500/10 text-blue-500 rounded-xl">
              <CalendarIcon size={22} />
            </div>
            <div>
              <h2 className="text-2xl font-black text-slate-900 dark:text-white">祝日設定マネージャー</h2>
              <p className="text-xs text-slate-400 dark:text-slate-500">
                祝日の追加や削除、および内蔵設定との比較が可能です
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors text-slate-500 hover:text-slate-700 dark:hover:text-slate-300"
          >
            <X size={20} />
          </button>
        </div>

        {/* メインコンテンツ */}
        {loading ? (
          <div className="flex-1 flex flex-col items-center justify-center gap-4">
            <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
            <p className="text-sm text-slate-500">データを分析中...</p>
          </div>
        ) : error ? (
          <div className="flex-1 flex flex-col items-center justify-center gap-3 p-8 text-center">
            <AlertCircle size={48} className="text-red-500" />
            <h3 className="text-lg font-bold text-slate-800 dark:text-slate-200">エラーが発生しました</h3>
            <p className="text-sm text-slate-500 max-w-md">{error}</p>
          </div>
        ) : (
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
            {/* 左側：統計とヘルプ */}
            <div className="w-full md:w-80 border-r border-slate-100 dark:border-slate-800 p-6 flex flex-col gap-6 bg-slate-50/50 dark:bg-slate-950/20 overflow-y-auto flex-shrink-0">
              <div>
                <h3 className="text-xs font-black text-slate-400 dark:text-slate-500 uppercase tracking-widest mb-4">
                  設定統計情報
                </h3>
                <div className="flex flex-col gap-3">
                  {stats.map(({ word, count }) => (
                    <div
                      key={word}
                      className="p-3 bg-white dark:bg-slate-850 border border-slate-100 dark:border-slate-800/60 rounded-2xl flex items-center justify-between shadow-sm"
                    >
                      <span className="text-sm font-semibold text-slate-600 dark:text-slate-300">
                        「{word}」の定義数
                      </span>
                      <span className="px-2.5 py-1 bg-blue-50 dark:bg-blue-950/40 text-blue-600 dark:text-blue-400 font-mono text-sm font-bold rounded-lg">
                        {count} 件
                      </span>
                    </div>
                  ))}
                </div>
              </div>

              <div className="p-4 bg-blue-500/5 border border-blue-500/10 rounded-2xl flex flex-col gap-2 mt-auto">
                <h4 className="text-xs font-bold text-blue-600 dark:text-blue-400 flex items-center gap-1.5">
                  <CheckCircle2 size={14} /> 共有クレート連携
                </h4>
                <p className="text-[11px] leading-relaxed text-slate-500 dark:text-slate-400">
                  差分検出にはプロジェクト間共有クレートの <code>compute_diff</code> (LCS アルゴリズム) が、統計には <code>count_occurrences</code> が使用されています。
                </p>
                {!isTauri() && (
                  <div className="mt-2 p-2 bg-amber-500/10 border border-amber-500/20 rounded-xl flex items-start gap-1.5">
                    <AlertCircle size={14} className="text-amber-500 flex-shrink-0 mt-0.5" />
                    <p className="text-[10px] text-amber-700 dark:text-amber-400 leading-tight">
                      Webブラウザ環境のため、Tauriコマンドの代わりにJavaScript代替ロジックで動作しています。
                    </p>
                  </div>
                )}
              </div>
            </div>

            {/* 右側：コンテンツビュー */}
            <div className="flex-1 flex flex-col overflow-hidden">
              {/* タブとアクションボタン */}
              <div className="px-6 py-4 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between flex-shrink-0">
                <div className="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-xl shadow-inner">
                  <button
                    onClick={() => setActiveTab('edit')}
                    className={`px-4 py-1.5 rounded-lg text-xs font-bold transition-all ${
                      activeTab === 'edit'
                        ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400'
                        : 'text-slate-500 hover:text-slate-700'
                    }`}
                  >
                    祝日の編集
                  </button>
                  <button
                    onClick={() => setActiveTab('diff')}
                    className={`px-4 py-1.5 rounded-lg text-xs font-bold transition-all ${
                      activeTab === 'diff'
                        ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400'
                        : 'text-slate-500 hover:text-slate-700'
                    }`}
                  >
                    変更差分 (Diff)
                  </button>
                  <button
                    onClick={() => setActiveTab('raw')}
                    className={`px-4 py-1.5 rounded-lg text-xs font-bold transition-all ${
                      activeTab === 'raw'
                        ? 'bg-white dark:bg-slate-700 shadow-sm text-blue-600 dark:text-blue-400'
                        : 'text-slate-500 hover:text-slate-700'
                    }`}
                  >
                    設定生データ
                  </button>
                </div>

                {/* 保存ボタン */}
                <button
                  onClick={handleSave}
                  disabled={saving}
                  className="px-4 py-1.5 bg-blue-500 hover:bg-blue-600 text-white text-xs font-bold rounded-xl shadow-md shadow-blue-500/20 flex items-center gap-2 transition-all disabled:opacity-50"
                >
                  <Save size={14} />
                  {saving ? '保存中...' : '保存して適用'}
                </button>
              </div>

              {/* ビューエリア */}
              <div className="flex-1 overflow-auto p-6 bg-slate-50 dark:bg-slate-950/40">
                {activeTab === 'edit' ? (
                  <div className="flex flex-col md:flex-row gap-6 h-full min-h-[300px]">
                    {/* 左半分：固定祝日リスト */}
                    <div className="flex-1 flex flex-col border border-slate-200 dark:border-slate-800 rounded-2xl overflow-hidden bg-white dark:bg-slate-900">
                      <div className="px-4 py-2.5 bg-slate-50 dark:bg-slate-850 border-b border-slate-200 dark:border-slate-800 text-xs font-bold text-slate-500">
                        現在の固定祝日一覧 ({holidaysConfig?.fixed ? Object.keys(holidaysConfig.fixed).length : 0}件)
                      </div>
                      <div className="flex-1 overflow-y-auto divide-y divide-slate-100 dark:divide-slate-800 max-h-[220px]">
                        {holidaysConfig?.fixed && Object.entries(holidaysConfig.fixed).map(([date, name]) => (
                          <div key={date} className="px-4 py-2 flex items-center justify-between hover:bg-slate-50/50 dark:hover:bg-slate-850/50 transition-colors">
                            <div className="flex items-center gap-3">
                              <span className="font-mono text-xs font-bold px-2 py-0.5 bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 rounded-md">
                                {date}
                              </span>
                              <span className="text-xs font-semibold text-slate-700 dark:text-slate-300">
                                {name}
                              </span>
                            </div>
                            <button
                              onClick={() => handleRemoveFixedHoliday(date)}
                              className="p-1.5 text-slate-400 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all"
                              title="削除"
                            >
                              <Trash2 size={13} />
                            </button>
                          </div>
                        ))}
                      </div>
                    </div>

                    {/* 右半分：追加フォーム */}
                    <div className="w-full md:w-64 flex flex-col gap-4 border border-slate-200 dark:border-slate-800 rounded-2xl p-4 bg-white dark:bg-slate-900 justify-center">
                      <h4 className="text-xs font-bold text-slate-500 flex items-center gap-1.5">
                        <Plus size={14} className="text-blue-500" />
                        新しい祝日の追加
                      </h4>
                      <form onSubmit={handleAddFixedHoliday} className="flex flex-col gap-3">
                        <div className="flex flex-col gap-1">
                          <label className="text-[10px] font-bold text-slate-400 dark:text-slate-500">日付 (MM-DD)</label>
                          <input
                            type="text"
                            placeholder="例: 07-07"
                            value={newFixedDate}
                            onChange={(e) => setNewFixedDate(e.target.value)}
                            className="px-3 py-1.5 border border-slate-200 dark:border-slate-800 rounded-xl bg-transparent text-xs font-mono text-slate-700 dark:text-slate-200 focus:outline-none focus:border-blue-500 transition-colors"
                          />
                        </div>
                        <div className="flex flex-col gap-1">
                          <label className="text-[10px] font-bold text-slate-400 dark:text-slate-500">祝日の名前</label>
                          <input
                            type="text"
                            placeholder="例: 七夕"
                            value={newFixedName}
                            onChange={(e) => setNewFixedName(e.target.value)}
                            className="px-3 py-1.5 border border-slate-200 dark:border-slate-800 rounded-xl bg-transparent text-xs text-slate-700 dark:text-slate-200 focus:outline-none focus:border-blue-500 transition-colors"
                          />
                        </div>
                        <button
                          type="submit"
                          className="w-full py-2 bg-blue-500 hover:bg-blue-600 text-white text-xs font-bold rounded-xl shadow-md shadow-blue-500/10 flex items-center justify-center gap-1 transition-all mt-2"
                        >
                          <Plus size={12} />
                          リストに追加
                        </button>
                      </form>
                    </div>
                  </div>
                ) : activeTab === 'diff' ? (
                  <div className="flex flex-col min-w-max font-mono text-[11px] leading-relaxed">
                    {diffParts.length === 0 ? (
                      <div className="text-center py-12 text-slate-400">
                        差分はありません。内蔵設定と完全に一致しています。
                      </div>
                    ) : (
                      diffParts.map((part, index) => {
                        const isAdded = part.diff_type === 'Added';
                        const isRemoved = part.diff_type === 'Removed';
                        const bgClass = isAdded
                          ? 'bg-green-500/10 text-green-700 dark:text-green-400 border-l-2 border-green-500'
                          : isRemoved
                          ? 'bg-red-500/10 text-red-700 dark:text-red-400 border-l-2 border-red-500'
                          : 'text-slate-500 dark:text-slate-400/80';
                        const prefix = isAdded ? '+' : isRemoved ? '-' : ' ';

                        return (
                          <div
                            key={index}
                            className={`px-3 py-0.5 whitespace-pre flex gap-2 rounded-sm ${bgClass}`}
                          >
                            <span className="opacity-50 select-none w-3 text-center">{prefix}</span>
                            <span>{part.value}</span>
                          </div>
                        );
                      })
                    )}
                  </div>
                ) : (
                  <pre className="text-slate-600 dark:text-slate-300 font-mono text-[11px]">
                    {rawJsonText}
                  </pre>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </motion.div>
  );
}

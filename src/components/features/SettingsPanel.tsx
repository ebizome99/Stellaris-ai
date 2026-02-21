'use client';

import { useState } from 'react';

interface Settings {
  lowVramMode: boolean;
  fp16: boolean;
  cpuOffload: boolean;
  language: string;
  theme: string;
  notifications: boolean;
}

export function SettingsPanel() {
  const [settings, setSettings] = useState<Settings>({
    lowVramMode: false,
    fp16: true,
    cpuOffload: false,
    language: 'zh-CN',
    theme: 'system',
    notifications: true,
  });

  const handleToggle = (key: keyof Settings) => {
    setSettings((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">设置</h2>

      <div className="card p-6">
        <h3 className="text-md font-semibold text-gray-900 dark:text-gray-100 mb-4">GPU 设置</h3>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">低显存模式</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                适用于 8GB 以下显存的显卡，自动降低内存使用
              </p>
            </div>
            <button
              onClick={() => handleToggle('lowVramMode')}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                settings.lowVramMode ? 'bg-primary-600' : 'bg-gray-300 dark:bg-gray-600'
              }`}
            >
              <span
                className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${
                  settings.lowVramMode ? 'left-7' : 'left-1'
                }`}
              />
            </button>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">FP16 半精度</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                使用半精度浮点数，减少显存占用
              </p>
            </div>
            <button
              onClick={() => handleToggle('fp16')}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                settings.fp16 ? 'bg-primary-600' : 'bg-gray-300 dark:bg-gray-600'
              }`}
            >
              <span
                className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${
                  settings.fp16 ? 'left-7' : 'left-1'
                }`}
              />
            </button>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">CPU Offload</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                将部分模型数据交换到系统内存
              </p>
            </div>
            <button
              onClick={() => handleToggle('cpuOffload')}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                settings.cpuOffload ? 'bg-primary-600' : 'bg-gray-300 dark:bg-gray-600'
              }`}
            >
              <span
                className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${
                  settings.cpuOffload ? 'left-7' : 'left-1'
                }`}
              />
            </button>
          </div>
        </div>
      </div>

      <div className="card p-6">
        <h3 className="text-md font-semibold text-gray-900 dark:text-gray-100 mb-4">界面设置</h3>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">语言</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">界面显示语言</p>
            </div>
            <select
              value={settings.language}
              onChange={(e) => setSettings((s) => ({ ...s, language: e.target.value }))}
              className="input-field w-32"
            >
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
            </select>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">主题</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">界面主题</p>
            </div>
            <select
              value={settings.theme}
              onChange={(e) => setSettings((s) => ({ ...s, theme: e.target.value }))}
              className="input-field w-32"
            >
              <option value="system">跟随系统</option>
              <option value="light">浅色</option>
              <option value="dark">深色</option>
            </select>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium text-gray-900 dark:text-gray-100">通知</p>
              <p className="text-sm text-gray-500 dark:text-gray-400">任务完成时发送通知</p>
            </div>
            <button
              onClick={() => handleToggle('notifications')}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                settings.notifications ? 'bg-primary-600' : 'bg-gray-300 dark:bg-gray-600'
              }`}
            >
              <span
                className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${
                  settings.notifications ? 'left-7' : 'left-1'
                }`}
              />
            </button>
          </div>
        </div>
      </div>

      <div className="card p-6">
        <h3 className="text-md font-semibold text-gray-900 dark:text-gray-100 mb-4">云端 API</h3>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              OpenAI API Key
            </label>
            <input
              type="password"
              placeholder="sk-..."
              className="input-field"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Stability AI API Key
            </label>
            <input
              type="password"
              placeholder="sk-..."
              className="input-field"
            />
          </div>
          <button className="btn-primary">保存 API Keys</button>
        </div>
      </div>
    </div>
  );
}

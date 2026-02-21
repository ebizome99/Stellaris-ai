'use client';

import { useState, useEffect } from 'react';
import { WeatherMoon20Regular, WeatherSunny20Regular } from '@fluentui/react-icons';

export function Header() {
  const [isDark, setIsDark] = useState(false);

  useEffect(() => {
    const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
    setIsDark(darkModeQuery.matches);

    const handler = (e: MediaQueryListEvent) => setIsDark(e.matches);
    darkModeQuery.addEventListener('change', handler);
    return () => darkModeQuery.removeEventListener('change', handler);
  }, []);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', isDark);
  }, [isDark]);

  const toggleTheme = () => {
    setIsDark(!isDark);
  };

  return (
    <header className="h-14 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between px-4">
      <div className="flex items-center gap-2">
        <span className="text-sm text-gray-600 dark:text-gray-400">状态:</span>
        <span className="flex items-center gap-1">
          <span className="w-2 h-2 rounded-full bg-green-500" />
          <span className="text-sm text-green-600 dark:text-green-400">本地引擎就绪</span>
        </span>
      </div>

      <div className="flex items-center gap-4">
        <button
          onClick={toggleTheme}
          className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          aria-label="切换主题"
        >
          {isDark ? (
            <WeatherSunny20Regular className="w-5 h-5 text-yellow-500" />
          ) : (
            <WeatherMoon20Regular className="w-5 h-5 text-gray-600" />
          )}
        </button>
      </div>
    </header>
  );
}

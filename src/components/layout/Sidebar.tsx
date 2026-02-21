'use client';

import {
  ChatSparkle20Regular,
  Image20Regular,
  Grid20Regular,
  Settings20Regular,
} from '@fluentui/react-icons';
import clsx from 'clsx';

interface SidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

const navItems = [
  { id: 'generate', label: '生成', icon: ChatSparkle20Regular },
  { id: 'gallery', label: '图库', icon: Image20Regular },
  { id: 'gpu', label: 'GPU', icon: Grid20Regular },
  { id: 'settings', label: '设置', icon: Settings20Regular },
];

export function Sidebar({ activeTab, onTabChange }: SidebarProps) {
  return (
    <aside className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
      <div className="p-4 border-b border-gray-200 dark:border-gray-700">
        <h1 className="text-xl font-bold text-primary-600">Stellaris AI</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400">企业级AI图片生成系统</p>
      </div>
      
      <nav className="flex-1 p-2">
        <ul className="space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            return (
              <li key={item.id}>
                <button
                  onClick={() => onTabChange(item.id)}
                  className={clsx(
                    'w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors',
                    activeTab === item.id
                      ? 'bg-primary-100 text-primary-700 dark:bg-primary-900/30 dark:text-primary-300'
                      : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                  )}
                >
                  <Icon className="w-5 h-5" />
                  {item.label}
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      <div className="p-4 border-t border-gray-200 dark:border-gray-700">
        <div className="text-xs text-gray-500 dark:text-gray-400">
          <p>版本 0.1.0</p>
          <p className="mt-1">© 2024 Stellaris AI Team</p>
        </div>
      </div>
    </aside>
  );
}

'use client';

import { useState } from 'react';
import { Image20Regular, Delete20Regular, Download20Regular } from '@fluentui/react-icons';

interface ImageItem {
  id: string;
  prompt: string;
  createdAt: Date;
  thumbnail: string;
}

const mockImages: ImageItem[] = Array.from({ length: 12 }, (_, i) => ({
  id: `img-${i}`,
  prompt: `示例图像 ${i + 1} - 一个美丽的风景画`,
  createdAt: new Date(Date.now() - i * 3600000),
  thumbnail: '',
}));

export function GalleryPanel() {
  const [selectedImage, setSelectedImage] = useState<ImageItem | null>(null);
  const [images] = useState<ImageItem[]>(mockImages);

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">图库</h2>
        <span className="text-sm text-gray-500 dark:text-gray-400">{images.length} 张图片</span>
      </div>

      <div className="flex-1 overflow-auto">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {images.map((image) => (
            <div
              key={image.id}
              onClick={() => setSelectedImage(image)}
              className="card aspect-square cursor-pointer overflow-hidden group"
            >
              <div className="w-full h-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center group-hover:bg-gray-200 dark:group-hover:bg-gray-600 transition-colors">
                <Image20Regular className="w-12 h-12 text-gray-400" />
              </div>
            </div>
          ))}
        </div>
      </div>

      {selectedImage && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="card max-w-4xl w-full max-h-[90vh] overflow-auto">
            <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
              <h3 className="font-semibold text-gray-900 dark:text-gray-100">图片详情</h3>
              <button
                onClick={() => setSelectedImage(null)}
                className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
              >
                ✕
              </button>
            </div>
            <div className="p-4">
              <div className="aspect-video bg-gray-100 dark:bg-gray-700 rounded-lg flex items-center justify-center mb-4">
                <Image20Regular className="w-24 h-24 text-gray-400" />
              </div>
              <div className="space-y-2">
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  <strong>提示词:</strong> {selectedImage.prompt}
                </p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  <strong>创建时间:</strong> {selectedImage.createdAt.toLocaleString('zh-CN')}
                </p>
              </div>
              <div className="flex gap-2 mt-4">
                <button className="btn-primary flex items-center gap-2">
                  <Download20Regular className="w-4 h-4" />
                  下载
                </button>
                <button className="btn-secondary flex items-center gap-2 text-red-600 hover:text-red-700">
                  <Delete20Regular className="w-4 h-4" />
                  删除
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

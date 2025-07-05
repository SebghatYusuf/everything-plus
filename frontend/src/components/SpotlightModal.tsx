import React, { useState, useEffect, useRef } from 'react';
import { useSearch } from '../hooks/useSearch';
import { TauriAPI } from '../lib/tauri';
import { appWindow } from '@tauri-apps/api/window';

const SpotlightModal: React.FC = () => {
  const { query, setQuery, results, isLoading } = useSearch();
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.min(prev + 1, results.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (results.length > 0) {
          const selectedItem = results[selectedIndex];
          if (e.ctrlKey || e.metaKey) {
            TauriAPI.openFileLocation(selectedItem.path);
          } else {
            TauriAPI.openFile(selectedItem.path);
          }
          appWindow.hide();
        }
      } else if (e.key === 'Escape') {
        appWindow.hide();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [results, selectedIndex]);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  return (
    <div className="bg-gray-800 bg-opacity-50 backdrop-blur-md rounded-lg shadow-2xl w-full h-full flex flex-col">
      <div className="p-4">
        <input
          ref={inputRef}
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search for files..."
          className="w-full bg-transparent text-white text-2xl placeholder-gray-400 focus:outline-none"
        />
      </div>
      <div className="border-t border-gray-700 flex-grow overflow-y-auto">
        {isLoading && <p className="text-gray-400 p-4">Loading...</p>}
        {!isLoading && (
          <ul>
            {results.map((item, index) => (
              <li
                key={item.path}
                className={`p-4 flex items-center cursor-pointer ${
                  selectedIndex === index ? 'bg-blue-600' : 'hover:bg-gray-700'
                }`}
                onClick={() => TauriAPI.openFile(item.path)}
                onDoubleClick={() => TauriAPI.openFileLocation(item.path)}
              >
                <div className="ml-4">
                  <p className="text-white font-semibold">{item.name}</p>
                  <p className="text-gray-400 text-sm">{item.path}</p>
                </div>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default SpotlightModal;
